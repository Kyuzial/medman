use std::io::{Write, Read};
use audiotags::AudioTagEdit;
use markdown_gen::markdown::*;
use crate::musicfile::MusicFile;

pub fn write2md(musicfiles: Vec<MusicFile>) {
    let file = File::create("musicfiles.md").unwrap();
    let mut md = Markdown::new(file);
    for music_file in musicfiles {
        md.write(music_file.title().as_str()).unwrap();
    }
}

