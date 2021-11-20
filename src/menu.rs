//use chrono::prelude::{DateTime, Utc};
use crate::cli::{CliArguments, CliArguments2};

//use medman::musicfile;
//use medman::musicfile::MusicFile;
use crate::scan::scan;
use crate::search::search_global;
//use structopt::StructOpt;
use crate::search::search_intractif;

//use std::env;
use console::Style;
use std::io;
extern crate chrono;
use chrono::{DateTime, TimeZone};
use std::path::PathBuf;
use std::process::exit;

//use std::process::exit;
//use std::io::prelude::*;

pub fn io_yes_no() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Echec de la lecture de l'entrée");
    input
}

pub fn command_search_auto(test: CliArguments2) {
    if test.command == "search" {
        scan(test.path());

        println!("{} BLABLA ", test.recherche);
        search_global(test.recherche);

        // println!("PAS ENCORE IMPLÉMENTÉ");
    } else {
        panic!("NO COMMANDS RECOGNIZED");
    }
}

pub fn command_scan_auto(test: CliArguments) {
    let cyan = Style::new().magenta();
    let red = Style::new().cyan();
    if test.command == "scan" {
        let mut cmp = 0;
        let music_files = scan(test.path());

        for music_file in &music_files {
            cmp = cmp + 1;

            println!(
                "\n {} ========== NEW SONG ========== \n ",
                red.apply_to(cmp)
            );
            println!(
            "path: {:?}\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
            cyan.apply_to(&music_file.path),
            cyan.apply_to(&music_file.artist),
            cyan.apply_to(&music_file.title),
            cyan.apply_to(&music_file.album),
            cyan.apply_to(&music_file.year),
            cyan.apply_to(&music_file.creation_date),
            cyan.apply_to(&music_file.last_access),
            cyan.apply_to(&music_file.last_modif),

        );
        }
    } else {
        panic!("NO COMMANDS RECOGNIZED");
    }
}

pub fn command_scan_search_interactif(test: CliArguments) {
    let cyan = Style::new().magenta();
    let red = Style::new().cyan();
    if test.command == "scan" {
        let mut cmp = 0;
        let music_files = scan(test.path());

        for music_file in &music_files {
            cmp = cmp + 1;

            println!(
                "\n {} ========== NEW SONG ========== \n ",
                red.apply_to(cmp)
            );
            println!(
            "path: {:?}\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
            cyan.apply_to(&music_file.path),
            cyan.apply_to(&music_file.artist),
            cyan.apply_to(&music_file.title),
            cyan.apply_to(&music_file.album),
            cyan.apply_to(&music_file.year),
            cyan.apply_to(&music_file.creation_date),
            cyan.apply_to(&music_file.last_access),
            cyan.apply_to(&music_file.last_modif),

        );
        }
    } else if test.command == "search" {
        scan(test.path());
        println!(
            "What type are you looking for? Choose one of the possibilities: \nArtist\nTitle\nYear\nAlbum"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Echec de la lecture de l'entrée");

        println!("Type the artist name, the music title, the album title or a year : ");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("FAILED TO READ ENTRY");

        search_intractif(input, input2)
    } else {
        println!("NO COMMANDS RECOGNIZED");
    }
}

pub fn interactif() {
    println!("Do you want to use the interactive mode ? Answer with \nYes\nNo");

    let input = io_yes_no();

    if input.trim().contains("Yes") {
        // PASSAGE EN MODE INTERATIF
        loop {
            println!("Type the command you want to use: \nscan\nsearch");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("FAILED TO READ ENTRY");

            println!("Give the path to analyze your data");

            let mut input2 = String::new();
            io::stdin()
                .read_line(&mut input2)
                .expect("FAILED TO READ ENTRY");

            let path = PathBuf::from(input2.trim());

            let test = CliArguments {
                command: input.trim().to_string(),
                path: path,
            };
            command_scan_search_interactif(test);

            println!("Do you have another order to make? Answer with \nYes\nNo");
            let input2 = io_yes_no();
            if input2.trim().contains("Yes") {
                continue;
            } else {
                println!("Fin de programme");
                exit(1);
            }
        }
    } else {
        println!("Fin de programme");
        exit(1);
    }
}
