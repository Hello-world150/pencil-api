use crate::error::AppError;
use pw_hash::bcrypt;
pub struct HashedPassword {
    pub value: String,
}
impl HashedPassword {
    pub fn new(password: String) -> Result<Self, AppError> {
        if password.trim().is_empty() {
            return Err(AppError::User("密码不能为空".to_string()));
        }
        let hashed_password = bcrypt::hash(password).unwrap();
        // TODO: 实现密码哈希逻辑
        println!("密码哈希成功: {}", hashed_password);
        Ok(HashedPassword {
            value: hashed_password,
        })
    }
}
