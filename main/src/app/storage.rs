use std::collections::HashMap;
use crate::app::class_info::class::ClassSettings;

const STORAGE_KEY: &str = "dex_builds";

fn get_storage() -> web_sys::Storage {
    web_sys::window()
        .expect("No global window found")
        .local_storage()
        .expect("No local storage available")
        .expect("Local storage permission denied")
}

pub fn load_all_builds() -> HashMap<String, ClassSettings> {
    let storage = get_storage();
    if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
        serde_json::from_str(&json).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

pub fn save_all_builds(builds: &HashMap<String, ClassSettings>) {
    let storage = get_storage();
    if let Ok(json) = serde_json::to_string(builds) {
        let _ = storage.set_item(STORAGE_KEY, &json);
    }
}