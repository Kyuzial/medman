use crate::markdown::*;
use std::io::{Write, Read};

pub fn write2md(musicfiles: Vec<MusicFile>) {
    let file = File::create("musicfiles.md").unwrap();
    let mut md = Markdown::new(file);
    for music_file in music_files {
        
    }
}

