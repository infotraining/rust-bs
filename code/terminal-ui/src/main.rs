mod text_viewer;
mod doc;

fn main() {
    let (term_width, term_height) = termion::terminal_size().unwrap();

    let mut text_viewer = text_viewer::TextViewBuilder::new()
        // .with_file_name("./doc_short.txt")
        .with_file_name("./doc_long.txt")
        .with_cursor_pos(1, 1)
        .with_terminal_size(term_width as usize, term_height as usize)
        .build();

    text_viewer.show_document();
    text_viewer.run();
}
