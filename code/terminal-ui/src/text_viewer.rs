use std::fs::read_to_string;
use std::io::{self, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};

pub struct Doc {
    lines: Vec<String>,
    file_name: Option<String>,
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

#[derive(Debug, PartialEq)]
pub struct Coordinates {
    x: usize,
    y: usize,
}

impl Into<(usize, usize)> for &Coordinates {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Into<(usize, usize)> for Coordinates {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Box {
    width: usize,
    height: usize,
}

pub struct TextViewer {
    doc: Doc,
    // doc_length: usize,
    cursor_pos: Coordinates,
    terminal_size: Box,
}

pub struct TextViewBuilder {
    file_name: Option<String>,
    cursor_pos: Option<Coordinates>,
    terminal_size: Box,
}

impl TextViewBuilder {
    pub fn new() -> Self {
        Self {
            file_name: None,
            cursor_pos: Some(Coordinates { x: 1, y: 1 }),
            terminal_size: Box {
                width: 80,
                height: 24,
            },
        }
    }

    pub fn with_file_name(mut self, file_name: &str) -> Self {
        self.file_name = Some(file_name.to_string());
        self
    }

    pub fn with_cursor_pos(mut self, x: usize, y: usize) -> Self {
        self.cursor_pos = Some(Coordinates { x, y });
        self
    }

    pub fn with_terminal_size(mut self, width: usize, height: usize) -> Self {
        self.terminal_size = Box { width, height };
        self
    }

    pub fn build(self) -> TextViewer {
        let doc = self
            .file_name
            .map_or(Doc::new(), |fname| Doc::load_from_file(&fname).unwrap());

        let cursor_pos = self.cursor_pos.unwrap_or(Coordinates {
            x: 1,
            y: doc.length(),
        });

        TextViewer {
            doc,
            cursor_pos,
            terminal_size: self.terminal_size,
        }
    }
}

impl TextViewer {
    pub fn show_document(&mut self) {
        let offset_top = 2 as usize;
        let offset_bottom = 3 as usize;

        let pos = &self.cursor_pos;

        let (old_x, old_y) = pos.into();

        print!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            style::Reset
        );
        println!(
            "{}{}{}-- Rusty Text Editor ver. 0.1 --{}",
            termion::cursor::Goto(1, 1),
            color::Bg(color::Black),
            color::Fg(color::White),
            style::Reset
        );

        if self.doc.length() < self.terminal_size.height - offset_top {
            for line_index in 0..self.doc.length() {
                print!(
                    "{}",
                    termion::cursor::Goto(1, (line_index + offset_top) as u16)
                );
                print!("{}", self.doc.lines[line_index]);
            }
        } else {
            if pos.y <= self.terminal_size.height - offset_bottom {
                for line_index in 0..(self.terminal_size.height - offset_bottom) {
                    print!(
                        "{}",
                        termion::cursor::Goto(1, (line_index + offset_top) as u16)
                    );
                    print!("{}", self.doc.lines[line_index]);
                }
            } else {
                let line_index_offset = pos.y - (self.terminal_size.height - offset_bottom);
                for line_index in line_index_offset..pos.y {
                    print!(
                        "{}",
                        termion::cursor::Goto(
                            1,
                            (line_index - line_index_offset + offset_top) as u16
                        )
                    );
                    print!("{}", self.doc.lines[line_index]);
                }
            }
        }

        println!(
            "{}",
            termion::cursor::Goto(0, (self.terminal_size.height - 2) as u16)
        );
        println!(
            "{}-- {} -- Cursor at ({}, {}){}",
            color::Bg(color::Black),
            self.doc.file_name().unwrap_or(&"Untitled".to_string()),
            old_x,
            old_y,
            style::Reset
        );

        self.show_cursor();
    }

    pub fn run(&mut self) {
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let stdin = io::stdin();

        for k in stdin.keys() {
            match k.unwrap() {
                Key::Ctrl('q') => break,
                Key::Up => {
                    self.cursor_up();
                }
                Key::Down => {
                    self.cursor_down();
                }
                Key::Left => {
                    self.cursor_left();
                }
                Key::Right => {
                    self.cursor_right();
                }
                _ => {}
            }
            self.show_document();
            self.show_cursor();
            stdout.flush().unwrap();
        }
    }

    fn show_cursor(&self) {
        // clip position to terminal size
        let clipped_x = self.cursor_pos.x.min(self.terminal_size.width - 3);
        let clipped_y = self.cursor_pos.y.min(self.terminal_size.height - 3);

        println!(
            "{}",
            termion::cursor::Goto(clipped_x as u16, clipped_y as u16)
        );
    }

    fn cursor_up(&mut self) {
        if self.cursor_pos.y > 1 {
            self.cursor_pos.y -= 1;
        }
    }

    fn cursor_down(&mut self) {
        if self.cursor_pos.y < self.doc.length() {
            self.cursor_pos.y += 1;
        }
    }

    fn cursor_left(&mut self) {
        if self.cursor_pos.x > 1 {
            self.cursor_pos.x -= 1;
        }
    }

    fn cursor_right(&mut self) {
        self.cursor_pos.x += 1;
    }
}
