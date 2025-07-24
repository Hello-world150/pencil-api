// 模块声明
pub mod item;
pub mod storage;
pub mod user;

// 重新导出主要类型和函数
pub use item::{HitokotoItem, RequestedHitokotoItem};
pub use storage::{
    AppState, add_item, add_user, get_random_item, get_user_by_id, load_data, save_item,
};
pub use user::{NewUserRequest, User};
