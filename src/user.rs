use crate::item::HitokotoItem;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use yit_id_generator::NextId;

// 自定义错误类型
#[derive(Debug)]
pub struct UserError {
    message: String,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for UserError {}

// 新用户注册请求
#[derive(Deserialize)]
pub struct NewUserRequest {
    pub username: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub items: Vec<HitokotoItem>,
}

impl User {
    pub fn new(username: String) -> Result<Self, UserError> {
        if username.trim().is_empty() {
            return Err(UserError {
                message: "用户名不能为空".to_string(),
            });
        }

        let user_id = NextId() as u32; // 使用yit_id_generator生成唯一uid
        Ok(User {
            user_id,
            username,
            items: Vec::new(),
        })
    }
}
