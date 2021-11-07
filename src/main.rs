//use chrono::prelude::{DateTime, Utc};
use medman::cli::CliArguments;
use medman::musicfile::MusicFile;
//use medman::musicfile;
//use medman::musicfile::MusicFile;
use medman::scan::scan;
use medman::search::search_global;
use medman::search::{search_by_artist, search_by_year};
use std::env;
use std::io;
use std::path::PathBuf;
//use std::io::prelude::*;

//use std::io::prelude::*;
//use std::io::Write;

//use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    println!("BIENVENU DANS LE PETIT GESTIONNAIRE DE FICHIER AUDIO MP3");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Echec de la lecture de l'entrée");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Echec de la lecture de l'entrée");

    let path = PathBuf::from(input2.trim());

    let test = CliArguments {
        command: input.trim().to_string(),
        path: path,
    };
    //let music_files: Vec<MusicFile>;
    if test.command == "scan" {
        let mut cmp = 0;
        let music_files = scan(test.path());

        for music_file in &music_files {
            //     //let modif: DateTime<Utc> = music_file.creation_date.as_secs();
            //     //println!("SONG SAVED : {:?} ", music_meta);
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

    //let args = CliArguments::new();
    // args.command = input;
    // args.path.push(input2);

    //println!("{:?}", test);

    // search_by_year("2020", music_files);
    //search_by_artist("Drake", music_files);
}
