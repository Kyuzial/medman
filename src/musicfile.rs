use std::path::{Path, PathBuf};
use audiotags::*;

#[derive(Debug)]
pub struct MusicFile {
    path: PathBuf,
    title: String,
    artist: String,
    album: String,
    year: u32,
}

impl MusicFile {
    pub fn new(path: &Path, title : String, artist: String, album: String, year: u32) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            title,
            artist,
            album,
            year,
        }
    }
}
