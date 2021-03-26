use std::fs::File;
use std::io::Write;

use bytes::Bytes;
use m3u8_rs::playlist::MediaPlaylist;

use crate::soundcloud::get_bytes;

pub(crate) fn download_playlist(playlist: MediaPlaylist, file: &File) {
    playlist.segments.iter().for_each(|seg| write_seg_to_file(download_seg(seg.uri.to_string()), file))
}

fn download_seg(url: String) -> Bytes {
    get_bytes(url).unwrap()
}

fn write_seg_to_file(bytes: Bytes, mut file: &File) {
    file.write_all(&bytes[..]).unwrap()
}

