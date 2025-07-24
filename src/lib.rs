// 模块声明
pub mod item;
pub mod storage;
pub mod user;

// 重新导出主要类型和函数
pub use item::{HitokotoItem, RequestedHitokotoItem};
pub use storage::{add_item, get_random_item, load_data, save_item};
pub use user::{NewUserRequest, User};
