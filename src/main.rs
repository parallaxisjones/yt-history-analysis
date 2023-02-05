use text_analysis::count_words;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use derive_visitor::{Visitor, Drive};
use serde::{Serialize, Deserialize};
// Some `use` statements have been omitted here for brevity

macro_rules! watch_history {() => (
    format!("{}/watch-history.json", env!("CARGO_MANIFEST_DIR")) // assumes Linux ('/')!
  )
}

#[derive(Serialize, Deserialize, Debug, Drive)]
struct Subtitles {
    #[drive(skip)]
    name: String,

    #[drive(skip)]
    url: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Drive)]
#[serde(rename_all = "camelCase")]
struct YTVideo {
    #[drive(skip)]
    header: String,

    #[drive(skip)]
    title: String,

    #[drive(skip)]
    title_url: Option<String>,

    subtitles: Option<Vec<Subtitles>>,

    #[drive(skip)]
    time: String,

    #[drive(skip)]
    products: Vec<String>,

    #[drive(skip)]
    activity_controls: Vec<String>
}

struct YT;

impl YT {
    pub fn get_watch_history() -> anyhow::Result<Vec<YTVideo>> {
        match File::open(watch_history!()) {
            Ok(mut file) => {
                let mut raw_data = String::new();
                file.read_to_string(&mut raw_data).unwrap();
                Ok(serde_json::from_str(&raw_data).unwrap())
            },
            Err(err) => {
                Err(err.into())
            }
        }
    }
}


#[derive(Drive)]
struct History {
    #[drive(skip)]
    name: String,
    items: Vec<YTVideo>
}


impl History {
    pub fn get_titles(&self) -> Vec<String> {
        //single pass over videos has already happened
        todo!()
    }

    pub fn get_subtitle_names(&self) -> HashSet<String> {
        self.items.iter().fold(HashSet::new(),|mut acc, video| {
            if video.subtitles.is_some() {
                let subtitles = &video.subtitles.as_ref().unwrap();
                subtitles.iter().for_each(|sub| {
                    acc.insert(String::from(&sub.name));
                })
            }
            acc
        })

    }
}

#[derive(Visitor, Default, Debug)]
#[visitor(YTVideo(enter))]
struct StatsCollector {
    has_subtitles: u32,
    videos: u32
}


//single pass, collect state while operating on each video
impl StatsCollector {
    fn enter_yt_video(&mut self, video: &YTVideo) {
        self.videos += 1;
        if video.subtitles.is_some() {
            self.has_subtitles += 1;
        }

        if video.title_url.is_some() {
            //println!("{}", &video.title_url.as_ref().unwrap())
        }
    }
}

// Mutating one map
fn merge(map1: &mut HashMap<String, u32>, map2: HashMap<String, u32>) {
    map1.extend(map2);
}
fn main() -> anyhow::Result<()> {
    let videos: Vec<YTVideo> = YT::get_watch_history().unwrap();
    let mut stats = StatsCollector::default();

    let history = History {
        name: String::from("ythistory"),
        items: videos
    };

    history.drive(&mut stats);

    dbg!(stats);
    let mut v: Vec<_> = history.get_subtitle_names().into_iter().collect();
    v.sort();
    let path = "results.txt";
    let mut output = File::create(path)?;
    write!(output, "{}", &v.join("\n"));
    Ok(())
}
