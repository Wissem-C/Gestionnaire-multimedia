use crate::musicfile::MusicFile;
use crate::Search::display;
use markdown_gen::markdown::AsMarkdown;
use markdown_gen::markdown::List;
use markdown_gen::markdown::Markdown;
use std::fs::File;

pub fn write2(music_file_vec: &Vec<MusicFile>) {
    let file = File::create("test.md").unwrap();
    let mut md = Markdown::new(file);

    for musicfile in music_file_vec {
        // md.write("Nouvelle musisque".heading(2)).unwrap();
        md.write("Musique trouvée :".heading(1)).unwrap();

        // md.write(musicfile.artist.as_str().trim()).unwrap();
        // md.write(musicfile.title.as_str()).unwrap();
        // md.write(musicfile.album.as_str()).unwrap();
        // md.write(musicfile.year.as_str().trim()).unwrap();

        // md.write(
        //     "Links: "
        //         .paragraph()
        //         .append(musicfile.artist.as_str())
        //         .append(musicfile.title.as_str())
        //         .append(musicfile.album.as_str()),
        // )
        // .unwrap();

        //  md.write("Heading".heading(1)).unwrap();
        //  md.write("quote".quote()).unwrap();
        md.write(musicfile.artist.paragraph()).unwrap();
        md.write(musicfile.title.paragraph()).unwrap();
        md.write(musicfile.album.paragraph()).unwrap();
        md.write(musicfile.year.paragraph()).unwrap();

        //  md.write(musicfile.artist);
    }
}
