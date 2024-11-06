use bincode::serialize;

pub struct Document {
    content: Vec<String>,
}

impl Document {
    pub fn new() -> Document {
        Document { content: vec![] }
    }

    pub fn content(&self) -> &Vec<String> {
        &self.content
    }

    pub fn add_line(&mut self, line: String) {
        self.content.push(line);
    }

    pub fn insert_line(&mut self, index: usize, line: String) {
        self.content.insert(index, line);
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn erase_line(&mut self, index: usize) {
        self.content.remove(index);
    }

    pub fn replace_text(&mut self, old_text: &str, new_text: &str) {
        for line in self.content.iter_mut() {
            *line = line.replace(old_text, new_text);
        }
    }

    pub fn create_snapshot(&self) -> DocumentSnapshot {
        bincode::serialize(&self.content).unwrap()
    }

    pub fn restore_snapshot(&mut self, snapshot: DocumentSnapshot) {
        self.content = bincode::deserialize(&snapshot).unwrap();
    }
}

pub type DocumentSnapshot = Vec<u8>;

#[cfg(test)]
pub(crate) mod tests_document {
    use rstest::{fixture, rstest};
    use crate::document::Document;

    #[test]
    fn document_is_empty_on_start() {
        let document = Document::new();
        assert_eq!(document.content(), &Vec::<String>::new());
    }

    #[test]
    fn document_add_line() {
        let mut document = Document::new();
        document.add_line("Hello, world!".to_string());
        assert_eq!(document.content(), &vec!["Hello, world!"]);
    }

    #[test]
    fn document_clear_content() {
        let mut document = Document::new();
        document.add_line("Hello, world!".to_string());
        document.add_line("Goodbye, world!".to_string());
        document.clear();
        assert_eq!(document.content(), &Vec::<String>::new());
    }

    #[fixture]
    pub fn document() -> Document {
        let mut doc = Document::new();
        doc.add_line("Line1".to_string());
        doc.add_line("Line2".to_string());
        doc.add_line("Line3".to_string());
        doc
    }

    #[rstest]
    fn document_inserting_line(mut document: Document) {
        document.insert_line(1, "Inserted line".to_string());
        assert_eq!(
            document.content(),
            &vec!["Line1", "Inserted line", "Line2", "Line3"]
        );
    }

    #[rstest]
    fn document_erase_a_line(mut document: Document) {
        document.erase_line(1);
        assert_eq!(document.content(), &vec!["Line1", "Line3"]);
    }

    #[rstest]
    fn document_replace_text(mut document: Document) {
        document.replace_text("Line2", "Replaced line");
        assert_eq!(document.content(), &vec!["Line1", "Replaced line", "Line3"]);
    }

    #[rstest]
    fn document_create_snaphot_that_restores_the_state(mut document: Document) {
        let snapshot = document.create_snapshot();

        document.clear();
        assert!(document.content().is_empty());

        document.restore_snapshot(snapshot);

        assert_eq!(document.content(), &vec!["Line1", "Line2", "Line3"]);
    }
}
