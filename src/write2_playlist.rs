use crate::musicfile::MusicFile;
use pls::*;
use std::fs::{self, File};
use std::io::Write;
//use crate::{musicfile::MusicFile, search::advancedsearch};

pub fn write2_playlist(music: &Vec<MusicFile>) {
    let mut _file = File::create("Playlist.pls").expect("Error creation of the file pls");
    //let result = search_global(keyword.trim_end_matches("\n").to_string());
    let mut save = Vec::new();
    let mut playlist: Vec<PlaylistElement> = Vec::new();

    for res in music {
        playlist.push(PlaylistElement {
            path: res.path.to_string_lossy().to_string(),
            title: Some(res.title.to_string()),
            len: ElementLength::Unknown,
        })
    }

    pls::write(&playlist, &mut save).expect("Could not create");
    _file.write(&mut save).expect("ERROR");

    match fs::write("Playlist", save) {
        Ok(()) => println!("Creation of the playlist"),
        Err(_) => panic!("Could not create the playlist of musics"),
    }
}
