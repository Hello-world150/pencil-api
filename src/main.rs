#[macro_use]
extern crate rocket;

use pencil_api::{
    AddToCollectionRequest, AppError, AppState, Collection, HitokotoItem, NewCollectionRequest,
    NewUserRequest, RequestedHitokotoItem, User, UserWithDetails, add_hitokoto_to_collection,
    add_item_to_data, add_user_to_state, create_collection, get_random_item, get_user_with_details,
    load_data, save_item_to_file,
};
use rocket::serde::{Serialize, json::Json};
use rocket::{State, http::Status, response::status};

// 成功应答
#[derive(Serialize)]
struct ApiResponse<T> {
    message: String,
    data: T,
}

#[get("/get/hitokoto")]
async fn get_item(state: &State<AppState>) -> Result<Json<HitokotoItem>, AppError> {
    get_random_item(state)
        .await
        .map(Json)
        .ok_or_else(|| AppError::NotFound("无法获取数据".to_string()))
}

#[get("/get/user/<user_id>")]
async fn get_user(
    user_id: u32,
    state: &State<AppState>,
) -> Result<Json<UserWithDetails>, AppError> {
    get_user_with_details(state, user_id)
        .await
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!("用户ID {user_id} 不存在")))
}

#[post("/submit/hitokoto", data = "<new_item>")]
async fn submit_item(
    state: &State<AppState>,
    new_item: Json<RequestedHitokotoItem>,
) -> Result<status::Custom<Json<ApiResponse<HitokotoItem>>>, AppError> {
    let item = add_item_to_data(state, new_item.into_inner()).await?;
    // 保存到文件
    if let Err(e) = save_item_to_file(state).await {
        eprintln!("保存数据到文件失败: {e}");
    }

    let response = ApiResponse {
        message: "提交成功".to_string(),
        data: item,
    };
    Ok(status::Custom(Status::Created, Json(response))) // 返回201 Created 状态码
}

#[post("/submit/collection", data = "<new_collection>")]
async fn create_collection_endpoint(
    new_collection: Json<NewCollectionRequest>,
    state: &State<AppState>,
) -> Result<Json<Collection>, AppError> {
    let request = new_collection.into_inner();
    create_collection(state, request.user_id, request.title, request.description)
        .await
        .map(Json)
}

#[post("/submit/collection/<collection_uuid>/add", data = "<add_request>")]
async fn add_to_collection_endpoint(
    collection_uuid: String,
    add_request: Json<AddToCollectionRequest>,
    state: &State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let request = add_request.into_inner();
    add_hitokoto_to_collection(state, collection_uuid, request.hitokoto_uuid).await?;
    Ok(Json(
        serde_json::json!({"success": true, "message": "添加成功"}),
    ))
}

#[post("/register/user", data = "<user_request>")]
async fn register_user(
    state: &State<AppState>,
    user_request: Json<NewUserRequest>,
) -> Result<status::Custom<Json<ApiResponse<User>>>, AppError> {
    let user = User::new(user_request.username.clone())?;
    let registered_user = add_user_to_state(state, user).await?;
    let response = ApiResponse {
        message: "用户注册成功".to_string(),
        data: registered_user,
    };
    Ok(status::Custom(Status::Created, Json(response)))
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
