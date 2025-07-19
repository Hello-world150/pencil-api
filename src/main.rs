#[macro_use]
extern crate rocket;

use pencil_api::{HitokotoItem, NewHitokotoItem, add_item, get_random_item, load_data, save_data};
use rocket::response::content;
use rocket::serde::{Serialize, json::Json};

// 错误应答
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// 成功应答
#[derive(Serialize)]
struct SuccessResponse {
    message: String,
    item: HitokotoItem,
}

#[get("/get")]
fn index() -> content::RawJson<String> {
    match get_random_item() {
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
fn submit_item(new_item: Json<NewHitokotoItem>) -> content::RawJson<String> {
    match add_item(new_item.into_inner()) {
        Ok(item) => {
            // 保存到文件
            if let Err(e) = save_data() {
                eprintln!("保存数据到文件失败: {e}");
            }

            let response = SuccessResponse {
                message: "提交成功".to_string(),
                item,
            };
            content::RawJson(serde_json::to_string(&response).unwrap())
        }
        Err(e) => {
            let response = ErrorResponse {
                error: format!("提交失败: {e}"),
            };
            content::RawJson(serde_json::to_string(&response).unwrap())
        }
    }
}

#[launch]
fn rocket() -> _ {
    // 启动时加载数据到内存
    if let Err(e) = load_data() {
        panic!("加载数据失败: {e}");
    }

    rocket::build().mount("/", routes![index, submit_item])
}
