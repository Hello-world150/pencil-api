use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// 普通Hitokoto条目
#[derive(Deserialize, Serialize, Clone)]
pub struct Hitokoto {
    pub uuid: String,
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub hitokoto_type: String,
    pub from: String,
    pub from_who: Option<String>,
    pub user: String,
    pub user_id: u32,
    pub created_at: u64,
    pub length: u32,
}

// 提交的Hitokoto条目（无UUID、uid、时间戳、长度)
#[derive(Deserialize)]
pub struct NewHitokotoRequest {
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub hitokoto_type: String,
    pub from: String,
    pub from_who: Option<String>,
    pub user_id: u32,
}

impl Hitokoto {
    pub fn new(
        hitokoto: String,
        hitokoto_type: String,
        from: String,
        from_who: Option<String>,
        user: String,
        user_id: u32,
    ) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let length = hitokoto.chars().count() as u32;

        Hitokoto {
            uuid,
            hitokoto,
            hitokoto_type,
            from,
            from_who,
            user,
            user_id,
            created_at,
            length,
        }
    }
}
