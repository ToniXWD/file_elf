use rocket::{get, routes, serde::json::Json};

use crate::{cache::CACHER, db::DB};

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
    }
}

#[get("/search?<entry>&<is_fuzzy>")]
fn search(entry: String, is_fuzzy: bool) -> Json<Vec<String>> {
    println!("search: entry({}), is_fuzzy({})", entry, is_fuzzy);
    let guard = CACHER.lock().unwrap(); // 使用 mut 解锁后可以释放锁
    let res = guard.search_entry(&entry, is_fuzzy);

    drop(guard); // 显式释放锁

    if res.is_empty() {
        // 缓存没有查到, 从数据库中尽显查询(数据库查询暂不支持模糊查询)
        println!("cache not found, DB search: entry({})", entry);
        match DB.lock().unwrap().find_by_entry(&entry) {
            Ok(recs) => {
                let res2 = recs
                    .into_iter()
                    .map(|elem| elem.path.to_string_lossy().to_string())
                    .collect();
                Json(res2)
            }
            Err(e) => {
                println!("DB error: {}", e);
                Json(Vec::new())
            }
        }
    } else {
        let res2 = res
            .into_iter()
            .map(|elem| elem.into_os_string().into_string().unwrap())
            .collect();
        Json(res2)
    }
}

#[get("/regex_search?<path>")]
fn regex_search(path: String) -> Json<Vec<String>> {
    println!("regex_search: entry({})", path);

    let guard = CACHER.lock().unwrap(); // 使用 mut 解锁后可以释放锁
    let res = guard.search_path_regex(&path);

    let res2 = res
        .into_iter()
        .map(|elem| elem.into_os_string().into_string().unwrap())
        .collect();
    Json(res2)
}

pub async fn init_route() {
    let rocket_instance = rocket::build()
        .mount("/file_elf", routes![search, regex_search])
        .attach(CORS);

    // 启动 Rocket 服务器并处理错误
    if let Err(e) = rocket_instance.launch().await {
        eprintln!("Failed to launch Rocket: {}", e);
    } else {
        println!("Rocket is running and listening for requests.");
    }
}
