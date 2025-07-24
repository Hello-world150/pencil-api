use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// 普通Hitokoto条目
#[derive(Deserialize, Serialize, Clone)]
pub struct HitokotoItem {
    pub uuid: String,
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub from: String,
    pub from_who: Option<String>,
    pub user: String,
    pub user_id: u32,
    pub created_at: String,
    pub length: u32,
}

// 提交的Hitokoto条目（无UUID、uid、时间戳、长度)
#[derive(Deserialize)]
pub struct RequestedHitokotoItem {
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub from: String,
    pub from_who: Option<String>,
    pub user: String,
}

impl HitokotoItem {
    pub fn new(
        hitokoto: String,
        item_type: String,
        from: String,
        from_who: Option<String>,
        user: String,
    ) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let length = hitokoto.chars().count() as u32;

        HitokotoItem {
            uuid,
            hitokoto,
            item_type,
            from,
            from_who,
            user,
            user_id: 0, // 默认值
            created_at,
            length,
        }
    }
}
