use crate::item::{HitokotoItem, RequestedHitokotoItem};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::{
    error::Error,
    fs::File,
    sync::{Mutex, OnceLock},
};

// 用于`serde_json::from_reader`解析
pub type Data = Vec<HitokotoItem>;

// 一个全局的Mutex保护的数据存储
pub static DATA_STORE: Mutex<Option<Data>> = Mutex::new(None);

// 用于随机数生成的静态OnceLock
static RNG: OnceLock<Mutex<StdRng>> = OnceLock::new();

// 获取随机数生成器
fn get_rng() -> &'static Mutex<StdRng> {
    RNG.get_or_init(|| Mutex::new(StdRng::from_entropy()))
}

// 加载数据到内存
pub fn load_data() -> Result<(), Box<dyn Error>> {
    let file = File::open("sentence.json")?;
    let data: Data = serde_json::from_reader(file)?;
    let mut store = DATA_STORE.lock().unwrap();
    *store = Some(data);
    Ok(())
}

// 获取随机Hitokoto条目
// 如果没有数据则返回None
pub fn get_random_item() -> Option<HitokotoItem> {
    let store = DATA_STORE.lock().unwrap();
    if let Some(data) = store.as_ref() {
        let mut rng = get_rng().lock().unwrap();
        data.choose(&mut *rng).cloned()
    } else {
        None
    }
}

// 添加新Hitokoto条目到数据存储
// 如果数据存储未初始化则返回错误
pub fn add_item(new_item: RequestedHitokotoItem) -> Result<HitokotoItem, Box<dyn Error>> {
    let mut store = DATA_STORE.lock().unwrap();

    if let Some(data) = store.as_mut() {
        // 创建完整的item
        let full_item = HitokotoItem::new(
            new_item.hitokoto,
            new_item.item_type,
            new_item.from,
            new_item.from_who,
            new_item.creator,
        );

        // 添加到数据中
        data.push(full_item.clone());

        Ok(full_item)
    } else {
        Err("数据存储未初始化".into())
    }
}

// 保存数据到文件
pub fn save_item() -> Result<(), Box<dyn Error>> {
    let store = DATA_STORE.lock().unwrap(); // 取得DATA使用权，并lock
    if let Some(data) = store.as_ref() {
        let json = serde_json::to_string_pretty(data)?;
        std::fs::write("sentence.json", json)?;
        Ok(())
    } else {
        Err("没有数据可保存".into())
    }
}
