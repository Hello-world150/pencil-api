#[macro_use]
extern crate rocket;

use pencil_api::{
    AppState, HitokotoItem, NewUserRequest, RequestedHitokotoItem, User, add_item, add_user,
    get_random_item, load_data, save_item,
};
use rocket::serde::{Serialize, json::Json};
use rocket::{State, http::Status, response::status};

// 错误应答
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// 成功Hitokoto应答
#[derive(Serialize)]
struct HitokotoSuccessResponse {
    message: String,
    item: HitokotoItem,
}

// 成功用户注册应答
#[derive(Serialize)]
struct UserSuccessResponse {
    message: String,
    item: User,
}
#[get("/get")]
async fn get_item(
    state: &State<AppState>,
) -> Result<Json<HitokotoItem>, status::Custom<Json<ErrorResponse>>> {
    match get_random_item(state).await {
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
async fn submit_item(
    state: &State<AppState>,
    new_item: Json<RequestedHitokotoItem>,
) -> Result<status::Custom<Json<HitokotoSuccessResponse>>, status::Custom<Json<ErrorResponse>>> {
    match add_item(state, new_item.into_inner()).await {
        Ok(item) => {
            // 保存到文件
            if let Err(e) = save_item(state).await {
                eprintln!("保存数据到文件失败: {e}");
            }

            let response = HitokotoSuccessResponse {
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

#[post("/register", data = "<user_request>")]
async fn register_user(
    state: &State<AppState>,
    user_request: Json<NewUserRequest>,
) -> Result<status::Custom<Json<UserSuccessResponse>>, status::Custom<Json<ErrorResponse>>> {
    match User::new(user_request.username.clone()) {
        Ok(user) => match add_user(state, user).await {
            Ok(registered_user) => {
                let response = UserSuccessResponse {
                    message: "用户注册成功".to_string(),
                    item: registered_user,
                };
                Ok(status::Custom(Status::Created, Json(response)))
            }
            Err(e) => {
                let response = ErrorResponse {
                    error: format!("注册失败: {e}"),
                };
                Err(status::Custom(Status::BadRequest, Json(response)))
            }
        },
        Err(e) => {
            let response = ErrorResponse {
                error: format!("注册失败: {e}"),
            };
            Err(status::Custom(Status::BadRequest, Json(response))) // 返回400 Bad Request 状态码
        }
    }
}

#[launch]
fn rocket() -> _ {
    // 创建 Tokio 运行时来处理异步初始化
    let rt = tokio::runtime::Runtime::new().expect("创建 Tokio 运行时失败");

    // 启动时加载数据到内存
    let app_state = rt.block_on(async {
        match load_data().await {
            Ok(state) => state,
            Err(e) => panic!("加载数据失败: {e}"),
        }
    });

    rocket::build()
        .manage(app_state)
        .mount("/", routes![get_item, submit_item, register_user])
}
