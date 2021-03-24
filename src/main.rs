use std::env;
use std::fs::File;
use std::io::Write;

use bytes::Bytes;

use soundcloud::{DownloadTrackResponse, TrackInfo};

mod soundcloud;

fn main() {
    let url: String = env::args().nth(1).expect("please supply an argument");

    let track_info: TrackInfo = soundcloud::resolve_track_info(url).expect("Failed to resolve track info");

    if track_info.downloadable {
        let raw_track: Bytes = download_raw_track(&track_info).expect("Failed to download raw track");
        save_track_locally(raw_track, track_info.permalink).expect("Failed to save raw track");
    } else {
        panic!("Track is not downloadable")
    }
}

fn download_raw_track(track_info: &TrackInfo) -> reqwest::Result<Bytes> {
    let download_response: DownloadTrackResponse = soundcloud::get_download_link(track_info.id)?;
    soundcloud::get_bytes(download_response.stream_url)
}

fn save_track_locally(bytes: Bytes, name: String) -> std::io::Result<()> {
    let mut buffer: File = File::create(format!("{}.mp3", name))?;
    buffer.write_all(&bytes[..])
}

