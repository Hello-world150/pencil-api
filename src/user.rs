use crate::error::AppError;
use crate::item::HitokotoItem;
use serde::{Deserialize, Serialize};
use yit_id_generator::NextId;

// 完整用户信息，包含文集和Hitokoto的完整内容
#[derive(Serialize)]
pub struct UserWithDetails {
    pub user_id: u32,
    pub username: String,
    pub items: Vec<HitokotoItem>, // 用户直接提交的Hitokoto
    pub collections: Vec<CollectionWithDetails>, // 用户的文集及其内容
}

// 文集及其包含的Hitokoto完整内容
#[derive(Serialize)]
pub struct CollectionWithDetails {
    pub collection_id: String,
    pub title: String,
    pub description: Option<String>,
    pub user_id: u32,
    pub hitokoto_items: Vec<HitokotoItem>, // 文集中的Hitokoto完整内容
    pub created_at: u64,
}

// 新用户注册请求
#[derive(Deserialize)]
pub struct NewUserRequest {
    pub username: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub items: Vec<String>,       // 存储 Hitokoto 的 UUID 引用
    pub collections: Vec<String>, // 存储文集的 ID 引用
}

impl User {
    pub fn new(username: String) -> Result<Self, AppError> {
        if username.trim().is_empty() {
            return Err(AppError::User("用户名不能为空".to_string()));
        }

        let user_id = NextId() as u32; // 使用yit_id_generator生成唯一uid
        Ok(User {
            user_id,
            username,
            items: Vec::new(),
            collections: Vec::new(),
        })
    }

    // 添加 Hitokoto UUID 到用户的 items 列表
    pub fn add_hitokoto_uuid(&mut self, uuid: String) {
        self.items.push(uuid);
    }

    // 添加文集 ID 到用户的 collections 列表
    pub fn add_collection_id(&mut self, collection_id: String) {
        if !self.collections.contains(&collection_id) {
            self.collections.push(collection_id);
        }
    }
}
