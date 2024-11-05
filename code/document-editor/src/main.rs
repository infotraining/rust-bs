mod application;
mod commands;
mod console;
mod document;

use std::{cell::RefCell, io::BufRead, rc::Rc};

use application::ApplicationBuilder;
use document::Document;

struct Terminal {}

impl console::Console for Terminal {
    fn read_line(&mut self) -> String {
        let mut line = String::new();
        let _ = std::io::stdin().lock().read_line(&mut line);
        line.trim().to_string()
    }

    fn print_line(&mut self, line: &str) {
        println!("{}", line);
    }
}

fn main() {
    let terminal = Rc::new(RefCell::new(Terminal {}));
    let doc = Rc::new(RefCell::new(Document::new()));
    {
        let mut app = ApplicationBuilder::new()
            .with_console(terminal.clone())
            .build();

        let print_command = commands::PrintCommand::new(doc.clone(), terminal.clone());
        app.add_command("Print".to_string(), Rc::new(RefCell::new(print_command)));

        let add_text_command = commands::AddTextCommand::new(doc.clone());
        app.add_command(
            "AddText".to_string(),
            Rc::new(RefCell::new(add_text_command)),
        );
        app.add_command(
            "ReplaceText".to_string(),
            Rc::new(RefCell::new(commands::ReplaceTextCommand::new(doc.clone()))),
        );

        app.run();
    }
}
