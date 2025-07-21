#[macro_use]
extern crate rocket;

use pencil_api::{HitokotoItem, NewHitokotoItem, add_item, get_random_item, load_data, save_data};
use rocket::serde::{Serialize, json::Json};
use rocket::{http::Status, response::status};

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
fn get_item() -> Result<Json<HitokotoItem>, status::Custom<Json<ErrorResponse>>> {
    match get_random_item() {
        Some(item) => Ok(Json(item)),
        None => Err(status::Custom(
            Status::NotFound,
            Json(ErrorResponse {
                error: "无法获取数据".to_string(),
            }),
        )),
    }
}

#[post("/submit", data = "<new_item>")]
fn submit_item(
    new_item: Json<NewHitokotoItem>,
) -> Result<status::Custom<Json<SuccessResponse>>, status::Custom<Json<ErrorResponse>>> {
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
            Ok(status::Custom(Status::Created, Json(response))) // 返回201 Created 状态码
        }
        Err(e) => {
            let response = ErrorResponse {
                error: format!("提交失败: {e}"),
            };
            Err(status::Custom(Status::BadRequest, Json(response))) // 返回400 Bad Request 状态码
        }
    }
}

#[launch]
fn rocket() -> _ {
    // 启动时加载数据到内存
    if let Err(e) = load_data() {
        panic!("加载数据失败: {e}");
    }

    rocket::build().mount("/", routes![get_item, submit_item])
}
