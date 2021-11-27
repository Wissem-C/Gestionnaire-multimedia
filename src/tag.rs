use std::io;
extern crate chrono;
use crate::menu2::the_menu_yes_no;

use crate::musicfile::MusicFile;
use std::process::exit;

pub fn tag_music(music_file: Vec<MusicFile>) {
    //let mut save_result: Vec<MusicFile> = Vec::new();
    //    print!("\x1B[2J\x1B[1;1H");
    println!("Voulez vous rajouter un tag sur les musiques de votre recherche ? \nYes \nNo");
    let input = the_menu_yes_no();
    if input.trim() == "Yes" {
        println!("Indiquez le tag que vous voulez utiliser : ");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("FAILED TO READ ENTRY");
        for mut music in music_file {
            music.comment = "Coucou".to_string();
        }
    } else if input == "Exit" {
        exit(1);
    }
}
