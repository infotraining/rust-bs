mod document;

use document::Document;

fn main() {
    let mut doc = Document::new();
    doc.add_line("Hello, world!".to_string());
    doc.add_line("Goodbye, world!".to_string());
}
