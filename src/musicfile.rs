use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicFile {
    path: PathBuf,
    pathstr: String,
    title: String,
    artist: String,
    album: String,
    year: u32,
}

impl PartialEq for MusicFile {
    fn eq(&self, other: &Self) -> bool {
        self.artist == other.artist &&
        self.album == other.album &&
        self.title == other.title &&
        self.year == other.year
    }
}
impl Eq for MusicFile {}

impl MusicFile {
    pub fn new(path: &Path, title : String, artist: String, album: String, year: u32) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            pathstr: path.to_string_lossy().to_string(),
            title,
            artist,
            album,
            year,
        }
    }

    pub fn copy(&self) -> MusicFile {
        MusicFile {
            path: self.path.clone(),
            pathstr: self.pathstr.clone(),
            title: self.title.clone(),
            artist: self.artist.clone(),
            album: self.album.clone(),
            year: self.year,
        }
    }

    pub fn path(&self) -> &str {
        &self.pathstr
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn artist(&self) -> &str {
        self.artist.as_str()
    }
    pub fn album(&self) -> &str {
        self.album.as_str()
    }
    pub fn year(&self) -> &u32 {
        &self.year
    }
}
