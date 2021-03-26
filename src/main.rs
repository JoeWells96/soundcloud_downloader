use std::env;
use std::fs::File;
use std::io::Write;

use bytes::Bytes;

mod soundcloud;
mod models;
mod m3u8;

fn main() {
    let url = env::args().nth(1).expect("please supply an argument");

    let track_info = soundcloud::resolve_track_info(url).expect("Failed to resolve track info");

    if track_info.downloadable {
        let raw_track = download_original_track(track_info.id).expect("Failed to download raw track");
        save_track_locally(raw_track, track_info.permalink).expect("Failed to save raw track");
    } else {
        let stream_url = track_info.get_stream_url().expect("No original download or mpeg stream available for track");
        download_hls_track(stream_url, track_info.permalink)
    }
}

fn download_original_track(id: u64) -> reqwest::Result<Bytes> {
    let download_response = soundcloud::get_download_link(id)?;
    soundcloud::get_bytes(download_response.redirect_url)
}

fn save_track_locally(bytes: Bytes, name: String) -> std::io::Result<()> {
    let mut buffer = File::create(format!("{}.mp3", name))?;
    buffer.write_all(&bytes[..])
}

fn download_hls_track(url: String, name: String) {
    let hls_url = soundcloud::get_hls_link(url).map(|r| r.url).unwrap();
    let file = File::create(format!("{}.mp3", name)).unwrap();
    m3u8::download_hls_stream_to_file(hls_url, &file);
}

