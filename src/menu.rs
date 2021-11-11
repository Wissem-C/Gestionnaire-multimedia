//use chrono::prelude::{DateTime, Utc};
use crate::cli::CliArguments;

//use medman::musicfile;
//use medman::musicfile::MusicFile;
use crate::scan::scan;
use crate::search::search_global;
//use structopt::StructOpt;

//use std::env;
use std::io;
use std::path::PathBuf;
//use std::process::exit;
//use std::io::prelude::*;

pub fn command_scan_search(test: CliArguments) {
    if test.command == "scan" {
        let mut cmp = 0;
        let music_files = scan(test.path());

        for music_file in &music_files {
            cmp = cmp + 1;

            println!("\n {} ========== NEW SONG ========== \n ", cmp);
            println!(
            "path: {:?}\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
            music_file.path,
            music_file.artist,
            music_file.title,
            music_file.album,
            music_file.year,
            music_file.creation_date,
            music_file.last_access,
            music_file.last_modif

        );
        }
    } else if test.command == "search" {
        scan(test.path());

        println!("Par quel type voulez vous recherchez ? Choisissez une des possibilités :\nArtist\nTitle\nYear");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Echec de la lecture de l'entrée");

        println!("Tapez le nom de l'artiste, le titre de la musique ou l'année : ");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("Echec de la lecture de l'entrée");

        search_global(input, input2)

        // println!("PAS ENCORE IMPLÉMENTÉ");
    } else {
        println!("AUCUNE COMMANDE RECONNUE");
    }
}

pub fn interactif() {
    // PASSAGE EN MODE INTERATIF
    println!("Tapez la commande que vous voulez utiliser :\nscan\nsearch");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Echec de la lecture de l'entrée");

    println!("Donnez le chemin pour analyser vos donner");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Echec de la lecture de l'entrée");

    let path = PathBuf::from(input2.trim());

    let test = CliArguments {
        command: input.trim().to_string(),
        path: path,
    };
    command_scan_search(test);
}
