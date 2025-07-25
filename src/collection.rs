use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// 文集结构体
#[derive(Deserialize, Serialize, Clone)]
pub struct Collection {
    pub collection_uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub user_id: u32,
    pub hitokoto_uuids: Vec<String>, // 存储 Hitokoto 的 UUID 引用
    pub created_at: u64,
}

// 创建新文集的请求
#[derive(Deserialize)]
pub struct NewCollectionRequest {
    pub user_id: u32,
    pub title: String,
    pub description: Option<String>,
}

// 向文集添加 Hitokoto 的请求
#[derive(Deserialize)]
pub struct AddToCollectionRequest {
    pub hitokoto_uuid: String,
}

impl Collection {
    pub fn new(title: String, description: Option<String>, user_id: u32) -> Result<Self, AppError> {
        if title.trim().is_empty() {
            return Err(AppError::Collection("文集标题不能为空".to_string()));
        }

        let collection_uuid = Uuid::new_v4().to_string();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Collection {
            collection_uuid,
            title,
            description,
            user_id,
            hitokoto_uuids: Vec::new(),
            created_at,
        })
    }

    // 添加 Hitokoto UUID 到文集
    pub fn add_hitokoto(&mut self, hitokoto_uuid: String) {
        if !self.hitokoto_uuids.contains(&hitokoto_uuid) {
            self.hitokoto_uuids.push(hitokoto_uuid);
        }
    }

    // 从文集中移除 Hitokoto UUID
    pub fn remove_hitokoto(&mut self, hitokoto_uuid: &str) -> bool {
        if let Some(pos) = self
            .hitokoto_uuids
            .iter()
            .position(|id| id == hitokoto_uuid)
        {
            self.hitokoto_uuids.remove(pos);
            true
        } else {
            false
        }
    }
}
