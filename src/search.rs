// use std::path::Path;
// use std::process::Command;
// use walkdir::{DirEntry, WalkDir};
extern crate mp3_metadata;

use crate::musicfile;
// use crate::cli::CliArguments;
use crate::{musicfile::MusicFile, write2md::write2};
use std::process::exit;

// use crate::scan::scan;
// use std::fs::File;
// use std::io::Write;
//use chrono::prelude::{DateTime, Utc};
//use std::time::{Duration, SystemTime};
// use std::io;
use std::io;
use std::{fs, panic};

//use std::time::{Duration, SystemTime};

pub fn search_global(recherche: String) {
    let mut save_result: Vec<MusicFile> = Vec::new();
    let music_file_stockage = get_vec_serialized();
    for music in music_file_stockage {
        if music.album.contains(recherche.trim()) {
            save_result.push(music);
        } else if music.title.contains(recherche.trim()) {
            save_result.push(music);
        } else if music.artist.contains(recherche.trim()) {
            save_result.push(music);
        } else if music.year.contains(recherche.trim()) {
            save_result.push(music);
        }
        write2(&save_result);
        println!("Oui mais y'a rien {}", recherche);
        display(&save_result);
    }
}

pub fn search_intractif(recherche: String, recherche2: String) {
    //let mut music_file_stockage: Vec<MusicFile> = Vec::new();
    if recherche.trim().contains("artist") || recherche.trim().contains("Artist") {
        // search_by_artist(recherche2.trim());
        // music_file_stockage = search_by_artist(recherche2.trim(), get_vec_serialized());
        // let mut music_file_stockage2 = improve_search(music_file_stockage);
        //display(music_file_stockage2);
        display(&improve_search(search_by_artist(
            recherche2.trim(),
            get_vec_serialized(),
        )));
        //write2(search_by_artist(recherche2.trim()));
    } else if recherche.trim().contains("title") || recherche.trim().contains("Title") {
        display(&improve_search(search_by_title(
            recherche2.trim(),
            get_vec_serialized(),
        )));
    } else if recherche.trim().contains("year") || recherche.trim().contains("Year") {
        display(&improve_search(search_by_year(
            recherche2.trim(),
            get_vec_serialized(),
        )));
    } else if recherche.trim().contains("album") || recherche.trim().contains("Album") {
        display(&improve_search(search_by_albums(
            recherche2.trim(),
            get_vec_serialized(),
        )));
    } else {
        println!("recherche{}recherche2{:?}", recherche, recherche2);
        panic!("One of the data entered was incorrectly typed");
    }
}

pub fn get_vec_serialized() -> Vec<MusicFile> {
    let music_files: Vec<MusicFile>;
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();

    music_files = serde_json::from_reader(file).expect("ERROR WHILE READING OR PARSING");
    music_files
}

pub fn search_by_artist(artist: &str, musics: Vec<MusicFile>) -> Vec<MusicFile> {
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();
    for music in musics {
        if artist.contains(&music.artist) {
            music_file_stockage.push(music);
        }
    }
    //println!("Le vecteur de retour :{:?}", music_file_stockage);
    music_file_stockage
}

pub fn search_by_title(title: &str, musics: Vec<MusicFile>) -> Vec<MusicFile> {
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();
    for music in musics {
        if title.contains(&music.title) {
            music_file_stockage.push(music);
        }
    }
    //  println!("Le vecteur de retour :{:?}", music_file_stockage);
    music_file_stockage
}

pub fn search_by_year(year: &str, musics: Vec<MusicFile>) -> Vec<MusicFile> {
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();
    for music in musics {
        if year.contains(&music.year) {
            music_file_stockage.push(music);
        }
    }
    //println!("Le vecteur de retour :{:?}", music_file_stockage);
    music_file_stockage
}

pub fn search_by_albums(album: &str, musics: Vec<MusicFile>) -> Vec<MusicFile> {
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();

    for music in musics {
        if album.contains(&music.album) {
            music_file_stockage.push(music);
        }
    }
    //println!("Le vecteur de retour :{:?}", music_file_stockage);
    music_file_stockage
}

pub fn display(music_files: &Vec<MusicFile>) {
    for music in music_files {
        println!(
                "RESULT OF THE QUERY :\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
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

pub fn improve_search(music_files: Vec<MusicFile>) -> Vec<MusicFile> {
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();
    println!("Voulez vous affiner votre recherche par année, titre ou par album ? Tapez votre réponse \nAlbum\nYear\nTitle\nNon");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("FAILED TO READ ENTRY");

    if "Title".contains(input1.trim()) {
        println!("Tapez le nom du titre pour afficher votre recherche:");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("FAILED TO READ ENTRY");

        music_file_stockage = search_by_title(&input2, music_files);
        write2(&music_file_stockage);

        music_file_stockage
    } else if "Year".contains(input1.trim()) {
        println!("Tapez le nom de l'année pour affiner pour recherche");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("FAILED TO READ ENTRY");

        music_file_stockage = search_by_year(&input2, music_files);
        write2(&music_file_stockage);

        music_file_stockage
    } else if "Album".contains(input1.trim()) {
        println!("Tapez le nom de l'année pour affiner pour recherche");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("FAILED TO READ ENTRY");

        music_file_stockage = search_by_albums(&input2, music_files);
        write2(&music_file_stockage);

        music_file_stockage
    } else {
        write2(&music_files);
        music_files
    }
}
