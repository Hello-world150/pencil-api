#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    message: String,
    item: pencil_api::HitokotoItem,
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

#[post("/submit", data = "<new_item>")]
fn submit_item(new_item: Json<pencil_api::NewHitokotoItem>) -> content::RawJson<String> {
    match pencil_api::add_item(new_item.into_inner()) {
        Ok(item) => {
            // 可选：保存到文件
            if let Err(e) = pencil_api::save_data() {
                eprintln!("保存数据到文件失败: {}", e);
            }

            let response = SuccessResponse {
                message: "提交成功".to_string(),
                item,
            };
            content::RawJson(serde_json::to_string(&response).unwrap())
        }
        Err(e) => {
            let response = ErrorResponse {
                error: format!("提交失败: {}", e),
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

    rocket::build().mount("/", routes![index, submit_item])
}
