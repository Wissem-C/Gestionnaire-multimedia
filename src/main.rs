//use chrono::prelude::{DateTime, Utc};
use medman::cli::CliArguments;
//use medman::musicfile::MusicFile;
//use medman::musicfile;
//use medman::musicfile::MusicFile;
use medman::menu::command_scan_search;
use medman::menu::interactif;
//use medman::scan::scan;
//use medman::search::search_global;
//use structopt::StructOpt;

use std::env;
use std::io;
//use std::path::PathBuf;
use console::Style;
use std::process::exit;

//use std::io::prelude::*;

//use std::io::prelude::*;
//use std::io::Write;

//use std::time::{Duration, SystemTime, UNIX_EPOCH};

// // FONCTION MAIN PRINCIPALE

fn main() {
    let cyan = Style::new().cyan();
    println!("BIENVENU DANS LE PETIT GESTIONNAIRE DE FICHIER AUDIO MP3");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Vous navez pas rentré d'arguments, voulez vous passez en mode intéractif ? Répondez par\nOui\nNon");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Echec de la lecture de l'entrée");

        if input.trim().contains("Oui") {
            interactif();
            loop {
                println!("Avez vous une autre commande à réaliser ? Répondez par\nOui\nNon");
                let mut input2 = String::new();
                io::stdin()
                    .read_line(&mut input2)
                    .expect("Echec de la lecture de l'entrée");
                if input2.trim().contains("Oui") {
                    interactif();
                    continue;
                } else {
                    println!("Fin de programme");
                    exit(1);
                }
            }
        } else if input.trim().contains("Non") {
            println!("Fin de programme");
            exit(1);
        }
        println!("Fin de programme");
        exit(1);
    } else {
        let test = CliArguments::new();
        command_scan_search(test);
    }
}
