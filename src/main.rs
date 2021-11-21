use medman::{cli::CliArguments, search};
use medman::scan::scan;
use medman::markdown::write2md;
use medman::mkplaylist::mkplaylist;
use medman::search::search;
use core::panic;
use std::io::{Write, Read};
use medman::musicfile::MusicFile;


fn main() {
    let args = CliArguments::new();
    println!("{:?}", args);
    match args.command().as_str() {
        "S" => {
            let music_files = scan(args.path());
            let mut file = std::fs::File::create("data.json").expect("create failed");
            let save = serde_json::to_string(&music_files).unwrap();
            file.write_all(save.as_bytes()).expect("Write failed");
            for music_file in music_files {
                println!("{:#?}", music_file);
            }
        },
        "R" => {
            let mut file = std::fs::File::open(args.path()).expect("Couldn't open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let mut music_files: Vec<MusicFile> = Vec::new();
            music_files = serde_json::from_str(&contents).expect("Can't deserialize the file");
            for music_file in music_files {
                println!("{:#?}", music_file);
            }
        },
        "W2D" => {
            let music_files = scan(args.path());
            write2md(music_files);
        },
        "W2P" => {
            let music_files = scan(args.path());
            mkplaylist(music_files);
        },
        "Ss" => {
            let music_files = scan(args.path());
            search(music_files, args.search())
        },
        "Rs" => {
            let mut file = std::fs::File::open(args.path()).expect("Couldn't open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let mut music_files: Vec<MusicFile> = Vec::new();
            music_files = serde_json::from_str(&contents).expect("Can't deserialize the file");
            search(music_files, args.search())
        },
        _ => {panic!("No args")},
    };

}
