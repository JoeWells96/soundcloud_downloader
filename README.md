# Soundcloud Downloader
Small rust cli tool to download mp3s from soundcloud.

Made as an experiment to learn some rust

Inspired by https://github.com/flyingrub/scdl

## Limitations
Currently, only works for soundcloud tracks which have download option enabled.

## Improvements
- Better error handling
- Download tracks which do not have downloading enabled via ffmpeg and hls streams
- Progress bar
- Download all tracks in a playlist / users like list