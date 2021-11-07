use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};
extern crate mp3_metadata;
use crate::cli::CliArguments;
use crate::musicfile::MusicFile;
use crate::scan::scan;
use std::fs::File;
use std::io::Write;
//use chrono::prelude::{DateTime, Utc};
use std::io;
use std::{fs, panic};

//use std::time::{Duration, SystemTime};

// pub fn search_global(recherche: String, path: CliArguments) {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Echec de la lecture de l'entrée");

//     let args = CliArguments::new();
//     scan(args.path());
// }

pub fn search_global(recherche: String, recherche2: String) {
    // return option pour la loop dans main ?
    if recherche.trim() == "Artist" {
        search_by_artist(recherche2.trim());
    } else if recherche.trim() == "Title" {
        search_by_title(recherche2.trim());
    } else if recherche.trim() == "Year" {
        search_by_year(recherche2.trim());
    } else {
        println!("Une des donnée renseigné à été mal tapé");
        println!("recherche{}recherche2{:?}", recherche, recherche2);
    }
}

pub fn search_by_artist(artist: &str) {
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();
    let music_files: Vec<MusicFile>;

    music_files = serde_json::from_reader(file).expect("error while reading or parsing");
    for music in music_files {
        if artist == music.artist {
            println!(
            "Voici ce que j'ai trouvé :\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
           // music_file.path,
            music.artist,
            music.title,
            music.album,
            music.year,
            music.creation_date,
            music.last_access,
            music.last_modif


        );
        }
    }
}

pub fn search_by_title(title: &str) {
    let music_files: Vec<MusicFile>;
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();

    music_files = serde_json::from_reader(file).expect("error while reading or parsing");
    for music in music_files {
        if title == music.title {
            println!(
            "Voici ce que j'ai trouvé :\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
           // music_file.path,
            music.artist,
            music.title,
            music.album,
            music.year,
            music.creation_date,
            music.last_access,
            music.last_modif


        );
        }
    }
}

pub fn search_by_year(year: &str) {
    let music_files: Vec<MusicFile>;
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();

    music_files = serde_json::from_reader(file).expect("error while reading or parsing");
    for music in music_files {
        if year == music.year {
            println!(
            "Voici ce que j'ai trouvé :\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
           // music_file.path,
            music.artist,
            music.title,
            music.album,
            music.year,
            music.creation_date,
            music.last_access,
            music.last_modif


        );
        }
    }
}

/* let file = fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json")
    .expect("file should open read only");

let musics: Vec<MusicFile> =
    serde_json::from_reader(file).expect("error while reading or parsing");
for music in musics {
    println!("Hello {}", music.artist)
}*/
