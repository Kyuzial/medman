use medman::{cli::CliArguments};
use medman::scan::scan;
use medman::markdown::write2md;
use medman::mkplaylist::mkplaylist;
use medman::search::search;
use std::io::{Write, Read};
use medman::musicfile::MusicFile;
use std::path::Path;


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
            let mut music_files = scan(args.path());
            music_files = search(music_files, args.search());
            write2md(music_files);
        },
        "Rs" => {
            let mut file = std::fs::File::open(args.path()).expect("Couldn't open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let mut music_files: Vec<MusicFile> = Vec::new();
            music_files = serde_json::from_str(&contents).expect("Can't deserialize the file");
            music_files = search(music_files, args.search());
            write2md(music_files);
        },
        _ => interractive(),
    };
}

fn interractive () {
    println!("What do you want to do ? You can either search, scan, or read a file");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read input");
    
    if input.trim() == "scan" {
        println!("Which folder do you want to scan ?");
        input.clear();
        std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read path");
        let input_str = input.trim();
        let path = Path::new(&input_str);
        let music_files = scan(path);

        for music_file in &music_files {
            println!("{:#?}", music_file);
        }

        println!("Do you want to save result as markdwon, json or none ?");
        input.clear();
        std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading command");

        match input.trim() {
            "markdown" => write2md(music_files),
            "json" => {
                let mut file = std::fs::File::create("data.json").expect("create failed");
                let save = serde_json::to_string(&music_files).unwrap();
                file.write_all(save.as_bytes()).expect("Write failed");
            },
            _ => println!("Not saving anything"),
        };
    }
    if input.trim() == "read" {
        println!("Which file do you want to read ?");
        input.clear();
        std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read path");
        let input_str = input.trim();
        
        let mut file = std::fs::File::open(input_str).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut music_files: Vec<MusicFile> = Vec::new();
        music_files = serde_json::from_str(&contents).expect("Can't deserialize the file");
        for music_file in music_files {
            println!("{:#?}", music_file);
        }
    }
    if input.trim() == "search"{
        println!("Do you want to read a json file or scan and search a folder ? (scan or read)");
        input.clear();
        std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read your input");
        let input_str = input.trim();

        match input_str {
            "read" => {
                let mut file = std::fs::File::open("data.json").expect("Couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                let mut music_files: Vec<MusicFile> = Vec::new();
                music_files = serde_json::from_str(&contents).expect("Can't deserialize the file");

                println!("Enter the pattern (Ex : artist=Bonjour+Year=1985)");
                input.clear();
                std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read your input");
                let input_str = input.trim().to_string();

                music_files = search(music_files, input_str);
                write2md(music_files);
            },
            "scan" => {
                println!("Which Folder ?");
                input.clear();
                std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read your input");
                let input_str = input.trim();
                let mut music_files = scan(Path::new(input_str));

                println!("Enter the pattern (Ex : artist=Bonjour+Year=1985)");
                input.clear();
                std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read your input");
                let input_str = input.trim().to_string();

                music_files = search(music_files, input_str);
                write2md(music_files);
            }
            _ => panic!("Enter read or scan!")
        }
    }
}


#[cfg(test)]
mod tests {
    use medman::scan;

    use super::*;

    #[test]
    fn scan_non_existing_path(){
        let musicfiles_empty:Vec<MusicFile> = Vec::new();
        let mut musicfiles:Vec<MusicFile> = Vec::new();
        musicfiles = scan(Path::new("/fafa/fa"));
        assert_eq!(musicfiles_empty, musicfiles);        
    }

    #[test]
    fn search_with_empty_data(){
        let musicfiles_empty:Vec<MusicFile> = Vec::new();
        let mut musicfiles:Vec<MusicFile> = Vec::new();
        musicfiles = search(musicfiles, "Artist=X".to_string());

        assert_eq!(musicfiles_empty, musicfiles);        
    }

    #[test]
    fn search_with_empty_pattern(){
        let mut musicfiles:Vec<MusicFile> = Vec::new();
        let musicfiles_empty:Vec<MusicFile> = Vec::new();
        musicfiles.push(MusicFile::new(Path::new("/home/music/test.mp3"), "X".to_string(), "Y".to_string(), "Z".to_string(), 1900));
        assert_eq!(musicfiles_empty, search(musicfiles, "".to_string()));    
    }
    #[test]
    fn search_with_bad_pattern(){
        let musicfiles_empty:Vec<MusicFile> = Vec::new();
        let mut musicfiles_result:Vec<MusicFile> = Vec::new();
        musicfiles_result = search(musicfiles_result, "FAFAFA=X".to_string());
        assert_eq!(musicfiles_empty, musicfiles_result);        
    }
}
