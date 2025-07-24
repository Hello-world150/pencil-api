#[macro_use]
extern crate rocket;

use pencil_api::{
    AddToCollectionRequest, AppState, Collection, ErrorResponse, 
    HitokotoItem, NewCollectionRequest, NewUserRequest, RequestedHitokotoItem, User, 
    UserWithDetails, add_hitokoto_to_collection, add_item, add_user, create_collection, 
    get_random_item, get_user_with_details, load_data, save_item,
};
use rocket::serde::{Serialize, json::Json};
use rocket::{State, http::Status, response::status};

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
        None => {
            let error_response = ErrorResponse {
                error: "无法获取数据".to_string(),
                code: "NO_DATA".to_string(),
            };
            Err(status::Custom(Status::NotFound, Json(error_response)))
        }
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
            let error_response = e.to_response();
            Err(status::Custom(e.status_code(), Json(error_response)))
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
                let error_response = e.to_response();
                Err(status::Custom(e.status_code(), Json(error_response)))
            }
        },
        Err(e) => {
            let error_response = e.to_response();
            Err(status::Custom(e.status_code(), Json(error_response)))
        }
    }
}

#[get("/user/<user_id>")]
async fn get_user(
    user_id: u32, 
    state: &State<AppState>
) -> Result<Json<UserWithDetails>, status::Custom<Json<ErrorResponse>>> {
    match get_user_with_details(state, user_id).await {
        Some(user_with_details) => Ok(Json(user_with_details)),
        None => {
            let error_response = ErrorResponse {
                error: format!("用户ID {} 不存在", user_id),
                code: "USER_NOT_FOUND".to_string(),
            };
            Err(status::Custom(Status::NotFound, Json(error_response)))
        }
    }
}

#[post("/collection/create", data = "<new_collection>")]
async fn create_collection_endpoint(
    new_collection: Json<NewCollectionRequest>,
    state: &State<AppState>,
) -> Result<Json<Collection>, status::Custom<Json<ErrorResponse>>> {
    let request = new_collection.into_inner();
    match create_collection(state, request.user_id, request.title, request.description).await {
        Ok(collection) => Ok(Json(collection)),
        Err(e) => {
            let error_response = e.to_response();
            Err(status::Custom(e.status_code(), Json(error_response)))
        }
    }
}

#[post("/collection/<collection_id>/add", data = "<add_request>")]
async fn add_to_collection_endpoint(
    collection_id: String,
    add_request: Json<AddToCollectionRequest>,
    state: &State<AppState>,
) -> Result<Json<serde_json::Value>, status::Custom<Json<ErrorResponse>>> {
    let request = add_request.into_inner();
    match add_hitokoto_to_collection(state, collection_id, request.hitokoto_uuid).await {
        Ok(()) => Ok(Json(
            serde_json::json!({"success": true, "message": "添加成功"}),
        )),
        Err(e) => {
            let error_response = e.to_response();
            Err(status::Custom(e.status_code(), Json(error_response)))
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

    rocket::build().manage(app_state).mount(
        "/",
        routes![
            get_item,
            submit_item,
            register_user,
            get_user,
            create_collection_endpoint,
            add_to_collection_endpoint
        ],
    )
}
