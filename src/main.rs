#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[get("/")]
fn index() -> content::RawJson<String> {
    match pencil_api::get_random_item() {
        Some(item) => content::RawJson(serde_json::to_string(&item).unwrap()),
        None => {
            let response = ErrorResponse {
                error: "无法获取数据".to_string(),
            };
            content::RawJson(serde_json::to_string(&response).unwrap())
        }
    }
}

#[launch]
fn rocket() -> _ {
    // 启动时加载数据到内存
    if let Err(e) = pencil_api::load_data() {
        panic!("加载数据失败: {}", e);
    }

    rocket::build().mount("/", routes![index])
}
