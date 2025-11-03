use std::fs::read_to_string;
use std::io;
use std::ops::Index;

pub struct Doc {
    lines: Vec<String>,
    file_name: Option<String>,
}

impl Index<usize> for Doc {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.lines[index]
    }
}

impl Doc {
    pub fn new() -> Self {
        Self {
            lines: vec!["Hello World!".to_string()],
            file_name: None,
        }
    }

    #[allow(dead_code)]
    pub fn from_lines(lines: Vec<String>) -> Self {
        Self {
            lines,
            file_name: None,
        }
    }

    pub fn load_from_file(filename: &str) -> Result<Doc, io::Error> {
        let mut doc = Doc {
            lines: vec![],
            file_name: None,
        };
        let file_content = read_to_string(filename)?;
        doc.lines = file_content.lines().map(|l| l.to_string()).collect();
        doc.file_name = Some(filename.to_string());
        Ok(doc)
    }

    pub fn length(&self) -> usize {
        self.lines.len()
    }

    pub fn file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    #[allow(dead_code)]
    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn insert_char(&mut self, line: usize, col: usize, ch: char) {
        if line >= self.lines.len() {
            return;
        }
        let target_line = &mut self.lines[line];
        if col > target_line.len() {
            return;
        }
        target_line.insert(col, ch);
    }

    pub fn remove(&mut self, line: usize, col: usize) {
        if line >= self.lines.len() {
            return;
        }
        // if col == 0 {
        //     if line == 0 {
        //         return;
        //     }
        //     let current_line_content = self.lines.remove(line);
        //     self.lines[line - 1].push_str(&current_line_content);
        //     return;
        // }

        let target_line = &mut self.lines[line];
        if col >= target_line.len() {
            return;
        }
        let end = usize::min(col + 1, target_line.len());
        target_line.drain(col..end);
    }

    pub fn insert_new_line(&mut self, line: usize, col: usize) {
        if line >= self.lines.len() {
            return;
        }
        let target_line = &mut self.lines[line];
        if col > target_line.len() {
            return;
        }
        let new_line_content = target_line.split_off(col);
        self.lines.insert(line + 1, new_line_content);
    }

    pub fn join_with_next_line(&mut self, line: usize) {
        if line + 1 >= self.lines.len() {
            return;
        }
        let next_line_content = self.lines.remove(line + 1);
        self.lines[line].push_str(&next_line_content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use io::Write;

    #[test]
    fn new_doc_has_default_content() {
        let doc = Doc::new();
        assert_eq!(doc.length(), 1);
        assert_eq!(doc[0], "Hello World!");
        assert_eq!(doc.file_name(), None);
    }

    #[test]
    fn loading_content_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Line 1").unwrap();
        writeln!(temp_file, "Line 2").unwrap();
        writeln!(temp_file, "Line 3").unwrap();

        let doc = Doc::load_from_file(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(doc.length(), 3);
        assert_eq!(doc[0], "Line 1");
        assert_eq!(doc[1], "Line 2");
        assert_eq!(doc[2], "Line 3");
        assert!(doc.file_name().is_some());
    }

    #[test]
    fn loading_empty_file_creates_empty_doc() {
        let temp_file = NamedTempFile::new().unwrap();
        let doc = Doc::load_from_file(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(doc.length(), 0);
    }

    #[test]
    fn loading_nonexistent_file_returns_error() {
        let result = Doc::load_from_file("/nonexistent/path/to/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn indexing_of_lines() {
        let doc = Doc::new();
        assert_eq!(doc[0], "Hello World!");
    }

    #[test]
    fn inserting_char_in_the_middle_of_a_line() {
        let mut doc = Doc::new();
        doc.insert_char(0, 6, 'X');
        assert_eq!(doc[0], "Hello XWorld!");
    }

    #[test]
    fn inserting_char_at_end_of_line() {
        let mut doc = Doc::new();
        doc.insert_char(0, 12, '!');
        assert_eq!(doc[0], "Hello World!!");
    }   

    #[test]
    fn inserting_char_at_beginning_of_line() {
        let mut doc = Doc::new();
        doc.insert_char(0, 0, 'Y');
        assert_eq!(doc[0], "YHello World!");
    }

    #[test]
    fn removing_char_from_middle_of_line() {
        let mut doc = Doc::new();
        assert!(doc[0] == "Hello World!");
        doc.remove(0, 5,);
        assert_eq!(doc[0], "HelloWorld!");
    }

    #[test]
    fn join_the_lines() {
        let mut doc = Doc::from_lines(vec!["Line1".to_string(), "Line2".to_string()]);
        assert_eq!(doc.lines(), &vec!["Line1".to_string(), "Line2".to_string()]);
        assert_eq!(doc.length(), 2);

        doc.join_with_next_line(0);
        assert_eq!(doc[0], "Line1Line2");
        assert_eq!(doc.length(), 1);
    }

    #[test]
    fn inserting_new_line_in_the_middle_of_line() {
        let mut doc = Doc::new();
        assert_eq!(doc[0], "Hello World!");
        assert_eq!(doc.length(), 1);
        doc.insert_new_line(0, 5);
        assert_eq!(doc[0], "Hello");
        assert_eq!(doc[1], " World!");
        assert_eq!(doc.length(), 2);
    }
}
