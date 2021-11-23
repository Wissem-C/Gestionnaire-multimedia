use crate::musicfile::MusicFile;
use markdown_gen::markdown::AsMarkdown;
use markdown_gen::markdown::List;
use markdown_gen::markdown::Markdown;
use std::fs::File;

pub fn write2(music_file_vec: &[MusicFile]) {
    let file = File::create("test.md").unwrap();
    let mut md = Markdown::new(file);

    md.write("Musics found: ".heading(1)).unwrap();

    for musicfile in music_file_vec {
        md.write(
            List::new(false)
                .title("\nNew Song :")
                .item(musicfile.artist.as_str())
                .item(musicfile.album.as_str())
                .item(musicfile.title.as_str())
                .item(musicfile.year.as_str()),
        )
        .unwrap();
    }
}
