use medman::cli::CliArguments;
use medman::cli::CliArguments2;
use medman::menu::command_scan_auto;
use medman::menu::command_search_auto;
use medman::menu::interactif;
use std::env;

extern crate chrono;
use medman::menu::io_yes_no;
use std::process::exit;

// // FONCTION MAIN PRINCIPALE

fn main() {
    // A METTRE DANS LE MENU.RS !!!!!! ET APPELER LA FONCTION DEPUIS
    println!("WELCOME TO THE SMALL MP3 AUDIO FILE MANAGER");

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", args.len());
        loop {
            interactif();
            println!("Do you have another order to make? Answer with \nYes\nNo");
            let input2 = io_yes_no();
            if input2.trim().contains("Yes") {
                continue;
            } else {
                println!("Fin de programme");
                exit(1);
            }
        }
    } else if args.len() == 4 {
        println!("{}", args.len());
        command_search_auto(CliArguments2::new());
    } else if args.len() == 3 {
        println!("{}", args.len());
        command_scan_auto(CliArguments::new())
    } else {
        panic!("ARGUMENTS ERROR");
    }
}
