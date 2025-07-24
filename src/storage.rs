use crate::collection::Collection;
use crate::error::{AppError, AppResult};
use crate::item::{HitokotoItem, RequestedHitokotoItem};
use crate::user::{CollectionWithDetails, User, UserWithDetails};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rocket::State;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

// 用于`serde_json::from_reader`解析
pub type Data = Vec<HitokotoItem>;

// Rocket管理的应用状态
pub struct AppState {
    pub data: Mutex<Data>,
    pub users: Mutex<HashMap<u32, User>>, // 用户存储，键为user_id
    pub collections: Mutex<HashMap<String, Collection>>, // 文集存储，键为collection_id
    pub rng: Mutex<StdRng>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(Vec::new()),
            users: Mutex::new(HashMap::new()),
            collections: Mutex::new(HashMap::new()),
            rng: Mutex::new(StdRng::from_entropy()),
        }
    }

    pub async fn load_from_file(&self) -> AppResult<()> {
        let mut file = File::open("sentence.json").await
            .map_err(|e| AppError::Io(format!("无法打开数据文件 sentence.json: {}", e)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await
            .map_err(|e| AppError::Io(format!("无法读取数据文件内容: {}", e)))?;
        let data: Data = serde_json::from_str(&contents)
            .map_err(|e| AppError::Json(format!("数据文件格式错误: {}", e)))?;
        let mut store = self.data.lock().await;
        *store = data;
        Ok(())
    }

    pub async fn load_users_from_file(&self) -> AppResult<()> {
        match File::open("user.json").await {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).await
                    .map_err(|e| AppError::Io(format!("无法读取用户数据文件: {}", e)))?;
                if !contents.trim().is_empty() {
                    let users: Vec<User> = serde_json::from_str(&contents)
                        .map_err(|e| AppError::Json(format!("用户数据文件格式错误: {}", e)))?;
                    let mut user_store = self.users.lock().await;
                    for user in users {
                        user_store.insert(user.user_id, user);
                    }
                }
            }
            Err(_) => {
                // 文件不存在，保持空的HashMap
            }
        }
        Ok(())
    }

    pub async fn save_users_to_file(&self) -> AppResult<()> {
        let users = self.users.lock().await;
        let users_vec: Vec<User> = users.values().cloned().collect();
        let json = serde_json::to_string_pretty(&users_vec)
            .map_err(|e| AppError::Json(format!("序列化用户数据失败: {}", e)))?;
        let mut file = File::create("user.json").await
            .map_err(|e| AppError::Io(format!("创建用户数据文件失败: {}", e)))?;
        file.write_all(json.as_bytes()).await
            .map_err(|e| AppError::Io(format!("写入用户数据失败: {}", e)))?;
        file.flush().await
            .map_err(|e| AppError::Io(format!("刷新用户数据文件失败: {}", e)))?;
        Ok(())
    }

    pub async fn load_collections_from_file(&self) -> AppResult<()> {
        match File::open("collection.json").await {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).await
                    .map_err(|e| AppError::Io(format!("无法读取文集数据文件: {}", e)))?;
                if !contents.trim().is_empty() {
                    let collections: Vec<Collection> = serde_json::from_str(&contents)
                        .map_err(|e| AppError::Json(format!("文集数据文件格式错误: {}", e)))?;
                    let mut collection_store = self.collections.lock().await;
                    for collection in collections {
                        let collection_id = collection.collection_id.clone();
                        collection_store.insert(collection_id, collection);
                    }
                }
            }
            Err(_) => {
                // 文件不存在，保持空的HashMap
            }
        }
        Ok(())
    }

    pub async fn save_collections_to_file(&self) -> AppResult<()> {
        let collections = self.collections.lock().await;
        let collections_vec: Vec<Collection> = collections.values().cloned().collect();
        let json = serde_json::to_string_pretty(&collections_vec)
            .map_err(|e| AppError::Json(format!("序列化文集数据失败: {}", e)))?;
        let mut file = File::create("collection.json").await
            .map_err(|e| AppError::Io(format!("创建文集数据文件失败: {}", e)))?;
        file.write_all(json.as_bytes()).await
            .map_err(|e| AppError::Io(format!("写入文集数据失败: {}", e)))?;
        file.flush().await
            .map_err(|e| AppError::Io(format!("刷新文集数据文件失败: {}", e)))?;
        Ok(())
    }
}

