use crate::item::{HitokotoItem, RequestedHitokotoItem};
use crate::user::User;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rocket::State;
use std::{collections::HashMap, error::Error};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

// 用于`serde_json::from_reader`解析
pub type Data = Vec<HitokotoItem>;

// Rocket管理的应用状态
pub struct AppState {
    pub data: Mutex<Data>,
    pub users: Mutex<HashMap<u32, User>>, // 用户存储，键为user_id
    pub rng: Mutex<StdRng>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(Vec::new()),
            users: Mutex::new(HashMap::new()),
            rng: Mutex::new(StdRng::from_entropy()),
        }
    }

    pub async fn load_from_file(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut file = File::open("sentence.json").await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let data: Data = serde_json::from_str(&contents)?;
        let mut store = self.data.lock().await;
        *store = data;
        Ok(())
    }

    pub async fn load_users_from_file(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        match File::open("user.json").await {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).await?;
                if !contents.trim().is_empty() {
                    let users: Vec<User> = serde_json::from_str(&contents)?;
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

    pub async fn save_users_to_file(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let users = self.users.lock().await;
        let users_vec: Vec<User> = users.values().cloned().collect();
        let json = serde_json::to_string_pretty(&users_vec)?;
        let mut file = File::create("user.json").await?;
        file.write_all(json.as_bytes()).await?;
        file.flush().await?;
        Ok(())
    }
}

// 加载数据到内存 (用于启动时初始化)
pub async fn load_data() -> Result<AppState, Box<dyn Error + Send + Sync>> {
    let state = AppState::new();
    state.load_from_file().await?;
    state.load_users_from_file().await?; // 也加载用户数据
    Ok(state)
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
) -> Result<HitokotoItem, Box<dyn Error + Send + Sync>> {
    let mut data = state.data.lock().await;

    // 创建完整的item
    let full_item = HitokotoItem::new(
        new_item.hitokoto,
        new_item.item_type,
        new_item.from,
        new_item.from_who,
        new_item.user,
    );

    // 添加到数据中
    data.push(full_item.clone());

    Ok(full_item)
}

// 保存数据到文件
pub async fn save_item(state: &State<AppState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = state.data.lock().await;
    let json = serde_json::to_string_pretty(&*data)?;
    let mut file = File::create("sentence.json").await?;
    file.write_all(json.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

// 添加用户到状态
pub async fn add_user(
    state: &State<AppState>,
    user: User,
) -> Result<User, Box<dyn Error + Send + Sync>> {
    let mut users = state.users.lock().await;

    // 检查用户名是否已存在
    for existing_user in users.values() {
        if existing_user.username == user.username {
            return Err("用户名已存在".into());
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
