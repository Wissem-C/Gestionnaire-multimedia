use markdown_gen::markdown::List;
use markdown_gen::markdown::Markdown;
use std::fs::File;

pub fn write2() {
    let file = File::create("test.md").unwrap();
    let mut md = Markdown::new(file);

    md.write(
        List::new(true)
            .item("Request result")
            .item("item 1")
            .item("bold"),
    );
}