// 加载数据到内存 (用于启动时初始化)
pub async fn load_data() -> AppResult<AppState> {
    let state = AppState::new();
    state.load_from_file().await?;
    state.load_users_from_file().await?; // 也加载用户数据
    state.load_collections_from_file().await?; // 也加载文集数据
    Ok(state)
}

// 辅助函数：验证用户是否存在并返回用户名
async fn get_username_by_id(state: &State<AppState>, user_id: u32) -> AppResult<String> {
    let users = state.users.lock().await;
    let user = users
        .get(&user_id)
        .ok_or_else(|| AppError::NotFound(format!("用户ID {} 不存在，请先注册用户", user_id)))?;
    Ok(user.username.clone())
}

// 辅助函数：检查Hitokoto是否存在
async fn hitokoto_exists(state: &State<AppState>, uuid: &str) -> bool {
    let data = state.data.lock().await;
    data.iter().any(|item| item.uuid == uuid)
}

// 辅助函数：验证用户存在并执行操作
async fn with_user_mut<F, R>(
    state: &State<AppState>,
    user_id: u32,
    operation: F,
) -> AppResult<R>
where
    F: FnOnce(&mut User) -> R,
{
    let mut users = state.users.lock().await;
    let user = users
        .get_mut(&user_id)
        .ok_or_else(|| AppError::NotFound(format!("用户ID {} 不存在，请先注册用户", user_id)))?;
    Ok(operation(user))
}

// 获取随机Hitokoto条目
// 如果没有数据则返回None
pub async fn get_random_item(state: &State<AppState>) -> Option<HitokotoItem> {
    let data = state.data.lock().await;
    let mut rng = state.rng.lock().await;
    data.choose(&mut *rng).cloned()
}

// 添加新Hitokoto条目到数据存储
// 如果数据存储未初始化则返回错误
pub async fn add_item(
    state: &State<AppState>,
    new_item: RequestedHitokotoItem,
) -> AppResult<HitokotoItem> {
    // 首先验证用户是否存在并获取用户名
    let username = get_username_by_id(state, new_item.user_id).await?;

    let mut data = state.data.lock().await;

    // 创建完整的item
    let full_item = HitokotoItem::new(
        new_item.hitokoto,
        new_item.item_type,
        new_item.from,
        new_item.from_who,
        username,
        new_item.user_id,
    );

    // 复制一份用于返回
    let result = full_item.clone();
    // 添加到数据中（移动所有权）
    data.push(full_item);

    Ok(result)
}

// 保存数据到文件
pub async fn save_item(state: &State<AppState>) -> AppResult<()> {
    let data = state.data.lock().await;
    let json = serde_json::to_string_pretty(&*data)
        .map_err(|e| AppError::Json(format!("序列化数据失败: {}", e)))?;
    let mut file = File::create("sentence.json").await
        .map_err(|e| AppError::Io(format!("创建数据文件失败: {}", e)))?;
    file.write_all(json.as_bytes()).await
        .map_err(|e| AppError::Io(format!("写入数据失败: {}", e)))?;
    file.flush().await
        .map_err(|e| AppError::Io(format!("刷新数据文件失败: {}", e)))?;
    Ok(())
}

// 添加用户到状态
pub async fn add_user(
    state: &State<AppState>,
    user: User,
) -> AppResult<User> {
    let mut users = state.users.lock().await;

    // 检查用户名是否已存在
    for existing_user in users.values() {
        if existing_user.username == user.username {
            return Err(AppError::AlreadyExists("用户名已存在".to_string()));
        }
    }

    users.insert(user.user_id, user.clone());

    // 保存到文件
    drop(users); // 释放锁
    if let Err(e) = state.save_users_to_file().await {
        eprintln!("保存用户数据到文件失败: {e}");
    }

    Ok(user)
}

