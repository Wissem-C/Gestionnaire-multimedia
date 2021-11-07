use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
//use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct MusicFile {
    pub path: PathBuf,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub year: String,
    pub creation_date: String,
    pub last_access: String,
    pub last_modif: String,
}

impl MusicFile {
    pub fn new(
        path: &Path,
        a: String,
        b: String,
        c: String,
        d: String,
        e: String,
        f: String,
        g: String,
    ) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            artist: a,
            title: b,
            album: c,
            year: d,
            creation_date: e,
            last_access: f,
            last_modif: g,
        }
    }

    //pub fn seria(music_file: MusicFile) {
    //  let j = serde_json::to_string(&music_file);
    //  let test = format!("{:?}", println!("{:?}", j));
    // println!("{}", music_file.album);
    //   }

    // pub fn get_so(&self) -> String {
    //   self.title.clone()
    //}
}
