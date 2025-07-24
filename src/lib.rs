// 模块声明
pub mod collection;
pub mod item;
pub mod storage;
pub mod user;

// 重新导出主要类型和函数
pub use collection::{AddToCollectionRequest, Collection, NewCollectionRequest};
pub use item::{HitokotoItem, RequestedHitokotoItem};
pub use storage::{
    AppState, add_hitokoto_to_collection, add_item, add_user, create_collection, get_random_item,
    get_user_by_id, get_user_with_details, load_data, save_item,
};
pub use user::{CollectionWithDetails, NewUserRequest, User, UserWithDetails};