// 根据用户ID获取用户
pub async fn get_user_by_id(state: &State<AppState>, user_id: u32) -> Option<User> {
    let users = state.users.lock().await;
    users.get(&user_id).cloned()
}

// 辅助函数：根据UUID列表获取Hitokoto项目
async fn get_hitokoto_items_by_uuids(
    state: &State<AppState>,
    uuids: &[String],
) -> Vec<HitokotoItem> {
    let data = state.data.lock().await;
    data.iter()
        .filter(|item| uuids.contains(&item.uuid))
        .cloned()
        .collect()
}

// 辅助函数：根据文集ID获取文集详情
async fn get_collection_with_details(
    state: &State<AppState>,
    collection_id: &str,
) -> Option<CollectionWithDetails> {
    let collections = state.collections.lock().await;
    if let Some(collection) = collections.get(collection_id) {
        // 获取文集中的Hitokoto内容
        let collection_items = get_hitokoto_items_by_uuids(state, &collection.hitokoto_ids).await;

        Some(CollectionWithDetails {
            collection_id: collection.collection_id.clone(),
            title: collection.title.clone(),
            description: collection.description.clone(),
            user_id: collection.user_id,
            hitokoto_items: collection_items,
            created_at: collection.created_at,
        })
    } else {
        None
    }
}

// 根据用户ID获取用户完整信息（包含文集和Hitokoto内容）
pub async fn get_user_with_details(
    state: &State<AppState>,
    user_id: u32,
) -> Option<UserWithDetails> {
    let (username, user_item_uuids, user_collection_ids) = {
        let users = state.users.lock().await;
        let user = users.get(&user_id)?;
        (
            user.username.clone(),
            user.items.clone(),
            user.collections.clone(),
        )
    };

    // 获取用户直接提交的Hitokoto
    let user_items = get_hitokoto_items_by_uuids(state, &user_item_uuids).await;

    // 获取用户的文集及其内容
    let mut user_collections = Vec::new();
    for collection_id in &user_collection_ids {
        if let Some(collection_details) = get_collection_with_details(state, collection_id).await {
            user_collections.push(collection_details);
        }
    }

    Some(UserWithDetails {
        user_id,
        username,
        items: user_items,
        collections: user_collections,
    })
}

// 创建新文集
pub async fn create_collection(
    state: &State<AppState>,
    user_id: u32,
    title: String,
    description: Option<String>,
) -> AppResult<Collection> {
    // 创建文集
    let collection = Collection::new(title, description, user_id)
        .map_err(|e| AppError::Collection(e.to_string()))?;
    let collection_id = collection.collection_id.clone();

    // 验证用户是否存在并将文集ID添加到用户的collections列表
    with_user_mut(state, user_id, |user| {
        user.add_collection_id(collection_id.clone())
    })
    .await?;

    // 保存用户数据
    if let Err(e) = state.save_users_to_file().await {
        eprintln!("保存用户数据到文件失败: {e}");
    }

    // 保存文集
    let mut collections = state.collections.lock().await;
    collections.insert(collection_id, collection.clone());
    drop(collections);

    // 保存文集数据到文件
    if let Err(e) = state.save_collections_to_file().await {
        eprintln!("保存文集数据到文件失败: {e}");
    }

    Ok(collection)
}

// 向文集添加Hitokoto
pub async fn add_hitokoto_to_collection(
    state: &State<AppState>,
    collection_id: String,
    hitokoto_uuid: String,
) -> AppResult<()> {
    // 验证Hitokoto是否存在
    if !hitokoto_exists(state, &hitokoto_uuid).await {
        return Err(AppError::NotFound(format!("Hitokoto UUID {} 不存在", hitokoto_uuid)));
    }

    // 添加到文集
    let mut collections = state.collections.lock().await;
    let collection = collections
        .get_mut(&collection_id)
        .ok_or_else(|| AppError::NotFound(format!("文集ID {} 不存在", collection_id)))?;

    collection.add_hitokoto(hitokoto_uuid);
    drop(collections);

    // 保存文集数据到文件
    if let Err(e) = state.save_collections_to_file().await {
        eprintln!("保存文集数据到文件失败: {e}");
    }

    Ok(())
}
