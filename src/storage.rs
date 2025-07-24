use crate::item::{HitokotoItem, RequestedHitokotoItem};
use crate::user::User;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rocket::State;
use std::{collections::HashMap, error::Error, fs::File, sync::Mutex};

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

    pub fn load_from_file(&self) -> Result<(), Box<dyn Error>> {
        let file = File::open("sentence.json")?;
        let data: Data = serde_json::from_reader(file)?;
        let mut store = self.data.lock().unwrap();
        *store = data;
        Ok(())
    }

    pub fn load_users_from_file(&self) -> Result<(), Box<dyn Error>> {
        if let Ok(file) = File::open("user.json") {
            // 检查文件是否为空
            let metadata = file.metadata()?;
            if metadata.len() > 0 {
                let users: Vec<User> = serde_json::from_reader(file)?;
                let mut user_store = self.users.lock().unwrap();
                for user in users {
                    user_store.insert(user.user_id, user);
                }
            }
        }
        // 如果文件不存在或为空，就保持空的HashMap
        Ok(())
    }

    pub fn save_users_to_file(&self) -> Result<(), Box<dyn Error>> {
        let users = self.users.lock().unwrap();
        let users_vec: Vec<User> = users.values().cloned().collect();
        let json = serde_json::to_string_pretty(&users_vec)?;
        std::fs::write("user.json", json)?;
        Ok(())
    }
}

// 加载数据到内存 (用于启动时初始化)
pub fn load_data() -> Result<AppState, Box<dyn Error>> {
    let state = AppState::new();
    state.load_from_file()?;
    state.load_users_from_file()?; // 也加载用户数据
    Ok(state)
}

// 获取随机Hitokoto条目
// 如果没有数据则返回None
pub fn get_random_item(state: &State<AppState>) -> Option<HitokotoItem> {
    let data = state.data.lock().unwrap();
    let mut rng = state.rng.lock().unwrap();
    data.choose(&mut *rng).cloned()
}

// 添加新Hitokoto条目到数据存储
// 如果数据存储未初始化则返回错误
pub fn add_item(
    state: &State<AppState>,
    new_item: RequestedHitokotoItem,
) -> Result<HitokotoItem, Box<dyn Error>> {
    let mut data = state.data.lock().unwrap();

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
pub fn save_item(state: &State<AppState>) -> Result<(), Box<dyn Error>> {
    let data = state.data.lock().unwrap();
    let json = serde_json::to_string_pretty(&*data)?;
    std::fs::write("sentence.json", json)?;
    Ok(())
}

// 添加用户到状态
pub fn add_user(state: &State<AppState>, user: User) -> Result<User, Box<dyn Error>> {
    let mut users = state.users.lock().unwrap();

    // 检查用户名是否已存在
    for existing_user in users.values() {
        if existing_user.username == user.username {
            return Err("用户名已存在".into());
        }
    }

    users.insert(user.user_id, user.clone());

    // 保存到文件
    drop(users); // 释放锁
    if let Err(e) = state.save_users_to_file() {
        eprintln!("保存用户数据到文件失败: {e}");
    }

    Ok(user)
}

// 根据用户ID获取用户
pub fn get_user_by_id(state: &State<AppState>, user_id: u32) -> Option<User> {
    let users = state.users.lock().unwrap();
    users.get(&user_id).cloned()
}
