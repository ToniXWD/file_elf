use rocket::{get, routes, serde::json::Json};

use crate::cache::CACHER;

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

#[get("/search?<entry>&<is_prefix>")]
fn search(entry: String, is_prefix: bool) -> Json<Vec<String>> {
    println!("search: entry({}), is_prefix({})", entry, is_prefix);
    let guard = CACHER.lock().unwrap(); // 使用 mut 解锁后可以释放锁
    let res = if is_prefix {
        guard.search_entry(&entry)
        // 暂时采用统一接口，后续再进行优化
    } else {
        guard.search_entry(&entry)
    };
    drop(guard); // 显式释放锁

    let res2 = res
        .into_iter()
        .map(|elem| elem.into_os_string().into_string().unwrap())
        .collect();
    Json(res2)
}

pub async fn init_route() {
    let rocket_instance = rocket::build().mount("/file_elf", routes![search]).attach(CORS);

    // 启动 Rocket 服务器并处理错误
    if let Err(e) = rocket_instance.launch().await {
        eprintln!("Failed to launch Rocket: {}", e);
    } else {
        println!("Rocket is running and listening for requests.");
    }
}
