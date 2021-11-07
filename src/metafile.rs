#[derive(Debug)]
pub struct Meta {
    artist: String,
    title: String,
    album: String,
    year: u16,
}

impl Meta {
    pub fn new(a: String, b: String, c: String, d: u16) -> Meta {
        Meta {
            title: a,
            artist: b,
            album: c,
            year: d,
        }
    }
}
