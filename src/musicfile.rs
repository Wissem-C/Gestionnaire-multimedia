use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct MusicFile {
    pub path: PathBuf,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub year: String,
    pub creation_date: SystemTime,
    pub last_access: SystemTime,
    pub last_modif: SystemTime,
    pub comment: String,
}

impl MusicFile {
    pub fn new(
        path: &Path,
        artist: String,
        title: String,
        album: String,
        year: String,
        creation_date: SystemTime,
        last_access: SystemTime,
        last_modif: SystemTime,
        comment: String,
    ) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            artist,
            title,
            album,
            year,
            creation_date,
            last_access,
            last_modif,
            comment,
        }
    }
}
