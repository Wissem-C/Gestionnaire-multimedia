// use std::path::Path;
// use std::process::Command;
// use walkdir::{DirEntry, WalkDir};
extern crate mp3_metadata;
// use crate::cli::CliArguments;
use crate::{musicfile::MusicFile, write2md::write2};
// use crate::scan::scan;
// use std::fs::File;
// use std::io::Write;
//use chrono::prelude::{DateTime, Utc};
use std::time::{Duration, SystemTime};
// use std::io;
use std::{fs, panic};

//use std::time::{Duration, SystemTime};

pub fn search_global(recherche: String, recherche2: String) {
    // return option pour la loop dans main ?
    if recherche.trim().contains("artist") || recherche.trim().contains("Artist") {
        write2(search_by_artist(recherche2.trim()));
    } else if recherche.trim().contains("title") || recherche.trim().contains("Title") {
        write2(search_by_title(recherche2.trim()));
    } else if recherche.trim().contains("year") || recherche.trim().contains("Year") {
        write2(search_by_year(recherche2.trim()));
    } else if recherche.trim().contains("album") || recherche.trim().contains("Album") {
        write2(search_by_albums(recherche2.trim()))
    } else {
        println!("recherche{}recherche2{:?}", recherche, recherche2);
        panic!("One of the data entered was incorrectly typed");
    }
}

pub fn search_by_artist(artist: &str) -> Vec<MusicFile> {
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();
    let music_files: Vec<MusicFile>;
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();

    music_files = serde_json::from_reader(file).expect("ERROR WHILE READING OR PARSING");
    for music in music_files {
        if artist.contains(&music.artist) {
            // music_file_stockage.push(MusicFile::new(
            //     &Path,
            //     music.artist,
            //     music.title,
            //     music.album,
            //     music.year,
            //     music.creation_date,
            //     music.last_access,
            //     music.last_modif,
            // ));

            println!(
                "RESULT OF THE QUERY :\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
               // music_file.path,
                music.artist,
                music.title,
                music.album,
                music.year,
                music.creation_date,
                music.last_access,
                music.last_modif,

            );
            music_file_stockage.push(music);
        }
    }
    //println!("Le vecteur de retour :{:?}", music_file_stockage);
    return music_file_stockage;
}

pub fn search_by_title(title: &str) -> Vec<MusicFile> {
    let music_files: Vec<MusicFile>;
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();

    music_files = serde_json::from_reader(file).expect("ERROR WHILE READING OR PARSING");
    for music in music_files {
        if title.contains(&music.title) {
            println!(
                "\nRESULT OF THE QUERY :\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
               // music_file.path,
                music.artist,
                music.title,
                music.album,
                music.year,
                music.creation_date,
                music.last_access,
                music.last_modif

            );
            music_file_stockage.push(music);
        }
    }
    println!("Le vecteur de retour :{:?}", music_file_stockage);
    return music_file_stockage;
}

pub fn search_by_year(year: &str) -> Vec<MusicFile> {
    let music_files: Vec<MusicFile>;
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();

    music_files = serde_json::from_reader(file).expect("ERROR WHILE READING OR PARSING");
    for music in music_files {
        if year.contains(&music.year) {
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
        music_file_stockage.push(music);
    }
    println!("Le vecteur de retour :{:?}", music_file_stockage);
    return music_file_stockage;
}

pub fn search_by_albums(album: &str) -> Vec<MusicFile> {
    let music_files: Vec<MusicFile>;
    let file =
        fs::File::open("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json").unwrap();
    let mut music_file_stockage: Vec<MusicFile> = Vec::new();

    music_files = serde_json::from_reader(file).expect("ERROR WHILE READING OR PARSING");
    for music in music_files {
        if album.contains(&music.album) {
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
        music_file_stockage.push(music);
    }
    println!("Le vecteur de retour :{:?}", music_file_stockage);
    return music_file_stockage;
}
