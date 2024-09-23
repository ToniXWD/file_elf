use log::{debug, error};
use rocket::{
    get,
    http::{Method, Status},
    routes,
    serde::json::Json,
    tokio::{signal, spawn},
};

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

use super::api::{api_hot_search, api_regex_search, api_search, api_star_path, api_unstar_path};

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
    Json(api_search(entry, is_fuzzy))
}

#[get("/hot_search?<entry>&<is_fuzzy>&<is_regex>")]
fn hot_search(entry: String, is_fuzzy: bool, is_regex: bool) -> Json<Vec<(String, bool)>> {
    Json(api_hot_search(entry, is_fuzzy, is_regex))
}

#[get("/regex_search?<path>")]
fn regex_search(path: String) -> Json<Vec<(String, bool)>> {
    Json(api_regex_search(path))
}

// TODO: 为了实现简洁, 更改本地状态的请求也使用了get请求, 后续需要修复并解决Option和CORS问题
#[get("/star_path?<path_data>")]
fn star_path(path_data: String) -> Json<bool> {
    Json(api_star_path(path_data))
}

// TODO: 为了实现简洁, 更改本地状态的请求也使用了get请求, 后续需要修复并解决Option和CORS问题
#[get("/unstar_path?<path_data>")]
fn unstar_path(path_data: String) -> Json<bool> {
    Json(api_unstar_path(path_data))
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
