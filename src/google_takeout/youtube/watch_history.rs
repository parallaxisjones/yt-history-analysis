use crate::google_takeout::youtube::WatchRecord;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WatchHistory {
    items: HashMap<WHKey, WatchRecord>,
}
pub type WHKey = DateTime<Utc>;

impl<'a> IntoIterator for &'a WatchHistory {
    type Item = (&'a WHKey, &'a WatchRecord);
    type IntoIter = hash_map::Iter<'a, WHKey, WatchRecord>;

    fn into_iter(self) -> hash_map::Iter<'a, WHKey, WatchRecord> {
        self.items.iter()
    }
}
impl IntoIterator for WatchHistory {
    type Item = (WHKey, WatchRecord);
    type IntoIter = hash_map::IntoIter<WHKey, WatchRecord>;

    fn into_iter(self) -> hash_map::IntoIter<WHKey, WatchRecord> {
        self.items.into_iter()
    }
}
// and we'll implement FromIterator
impl FromIterator<(WHKey, WatchRecord)> for WatchHistory {
    fn from_iter<I: IntoIterator<Item = (WHKey, WatchRecord)>>(iter: I) -> Self {
        let mut c = WatchHistory::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
impl FromIterator<WatchRecord> for WatchHistory {
    fn from_iter<I: IntoIterator<Item = WatchRecord>>(iter: I) -> Self {
        let mut c = WatchHistory::new();
        for i in iter {
            c.add_record(i);
        }

        c
    }
}

impl WatchHistory {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn count(&self) -> usize {
        self.items.len()
    }
    pub fn from_vec(records: Vec<WatchRecord>) -> Self {
        Self {
            items: records
                .iter()
                .map(|r| (r.get_timestamp(), r.clone()))
                .collect(),
        }
    }

    pub fn to_csv(&self) -> anyhow::Result<()> {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        for video in self.items.values() {
            video.output_csv(&mut wtr)?;
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn add_record(&mut self, record: WatchRecord) {
        self.items.insert(record.get_timestamp(), record);
    }
    pub fn add(&mut self, item: (WHKey, WatchRecord)) {
        self.items.insert(item.0, item.1);
    }
}
