use std::path::{Path, PathBuf};
use mp3_metadata::*;

#[derive(Debug)]
pub struct MusicFile {
    path: PathBuf,
    mp3_metadata : Vec<OptionalAudioTags>,
}

impl MusicFile {
    pub fn new(path: &Path, mp3_metadata : Vec<OptionalAudioTags>) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            mp3_metadata,
        }
    }
}
