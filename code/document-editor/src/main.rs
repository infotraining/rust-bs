mod document;
mod application;
mod commands;
mod console;


use std::{cell::RefCell, io::{BufRead, Write}, rc::Rc};

use application::ApplicationBuilder;
use document::Document;

struct Terminal{}

impl console::Console for Terminal {
    fn read_line(&mut self) -> String {
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line);
        line.trim().to_string()
    }

    fn print_line(&mut self, line: &str) {
        // std::io::stdout().lock().write_all(line.as_bytes());
        // std::io::stdout().lock().flush();
        println!("{}", line);
    }
}

fn main() {
    let mut terminal = Terminal{};
    let mut doc = Document::new();
    let mut app = ApplicationBuilder::new()
        .with_document(&mut doc)
        .with_console(&mut terminal)
        .build();

    let print_command = commands::PrintCommand::new(&doc, &mut terminal);
    app.add_command("Print".to_string(), Rc::new(RefCell::new(print_command)));
    
    let add_text_command = commands::AddTextCommand::new(&mut doc);
    app.add_command("AddText".to_string(), Rc::new(RefCell::new(add_text_command)));

    app.run();
}
