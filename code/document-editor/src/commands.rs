use std::cell::RefCell;
use std::rc::Rc;

use crate::console::{Console, MockConsole};
use crate::document::{document, Document};
use mockall::{automock, predicate::eq, Sequence};
use rstest::{fixture, rstest};

/// A command that can be executed by the application.
#[automock]
pub trait Command {
    fn execute(&mut self);
    fn parse(&mut self, _command: &str) {}
}

/// A command to print the document content to the console.
pub struct PrintCommand {
    document: Rc<RefCell<Document>>,
    console: Rc<RefCell<dyn Console>>,
}

impl PrintCommand {
    pub fn new(document: Rc<RefCell<Document>>, console: Rc<RefCell<dyn Console>>) -> PrintCommand {
        PrintCommand { document, console }
    }
}

impl Command for PrintCommand {
    fn execute(&mut self) {
        for line in self.document.borrow().content() {
            self.console.as_ref().borrow_mut().print_line(&line);
        }
    }
}

/// Tests for the PrintCommand
#[fixture]
fn mock_console() -> Rc<RefCell<MockConsole>> {
    let mock = Rc::new(RefCell::new(MockConsole::new()));
    mock
}

#[rstest]
fn print_command_prints_document_on_console(
    document: Document,
    mock_console: Rc<RefCell<MockConsole>>,
) {
    {
        let mut console = mock_console.as_ref().borrow_mut();

        let mut seq = Sequence::new();
        console
            .expect_print_line()
            .with(eq("Line1"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());
        console
            .expect_print_line()
            .with(eq("Line2"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());
        console
            .expect_print_line()
            .with(eq("Line3"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| ());
    }

    let mut print_cmd = PrintCommand::new(Rc::new(RefCell::new(document)), mock_console.clone());

    print_cmd.execute();
}

pub struct AddTextCommand {
    document: Rc<RefCell<Document>>,
    text: Option<String>,
}

impl AddTextCommand {
    pub fn new(document: Rc<RefCell<Document>>) -> AddTextCommand {
        AddTextCommand {
            document,
            text: None,
        }
    }

    pub fn with_text(mut self, text: &str) -> AddTextCommand {
        self.text = Some(text.to_string());
        self
    }
}

impl Command for AddTextCommand {
    fn execute(&mut self) {
        if let Some(text) = &self.text {
            self.document.borrow_mut().add_line(text.clone());
        }
    }

    fn parse(&mut self, command: &str) {
        let parts: Vec<&str> = command.splitn(2, ' ').collect();
        if parts.len() == 2 {
            self.text = Some(parts[1].to_string());
        }
    }
}

#[rstest]
fn add_text_command_adds_text_to_document(document: Document) {
    let doc = Rc::new(RefCell::new(document));
    let mut add_text_cmd = AddTextCommand::new(doc.clone()).with_text("Hello, world!");

    add_text_cmd.execute();

    assert_eq!(
        doc.as_ref().borrow().content(),
        vec!["Line1", "Line2", "Line3", "Hello, world!"]
    );
}

#[rstest]
fn parsing_add_text_command_arguments() {
    let mut add_text_cmd = AddTextCommand::new(Rc::new(RefCell::new(Document::new())));

    add_text_cmd.parse("AddText Hello, world!");

    assert_eq!(add_text_cmd.text, Some("Hello, world!".to_string()));
}
