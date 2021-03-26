use std::env;
use std::fs::File;
use std::io::Write;

use bytes::Bytes;
use m3u8_rs::playlist::Playlist;

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
        let m3u8_url = soundcloud::get_m3u8_link(stream_url).map(|r| r.url).unwrap();
        let m3u8_bytes = soundcloud::get_bytes(m3u8_url).unwrap();
        let parsed_m3u8 = m3u8_rs::parse_playlist_res(&m3u8_bytes[..]);
        let file = File::create(format!("{}.mp3", track_info.permalink)).unwrap();
        match parsed_m3u8 {
            Ok(Playlist::MediaPlaylist(pl)) => m3u8::download_playlist(pl, &file),
            _ => panic!()
        }
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

