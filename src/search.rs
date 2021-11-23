use crate::musicfile::*;

/// Recherche des infos dans les metadonnées
pub fn search(musicfiles: Vec<MusicFile>, pattern: String) -> Vec<MusicFile> {
    let mut result_search: Vec<MusicFile> = Vec::new();

    let mut i =0;
    let v: Vec<&str> = pattern.split('+').collect();
    while i<v.len() {
        for music in &musicfiles {
            let search: Vec<&str> = v[i].split('=').collect();
            match search[0].to_lowercase().as_str() {
                "artist" => {
                    if music.artist().to_lowercase().as_str().contains(search[1].to_lowercase().as_str()) {
                        if !entry_is_in_tab(&result_search, &music.copy()) {
                            result_search.push(music.copy());
                        }
                    }
                },
                "album" => {
                    if music.album().to_lowercase().as_str().contains(search[1].to_lowercase().as_str()) {
                        if !entry_is_in_tab(&result_search, &music.copy()) {
                            result_search.push(music.copy());
                        }
                    }
                },
                "title" => {
                    if music.title().to_lowercase().as_str().contains(search[1].to_lowercase().as_str()) {
                        if !entry_is_in_tab(&result_search, &music.copy()) {
                            result_search.push(music.copy());
                        }
                    }
                },
                "year" => {
                    if music.year() == &search[1].parse::<u32>().unwrap() {
                        if !entry_is_in_tab(&result_search, &music.copy()) {
                            result_search.push(music.copy());
                        }
                    }
                },
                _ => panic!("Only search artist, title, album, or year")
            }
        }
        i = i +1;
    }
    println!("{:#?}", result_search);
    result_search
}

pub fn entry_is_in_tab (tab: &Vec<MusicFile>, musicfile: &MusicFile) -> bool{
    for music in tab {
        if music == musicfile {
            return true;
        }
    }
    return false;
}
