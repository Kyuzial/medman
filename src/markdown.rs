use markdown_gen::markdown::*;
use crate::musicfile::MusicFile;

/// Représente les arguments en paramètres de ligne de commande
/// /// /// # Example
/// ```ignore
/// fn foo() {
///     write2md(musicfile);
///     //The Vec music files contains all of the data
/// }
pub fn write2md(musicfiles: Vec<MusicFile>) {
    let file = std::fs::File::create("musicfiles.md").unwrap();
    let mut md = Markdown::new(file);
    for music_file in musicfiles {
        md.write(music_file.title().heading(1)).unwrap();
        md.write(format!("Artist: {}", music_file.artist()).as_str()).unwrap();
        md.write(format!("Album: {}", music_file.album()).as_str()).unwrap();
        md.write(format!("Year: {}", music_file.year()).as_str()).unwrap();
        md.write(format!("Path: {}", music_file.path()).as_str()).unwrap();
    }
}