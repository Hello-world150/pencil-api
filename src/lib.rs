// 模块声明
pub mod collection;
pub mod error;
pub mod hitokoto;
pub mod password;
pub mod storage;
pub mod user;

#[cfg(test)]
mod error_tests;

// 重新导出主要类型和函数
pub use crate::storage::collection::{add_hitokoto_to_collection, create_collection};
pub use crate::storage::hitokoto::{
    add_hitokoto_to_data, get_random_hitokoto, save_hitokoto_to_file,
};
pub use crate::storage::user::{get_user_by_id, get_user_with_details};
pub use collection::{AddToCollectionRequest, Collection, NewCollectionRequest};
pub use error::{AppError, AppResult, ErrorResponse};
pub use hitokoto::{Hitokoto, NewHitokotoRequest};
pub use password::HashedPassword;
pub use storage::{AppState, add_user_to_state, load_data};
pub use user::{CollectionWithDetails, NewUserRequest, User, UserWithDetails};
