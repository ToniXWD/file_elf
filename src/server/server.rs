use log::{debug, error};
use rocket::{
    get,
    http::{Method, Status},
    routes,
    serde::json::Json,
    tokio::{signal, spawn},
};
use std::path::PathBuf;

use crate::{
    backend::{
        new_event_handler,
        writer::{DbAction, SENDER},
    },
    cache::{hot_dir::search_files_from_hot_dirs, CACHER},
    db::DB,
    util::is_excluded,
};

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if _request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }
}

#[get("/search?<entry>&<is_fuzzy>")]
fn search(entry: String, is_fuzzy: bool) -> Json<Vec<(String, bool)>> {
    debug!("search: entry({}), is_fuzzy({})", entry, is_fuzzy);
    let guard = CACHER.lock().unwrap(); // 使用 mut 解锁后可以释放锁
    let res = guard.search_entry(&entry, is_fuzzy);

    drop(guard); // 显式释放锁

    if res.is_empty() {
        // 缓存没有查到, 从数据库中尽显查询(数据库查询暂不支持模糊查询)
        debug!("cache not found, DB search: entry({})", entry);
        match DB.lock().unwrap().find_by_entry(&entry) {
            Ok(recs) => {
                let res2 = recs
                    .into_iter()
                    .map(|elem| (elem.path.to_string_lossy().to_string(), true))
                    .collect();
                debug!("search: res2({:?})", res2);
                Json(res2)
            }
            Err(e) => {
                error!("DB error: {}", e);
                Json(Vec::new())
            }
        }
    } else {
        let res2 = res
            .into_iter()
            .map(|elem| (elem.into_os_string().into_string().unwrap(), true))
            .collect();
        debug!("search: res2({:?})", res2);

        Json(res2)
    }
}

#[get("/hot_search?<entry>&<is_fuzzy>&<is_regex>")]
fn hot_search(entry: String, is_fuzzy: bool, is_regex: bool) -> Json<Vec<(String, bool)>> {
    let res = search_files_from_hot_dirs(&entry, is_fuzzy, is_regex);

    let mut cache_guard = CACHER.lock().unwrap();

    let res2 = res
        .into_iter()
        .map(|elem| {
            if cache_guard.contains_path(&PathBuf::from(&elem), false) {
                (elem, true)
            } else {
                (elem, false)
            }
        })
        .collect();

    debug!("hot_search: res2({:?})", res2);

    Json(res2)
}

#[get("/regex_search?<path>")]
fn regex_search(path: String) -> Json<Vec<(String, bool)>> {
    debug!("regex_search: entry({})", path);

    let guard = CACHER.lock().unwrap(); // 使用 mut 解锁后可以释放锁
    let res = guard.search_path_regex(&path);

    let res2 = res
        .into_iter()
        .map(|elem| (elem.into_os_string().into_string().unwrap(), true))
        .collect();
    debug!("regex_search: res2({:?})", res2);
    Json(res2)
}

// TODO: 为了实现简洁, 更改本地状态的请求也使用了get请求, 后续需要修复并解决Option和CORS问题
#[get("/star_path?<path_data>")]
fn star_path(path_data: String) -> Json<bool> {
    let r_path = PathBuf::from(path_data);

    // 先插入缓存
    let mut guard = CACHER.lock().unwrap();
    _ = guard.add_path(&r_path, None, false);
    debug!("star_path: {:#?} insert to cache success", r_path);
    drop(guard);

    // 再插入数据库
    let sender = SENDER.clone();
    new_event_handler(&r_path, &sender);
    debug!("star_path: {:#?} insert to db success", r_path);

    Json(true)
}

// TODO: 为了实现简洁, 更改本地状态的请求也使用了get请求, 后续需要修复并解决Option和CORS问题
#[get("/unstar_path?<path_data>")]
fn unstar_path(path_data: String) -> Json<bool> {
    let r_path = PathBuf::from(path_data);
    if is_excluded(&r_path) {
        return Json(true);
    }

    // 先删除缓存
    let mut guard = CACHER.lock().unwrap();
    _ = guard.remove_path(&r_path);
    drop(guard);

    // 再删除数据库
    let msg = DbAction::DELETE(r_path);

    let sender = SENDER.clone();
    match sender.send(msg) {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn init_route() {
    let figment = rocket::Config::figment().merge(("port", 6789));

    let rocket_instance = rocket::custom(figment)
        .mount(
            "/file_elf",
            routes![search, regex_search, hot_search, unstar_path, star_path],
        )
        .attach(CORS);

    // 启动 Rocket 服务器并处理错误
    let _rocket_handler = spawn(async move {
        if let Err(e) = rocket_instance.launch().await {
            error!("Failed to launch Rocket: {}", e);
        } else {
            debug!("Rocket is running and listening for requests.");
        }
    });

    // 处理信号
    signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl_c signal");
    debug!("Received Ctrl+C, shutting down...");
    // _rocket_handler.abort();
    // 强制退出整个进程
    std::process::exit(1);
}
