use serde::{Deserialize, Serialize};
// Some `use` statements have been omitted here for brevity
use chrono::DateTime;

use crate::google_takeout::youtube::watch_history::WHKey;

pub enum WatchType {
    Advert,
    View,
    Removed,
    Deleted,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subtitles {
    name: String,
    url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WatchRecord {
    header: String,

    title: String,

    title_url: Option<String>,

    pub subtitles: Option<Vec<Subtitles>>,

    time: String,

    products: Vec<String>,

    activity_controls: Vec<String>,
}

impl WatchRecord {
    pub fn get_url(&self) -> String {
        match &self.title_url {
            Some(url) => url.to_string(),
            None => String::from("missing"),
        }
    }
    pub fn get_watch_type(&self) -> WatchType {
        match &self.subtitles {
            Some(_subtitles) => WatchType::View,
            None => {
                let is_deleted = {
                    self.title
                        .contains("Watched https://www.youtube.com/watch?v=")
                };

                let is_missing = { self.title.contains("Watched a video that has been removed") };

                if is_missing {
                    WatchType::Removed
                } else if is_deleted {
                    WatchType::Deleted
                } else {
                    WatchType::Advert
                }
            }
        }
    }
    pub fn get_timestamp(&self) -> WHKey {
        DateTime::parse_from_rfc3339(&self.time).unwrap().into()
    }

    pub fn get_subtitles(&self) -> Subtitles {
        Subtitles {
            name: if self.subtitles.is_none() {
                String::from("missing")
            } else {
                let subs = &self.subtitles.as_ref().unwrap()[0];
                String::from(&subs.name)
            },
            url: if self.subtitles.is_none() {
                Some(String::from("missing"))
            } else {
                let subtitles = &self.subtitles.as_ref().unwrap()[0];
                if let Some(url) = &subtitles.url {
                    Some(String::from(url))
                } else {
                    Some(String::from("missing"))
                }
            },
        }
    }

    pub fn output_csv<T: std::io::Write>(&self, wtr: &mut csv::Writer<T>) -> anyhow::Result<()> {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct VideoRecord {
            time: WHKey,
            title: String,
            url: String,
            channel: String,
            channel_url: String,
        }
        let subtitles = self.get_subtitles();
        match wtr.serialize(VideoRecord {
            time: self.get_timestamp(),
            title: self.title.to_owned(),
            url: self.get_url(),
            channel: subtitles.name.to_owned(),
            channel_url: subtitles.url.unwrap(),
        }) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
