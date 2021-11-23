use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use audiotags::*;

use crate::musicfile::MusicFile;

const SUPPORTED_EXTENSIONS: [&str; 3] = ["mp3", "flac", "mp4"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

/// Scan folder to get metadata
/// # Example
/// ```rust
/// fn foo() {
///     let music_files = scan(args.path());
///     //The Vec music files contains all of the data
/// }

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        match entry {
            Ok(x) => {
                if is_supported(&x) {
                    match x.path().extension().unwrap().to_str().unwrap() {
                        "mp3" | "flac" | "mp4" => { 
                            let tags = Tag::default().read_from_path(x.path()).unwrap();
                            music_files.push(MusicFile::new(x.path(), 
                            match tags.title() {
                                Some(a) => String::from(a),
                                None => String::new(),
                            },
                            match tags.artist() {
                                Some(a) => String::from(a),
                                None => String::new(),
                            },
                            match tags.album_title() {
                                Some(a) => String::from(a),
                                None => String::new(),
                            },
                            match tags.year() {
                                Some(b) => b as u32,
                                None => 0,
                            },

                            ))},
                        _ => println!("Only mp3, flac and mp4 supported at the moment")
                    }
                }
            },
            Err(e) => println!("Error reading directory {}", e),
        };
    };
    music_files
}
