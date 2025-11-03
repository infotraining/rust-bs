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
}