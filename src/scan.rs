use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};
extern crate mp3_metadata;
use crate::musicfile::MusicFile;
use std::fs::File;
use std::io::Write;
//use chrono::prelude::{DateTime, Utc};
use std::{fs, panic};
//use std::time::{Duration, SystemTime};

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut cmp = 0;
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();

    //delete_dir : Supprime le fichier DS_STORE avant de rentrer dans la boucle et d'appeler is_supported.
    delete_dir(path);
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => panic!("ERROR TYPE OF FILE"),
        };

        if is_supported(&entry) {
            let meta_song =
                mp3_metadata::read_from_file(entry.path()).expect("File error mp3_metadata");
            let meta_file = fs::metadata(entry.path()).expect("Error mp3 metadata");

            if let Some(tag) = meta_song.tag {
                music_files.push(MusicFile::new(
                    entry.path(),
                    tag.artist.trim_end_matches('\0').to_owned(),
                    tag.title.trim_end_matches('\0').to_owned(),
                    tag.album.trim_end_matches('\0').to_owned(),
                    format!("{}", tag.year),
                    format!("{:?}", meta_file.created()),
                    format!("{:?}", meta_file.accessed()),
                    format!("{:?}", meta_file.modified()),
                ));
            }

            cmp = cmp + 1;
        }

        /*let res = serde_json::from_str(&serialized);
        if res.is_ok() {
            let p: MusicFile = res.unwrap();
            println!("The name of song is => {}", p.title);
            println!("The name of the album's song is => {}", p.album);
            println!("The name of the artist is => {}", p.artist);
            println!("The year of the song is =>  {}", p.year);
            println!("The creation date of the file => {}", p.creation_date);
        } else {
            panic!("ALERT");
        }
        */
    }
    println!("\nNumber of songs added : {}", cmp);
    let result_serialise = serialise(&music_files);
    let result_deserialise = deserialise(&result_serialise);

    println!("serialized = {}\n", result_serialise);
    println!("deserialized = {:?}\n", result_deserialise);

    // let file = fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json")
    //     .expect("file should open read only");

    //let musics: Vec<MusicFile> =
    //serde_json::from_reader(file).expect("error while reading or parsing");
    // for music in &music_files {
    //     println!("Hello {}", music.artist)
    // }
    music_files
}

// Fonction to delete "DS_STORE" mac files when scanning on directory causing errors
// find . -name '.DS_Store' -type f -delete
pub fn delete_dir(path: &Path) {
    let mut list_dir = Command::new("rm");
    list_dir.arg(".DS_STORE");
    list_dir.current_dir(&path);

    if list_dir.status().is_ok() {
        println!("\nLE FICHIER DS_STORE À BIEN ÉTÉ SUPPRIMÉ OU N'EST PAS PRÉSENT")
    } else {
        panic!("LE FICHIER DS_STORE POSE PROBLEME !!");
    }
}

pub fn serialise(music_file: &Vec<MusicFile>) -> String {
    let mut file = File::create("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json")
        .expect("Error");

    let serialized = serde_json::to_string(&music_file).unwrap();

    file.write_all(serialized.as_bytes()).unwrap();

    serialized
}

pub fn deserialise(serialized: &str) -> Vec<MusicFile> {
    let deserialized: Vec<MusicFile> = serde_json::from_str(&serialized).unwrap();

    deserialized
}
