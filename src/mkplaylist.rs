use pls::*;
use crate::musicfile::MusicFile;
use std::io::Write;

///Génère une playlist VLC à partir d'une liste (playlist.pls)
/// /// # Example
/// ```ignore
/// fn foo() {
///     mkplaylist(musicfile);
///     //The Vec music files contains all of the data
/// }
pub fn mkplaylist(musicfiles: Vec<MusicFile>) {
    
    let mut buf = Vec::new();
    let mut playlist:Vec<PlaylistElement> = Vec::new();
    for music in &musicfiles {
        playlist.push(PlaylistElement {
            path: music.path().to_string(),
            title: Some(music.title().to_string()),
            len: ElementLength::Unknown,
        })
    }
    pls::write(&playlist,
    &mut buf).unwrap();

    let mut file = std::fs::File::create("playlist.pls").expect("create failed");
    file.write_all(&buf).expect("Write failed");
}

