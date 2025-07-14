use rand::seq::SliceRandom;
use std::sync::Mutex;

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
