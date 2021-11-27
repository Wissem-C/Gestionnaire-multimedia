use crate::cli::{CliArguments, CliArguments2};
use crate::menu2::{the_menu_categorie, the_menu_path, the_menu_scan_search, the_menu_yes_no};
use crate::write2_playlist::write2_playlist;

use crate::scan::scan;
use crate::search::search_global;

use crate::search::search_intractif;

use console::Style;
use std::io;
extern crate chrono;
use crate::search::display;

use crate::tag::tag_music;
use chrono::DateTime;
use chrono::Utc;
use std::path::PathBuf;
use std::process::exit;

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
        write2_playlist(&search_global(&test.recherche));
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
            cmp += 1;

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
        let mut _cmp = 0;
        let music_files = scan(test.path());

        for music_file in &music_files {
            let a: DateTime<Utc> = music_file.creation_date.into();
            let b: DateTime<Utc> = music_file.last_access.into();
            let c: DateTime<Utc> = music_file.last_modif.into();
            _cmp += 1;

            println!(
                "\n {} ========== NEW SONG ========== \n ",
                red.apply_to(_cmp)
            );
            println!(
            "path: {:?}\nArtiste: {}\nTitle: {}\nAlbum: {}\nYear: {:?}\nCreation date : {:?}\nLast acess: {:?}\nLast modification : {:?}\n",
            cyan.apply_to(&music_file.path),
            cyan.apply_to(&music_file.artist),
            cyan.apply_to(&music_file.title),
            cyan.apply_to(&music_file.album),
            cyan.apply_to(&music_file.year),
            cyan.apply_to(a),
            cyan.apply_to(b),
            cyan.apply_to(c),

        );
        }
    } else if test.command == "search" {
        scan(test.path());

        println!(" WHAT TYPE ARE YOU LOOKING FOR ? ");
        let mut input = the_menu_categorie();
        io::stdin()
            .read_line(&mut input)
            .expect("Echec de la lecture de l'entrée");

        println!("Type the artist name, the music title, the album title or a year : ");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("FAILED TO READ ENTRY");

        //tag_music(search_intractif(input, input2));
        search_intractif(input, input2);
    } else {
        println!("NO COMMANDS RECOGNIZED");
    }
}

pub fn interactif() {
    println!("DO YOU WANT TO USE INTERATIVE MODE ?");
    let input = the_menu_yes_no();

    if input.trim().contains("Yes") {
        // PASSAGE EN MODE INTERATIF
        loop {
            println!("TYPE THE COMMAND YOU WANT TO USE  : ");
            let input = the_menu_scan_search();
            println!("GIVE THE PATH TO ANALYSE YOUR DATA : ");

            let input2 = the_menu_path();
            if input.trim().contains("Exit") {
                println!("Fin de programme");
                exit(1);
            }
            let path = PathBuf::from(input2.trim());

            let test = CliArguments {
                command: input.trim().to_string(),
                path,
            };
            command_scan_search_interactif(test);
            search_intractif(input, input2);

            println!("DO YOU WANT TO MAKE AN OTHER SCAN OR SEARCH ?:");
            let input2 = the_menu_yes_no();
            if input2.trim().contains("Yes") {
                continue;
            } else if input2.trim().contains("Exit") {
                println!("Fin de programme");
                exit(1);
            }
        }
    } else if input.trim().contains("Exit") {
        println!("Fin de programme");
        exit(1);
    }
}
