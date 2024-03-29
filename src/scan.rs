use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};
extern crate mp3_metadata;
use crate::musicfile::MusicFile;
use crate::search::search_by_artist;
use crate::search::search_by_title;
use crate::search::search_by_year;
use std::fs::File;
use std::io::Write;
use std::{fs, panic};

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file()
}

pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut _cmp = 0;
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();

    //delete_dir : Supprime le fichier DS_STORE avant de rentrer dans la boucle et d'appeler is_supported.
    delete_dir(path);
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => panic!("ERROR TYPE OF FILE"),
        };

        if is_supported(&entry) {
            let meta_song =
                mp3_metadata::read_from_file(entry.path()).expect("FILE ERROR MP3_METADATA");
            let meta_file = fs::metadata(entry.path()).expect("FILE ERROR FS");

            if let Some(tag) = meta_song.tag {
                music_files.push(MusicFile::new(
                    entry.path(),
                    tag.artist.trim_end_matches('\0').to_owned(),
                    tag.title.trim_end_matches('\0').to_owned(),
                    tag.album.trim_end_matches('\0').to_owned(),
                    format!("{}", tag.year),
                    meta_file.created().unwrap(),
                    meta_file.accessed().unwrap(),
                    meta_file.modified().unwrap(),
                    tag.comment.to_string(),
                ));
            }

            _cmp += 1;
        }
    }
    //println!("\nNumber of songs added : {}", cmp);
    serialise(&music_files);
    //let result_deserialise = deserialise(&result_serialise);

    //println!("serialized = {}\n", result_serialise);
    // println!("deserialized = {:?}\n", result_deserialise);

    music_files
}

// Fonction to delete "DS_STORE" mac files when scanning on directory causing errors
// find . -name '.DS_Store' -type f -delete
pub fn delete_dir(path: &Path) {
    let mut list_dir = Command::new("rm");
    list_dir.arg(".DS_STORE");
    list_dir.current_dir(&path);
}

pub fn serialise(music_file: &[MusicFile]) -> String {
    let mut file = File::create("/Users/wissemcherifi/Desktop/medman-Wissem-C-main/src/save.json")
        .expect("ERROR SERIALISATION");

    let serialized = serde_json::to_string_pretty(&music_file).unwrap();

    file.write_all(serialized.as_bytes()).unwrap();

    serialized
}

pub fn deserialise(serialized: &str) -> Vec<MusicFile> {
    let deserialized: Vec<MusicFile> = serde_json::from_str(serialized).unwrap();

    deserialized
}

#[test]
fn test_scan_title() {
    let music = scan(Path::new("/Users/wissemcherifi/Desktop/testmusique/MHD"));
    let save = search_by_title("Jamais", music);
    assert_eq!("Jamais", save[0].title)
}

#[test]

fn test_scan_artist() {
    let music = scan(Path::new("/Users/wissemcherifi/Desktop/testmusique/MHD"));
    let save = search_by_artist("MHD", music);
    assert_eq!("MHD", save[0].artist)
}
#[test]
fn test_scan_year() {
    let music = scan(Path::new("/Users/wissemcherifi/Desktop/testmusique"));
    let save = search_by_year("2021", music);
    assert_eq!("2021", save[0].year)
}
