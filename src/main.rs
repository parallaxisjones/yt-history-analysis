mod google_takeout;

use google_takeout::youtube::WatchHistory;

use crate::google_takeout::youtube::watch_record::WatchType;

fn percentage(count: usize, total: &usize) -> f32 {
    count as f32 * 100.0 / *total as f32
}

fn main() -> anyhow::Result<()> {
    let videos: WatchHistory = google_takeout::export::get_watch_history().unwrap();
    let total = &videos.count();
    println!("total: {total}");
    let ads = videos
        .clone()
        .into_iter()
        .filter(|(_time, video)| matches!(video.get_watch_type(), WatchType::Advert))
        .collect::<WatchHistory>()
        .count();
    let percent_ads = percentage(ads, total);

    let removed = videos
        .clone()
        .into_iter()
        .filter(|(_time, video)| matches!(video.get_watch_type(), WatchType::Removed))
        .collect::<WatchHistory>()
        .count();
    let percent_removed = percentage(removed, total);

    let views = videos
        .clone()
        .into_iter()
        .filter(|(_time, video)| matches!(video.get_watch_type(), WatchType::View))
        .collect::<WatchHistory>()
        .count();
    let percent_views = percentage(views, total);

    let deleted = videos
        .into_iter()
        .filter(|(_time, video)| matches!(video.get_watch_type(), WatchType::Deleted))
        .collect::<WatchHistory>()
        .count();
    let percent_deleted = percentage(deleted, total);

    println!("views: {views} / {percent_views}%");
    println!("removed: {removed} / {percent_removed}%");
    println!("deleted: {deleted} / {percent_deleted}%");
    println!("ads: {ads} / {percent_ads}%");

    //filtered_videos.to_csv()?;
    Ok(())
}
