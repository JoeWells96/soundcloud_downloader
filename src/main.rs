use std::env;
use std::fs::File;

use reqwest::Error;

use crate::soundcloud::{DownloadTrackResponse, TrackInfo};
use std::io::Write;

mod soundcloud;

fn main() {
    let url: String = env::args().nth(1).expect("please supply an argument");

    let response: Result<soundcloud::TrackInfo, Error> = soundcloud::resolve_track_info(url);

    match response {
        Ok(track_info) => if track_info.downloadable { download_track(track_info) } else { println!("Track is not downloadable") },
        Err(error) => println!("{}", error.to_string())
    }
}

fn download_track(track_info: TrackInfo) {
    let response: DownloadTrackResponse = soundcloud::get_download_link(track_info.id).unwrap();
    let bytes = soundcloud::get_bytes(response.stream_url).unwrap();
    let mut buffer = File::create(format!("{}.mp3", track_info.permalink)).unwrap();
    buffer.write_all(&bytes[..]).unwrap()
}

