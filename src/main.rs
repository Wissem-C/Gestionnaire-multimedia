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
extern crate chrono;
use std::process::exit;

//use std::io::prelude::*;

//use std::io::prelude::*;
//use std::io::Write;

//use std::time::{Duration, SystemTime, UNIX_EPOCH};

// // FONCTION MAIN PRINCIPALE

fn main() {
    let cyan = Style::new().cyan();
    println!("WELCOME TO THE SMALL MP3 AUDIO FILE MANAGER");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You have not entered any arguments, do you want to switch to interactive mode ? Answer with \nYes\nNo");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Echec de la lecture de l'entrée");

        if input.trim().contains("Yes") {
            interactif();
            loop {
                println!("Do you have another order to make? Answer with \nYes\nNo");
                let mut input2 = String::new();
                io::stdin()
                    .read_line(&mut input2)
                    .expect("Echec de la lecture de l'entrée");
                if input2.trim().contains("Yes") {
                    interactif();
                    continue;
                } else {
                    println!("Fin de programme");
                    exit(1);
                }
            }
        } else if input.trim().contains("No") {
            println!("Fin de programme");
            exit(1);
        } else {
            println!("Fin de programme");
            exit(1);
        }
    } else {
        let test = CliArguments::new();
        command_scan_search(test);
    }
}
