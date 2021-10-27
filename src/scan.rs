use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use mp3_metadata::*;
use ogg_metadata::*;

use crate::musicfile::MusicFile;

const SUPPORTED_EXTENSIONS: [&str; 2] = ["mp3", "opus"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        match entry {
            Ok(x) => {
                if is_supported(&x) {
                    match x.path().extension().unwrap().to_str().unwrap() {
                        "mp3" => { match read_from_file(&x.path()) {
                            Ok(a) => music_files.push(MusicFile::new(x.path(), a.optional_info)),
                            Err(e)  => panic!("Error read files"),};
                        }
                        _ => println!("Not implemented")
                    }
                }
            },
            Err(e) => println!("Error reading directory {}", e),
        };
    };
    music_files
}
