use rand::seq::SliceRandom;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct HitokotoItem {
    pub uuid: String,
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub from: String,
    pub from_who: Option<String>,
    pub creator: String,
    pub creator_uid: u32,
    pub created_at: String,
    pub length: u32,
}

pub type Data = Vec<HitokotoItem>;

pub static DATA_STORE: Mutex<Option<Data>> = Mutex::new(None);

pub fn load_data() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("sentence.json")?;
    let data: Data = serde_json::from_reader(file)?;
    let mut store = DATA_STORE.lock().unwrap();
    *store = Some(data);
    Ok(())
}

pub fn get_random_item() -> Option<HitokotoItem> {
    let store = DATA_STORE.lock().unwrap();
    if let Some(data) = store.as_ref() {
        let mut rng = rand::thread_rng();
        data.choose(&mut rng).cloned()
    } else {
        None
    }
}

#[derive(serde::Deserialize)]
pub struct NewHitokotoItem {
    pub hitokoto: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub from: String,
    pub from_who: Option<String>,
    pub creator: String,
}

pub fn add_item(new_item: NewHitokotoItem) -> Result<HitokotoItem, Box<dyn std::error::Error>> {
    let mut store = DATA_STORE.lock().unwrap();

    if let Some(data) = store.as_mut() {
        // 生成UUID v4
        let new_uuid = Uuid::new_v4().to_string();

        // 创建完整的item
        let full_item = HitokotoItem {
            uuid: new_uuid,
            hitokoto: new_item.hitokoto.clone(),
            item_type: new_item.item_type,
            from: new_item.from,
            from_who: new_item.from_who,
            creator: new_item.creator,
            creator_uid: 0, // 默认值
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            length: new_item.hitokoto.chars().count() as u32,
        };

        // 添加到数据中
        data.push(full_item.clone());

        Ok(full_item)
    } else {
        Err("数据存储未初始化".into())
    }
}

pub fn save_data() -> Result<(), Box<dyn std::error::Error>> {
    let store = DATA_STORE.lock().unwrap();
    if let Some(data) = store.as_ref() {
        let json = serde_json::to_string_pretty(data)?;
        std::fs::write("sentence.json", json)?;
        Ok(())
    } else {
        Err("没有数据可保存".into())
    }
}
