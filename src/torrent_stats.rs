use std::collections::HashMap;

use crate::transmission::TorrentInfo;
#[derive(Debug)]
pub struct TorrentGroupStats {
    pub num_total: u64,
    pub num_downloading: u64,
    pub num_queue_down: u64,
    pub num_queue_up: u64,
    pub num_seeding: u64,
    pub num_checking: u64,
    pub num_stopped: u64,
    pub num_queue_checking: u64,
    pub num_error: u64,
    pub folders: HashMap<String, u64>,
}
impl TorrentGroupStats {
    pub fn empty() -> Self {
        TorrentGroupStats {
            num_total: 0,
            num_downloading: 0,
            num_queue_down: 0,
            num_queue_up: 0,
            num_checking: 0,
            num_queue_checking: 0,
            num_stopped: 0,
            num_seeding: 0,
            num_error: 0,
            folders: HashMap::new(),
        }
    }
}

pub const STOPPED: i64 = 0;
pub const VERIFY_QUEUED: i64 = 1;
pub const VERIFYING: i64 = 2;
pub const DOWN_QUEUED: i64 = 3;
pub const DOWNLOADING: i64 = 4;
pub const SEED_QUEUED: i64 = 5;
pub const SEEDING: i64 = 6;

pub fn update_torrent_stats(torrents: &HashMap<i64, TorrentInfo>) -> TorrentGroupStats {
    let mut group_stats = TorrentGroupStats::empty();
    for x in torrents.values() {
        let status = x.status;
        let error = x.error;
        if error != 0 {
            group_stats.num_error += 1;
        }
        let folder = x.download_dir.clone();
        *group_stats.folders.entry(folder).or_insert(1) += 1;
        group_stats.num_total += 1;
        match status {
            STOPPED => group_stats.num_stopped += 1,
            VERIFY_QUEUED => group_stats.num_queue_checking += 1,
            VERIFYING => group_stats.num_checking += 1,
            DOWN_QUEUED => group_stats.num_queue_down += 1,
            DOWNLOADING => group_stats.num_downloading += 1,
            SEED_QUEUED => group_stats.num_queue_up += 1,
            SEEDING => group_stats.num_seeding += 1,
            _ => (),
        }
    }


    group_stats
    /*
    i = 0;
    while let Some(x) = category_model.item(i) {
        if x.property_value("is-folder").get::<bool>().expect("sdkfj") == true {
            let download_dir = x.property_value("download-dir").get::<String>().expect("skdfk");
            match group_stats.folders.get(&download_dir) {
                Some(count) => {
                    x.set_property("count", count.to_value());
                    group_stats.folders.remove(&download_dir);
                }
                None => {
                    category_model.remove(i);
                    continue;
                }
            }
        } else {
            let n = match x.property_value("status").get::<i64>().expect("skdfk") {
                ALL => group_stats.num_total,
                STOPPED => group_stats.num_stopped,
                VERIFY_QUEUED => group_stats.num_queue_checking,
                VERIFYING => group_stats.num_checking,
                DOWN_QUEUED => group_stats.num_queue_down,
                DOWNLOADING => group_stats.num_downloading,
                SEED_QUEUED => group_stats.num_queue_up,
                SEEDING => group_stats.num_seeding,
                ERROR => group_stats.num_error,
                _ => 0,
            };
            x.set_property("count", n.to_value());
        }
        i += 1;
    }

    for (key, val) in group_stats.folders.iter() {
        category_model.append(&CategoryObject::new(
            process_folder(key.to_string()),
            *val,
            FOLDER,
            true,
            key.to_string(),
        ));
    }
    */
}
