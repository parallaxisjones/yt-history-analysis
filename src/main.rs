use anyhow::Context;
use text_analysis::count_words;
use std::{fs::File, path::{Path, PathBuf}};
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use derive_visitor::{Visitor, Drive};
use serde::{Serialize, Deserialize};
// Some `use` statements have been omitted here for brevity

macro_rules! watch_history {() => (
    format!("{}/watch-history.json", env!("CARGO_MANIFEST_DIR")) // assumes Linux ('/')!
  )
}


macro_rules! channel_buckets {() => (
    format!("{}/results.txt", env!("CARGO_MANIFEST_DIR")) // assumes Linux ('/')!
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
    items: Vec<YTVideo>
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ChannelBuckets(HashMap<String, u32>);

impl ChannelBuckets {
    pub fn try_from_path<P: AsRef<Path>>(path: &P) -> anyhow::Result<ChannelBuckets> {
        let mut channels = String::new();
        let mut file = File::open(path).context("file exits")?;
        file.read_to_string(&mut channels).context("expect channel file")?;

        Ok(
            ChannelBuckets(channels.split("/n").map(|c| { (c.into(), 0)}).collect())
        )
    }
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

#[derive(Visitor, Default, Debug, Serialize, Deserialize)]
#[visitor(YTVideo(enter))]
struct StatsCollector {
    has_subtitles: u32,
    videos: u32,
    titles: HashSet<String>,
    channel_buckets: ChannelBuckets
}


//single pass, collect state while operating on each video
impl StatsCollector {
    pub fn new(channel_buckets: ChannelBuckets) -> Self {
        StatsCollector {
            has_subtitles: 0,
            videos: 0,
            titles: HashSet::new(),
            channel_buckets
        }
    }
    fn enter_yt_video(&mut self, video: &YTVideo) {
        self.videos += 1;
        if video.subtitles.is_some() {
            self.has_subtitles += 1;
            let subs = video.subtitles.as_ref().unwrap();
            for sub in subs {
                let name = &sub.name;
                let count = self.channel_buckets.0.get(name).unwrap_or(&0);
                self.channel_buckets.0.insert(name.to_string(), count + 1);
            }
        }
        self.titles.insert(String::from(&video.title));

        if video.title_url.is_some() {
            //println!("{}", &video.title_url.as_ref().unwrap())
        }
    }
}

fn main() -> anyhow::Result<()> {
    let videos: Vec<YTVideo> = YT::get_watch_history().unwrap();
    let channel_buckets = ChannelBuckets::try_from_path(&channel_buckets!()).context("bet")?;
    let mut stats = StatsCollector::new(channel_buckets);

    let history = History {
        items: videos
    };

    history.drive(&mut stats);
    // Serialize it to a JSON string.
    let mut v: _ = Vec::from_iter(&stats.channel_buckets.0);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(a));
    let top_10 = v[0..400].to_vec();
    println!("{:#?}", &top_10);
    //let j = serde_json::to_string(&stats)?;
    // Print, write to a file, or send to an HTTP server.
    //println!("{j}");
    //dbg!(stats);
    //let mut v: Vec<_> = history.get_subtitle_names().into_iter().collect();
    //v.sort();
    //let path = "results.txt";
    //let mut output = File::create(path)?;
    //write!(output, "{}", &v.join("\n"))?;
    Ok(())
}

