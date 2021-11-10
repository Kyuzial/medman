use medman::cli::CliArguments;
use medman::scan::scan;
use medman::search::search;
use std::io::{Write, Read};

fn main() {
    let args = CliArguments::new();
    println!("{:?}", args);
    match args.command().as_str() {
        "scan" => {
            let music_files = scan(args.path());
            let mut file = std::fs::File::create("data.json").expect("create failed");
            let save = serde_json::to_string_pretty(&music_files).unwrap();
            file.write_all(save.as_bytes()).expect("Write failed");
            for music_file in music_files {
                println!("{:#?}", music_file);
            }
        },
        "restore" => {
            let mut file = std::fs::File::open(args.path()).expect("File doesn't exist!");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            print!("{}", contents);
        },
        "search" => {
            
        },
        "write2md" => {

        },
        _ => {panic!("No args")},
    };

}
