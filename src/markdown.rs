use markdown_gen::markdown::*;
use crate::musicfile::MusicFile;

pub fn write2md(musicfiles: Vec<MusicFile>) {
    let file = std::fs::File::create("musicfiles.md").unwrap();
    let mut md = Markdown::new(file);
    for music_file in musicfiles {
        println!("{:#?}", music_file);
        md.write(music_file.title().heading(1)).unwrap();
        md.write(format!("Artist: {}", music_file.artist()).as_str()).unwrap();
        md.write(format!("Album: {}", music_file.album()).as_str()).unwrap();
        md.write(format!("Year: {}", music_file.year().to_string().as_str()).as_str()).unwrap();
    }
}

