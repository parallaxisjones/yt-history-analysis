use std::{fs::File, io::Read};

use crate::google_takeout::youtube::WatchRecord;

use super::youtube::WatchHistory;

macro_rules! watch_history {
    () => {
        format!("{}/watch-history.json", env!("CARGO_MANIFEST_DIR")) // assumes Linux ('/')!
    };
}

pub fn get_watch_history() -> anyhow::Result<WatchHistory> {
    match File::open(watch_history!()) {
        Ok(mut file) => {
            let mut raw_data = String::new();
            file.read_to_string(&mut raw_data).unwrap();
            let items: Vec<WatchRecord> = serde_json::from_str(&raw_data).unwrap();
            Ok(WatchHistory::from_iter(items))
        }
        Err(err) => Err(err.into()),
    }
}
