use mockall::{automock, predicate::eq, Sequence};
use rstest::rstest;
use crate::console::{Console, MockConsole};
use crate::document::{document, Document};

/// A command that can be executed by the application.
#[automock]
pub trait Command {
    fn execute(&mut self);
    fn parse(&mut self, _command: &str) {}
}

/// A command to print the document content to the console.
pub struct PrintCommand<'a> {
    document: &'a Document,
    console: &'a mut dyn Console,
}

impl<'a> PrintCommand<'a> {
    pub fn new(document: &'a Document, console: &'a mut dyn Console) -> PrintCommand<'a> {
        PrintCommand { document, console }
    }
}

impl Command for PrintCommand<'_> {
    fn execute(&mut self) {
        for line in self.document.content() {
            self.console.print_line(&line);
        }
    }
}

/// Tests for the PrintCommand
#[rstest]
fn print_command_prints_document_on_console(document: Document) {
    let mut mock_console = MockConsole::new();

    let mut seq = Sequence::new();
    mock_console
        .expect_print_line()
        .with(eq("Line1"))
        .times(1)
        .in_sequence(&mut seq).returning(|_|());
    mock_console
        .expect_print_line()
        .with(eq("Line2"))
        .times(1)
        .in_sequence(&mut seq).returning(|_|());
    mock_console
        .expect_print_line()
        .with(eq("Line3"))
        .times(1)
        .in_sequence(&mut seq).returning(|_|());

    let mut print_cmd = PrintCommand::new(&document, &mut mock_console);

    print_cmd.execute();
}

pub struct AddTextCommand<'a> {
    document: &'a mut Document,
    text: Option<String>,
}

impl<'a> AddTextCommand<'a> {
    pub fn new(document: &'a mut Document) -> AddTextCommand<'a> {
        AddTextCommand { document, text: None }
    }

    pub fn with_text(mut self, text: &str) -> AddTextCommand<'a> {
        self.text = Some(text.to_string());
        self
    }
}

impl Command for AddTextCommand<'_> {
    fn execute(&mut self) {
        if let Some(text) = &self.text {
            self.document.add_line(text.clone());
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
fn add_text_command_adds_text_to_document(mut document: Document) {
    let mut add_text_cmd = AddTextCommand::new(&mut document).with_text("Hello, world!");

    add_text_cmd.execute();

    assert_eq!(document.content(), vec!["Line1", "Line2", "Line3", "Hello, world!"]);
}

#[rstest]
fn parsing_add_text_command_arguments() {
    let mut document = Document::new();
    let mut add_text_cmd = AddTextCommand::new(&mut document);

    add_text_cmd.parse("AddText Hello, world!");

    assert_eq!(add_text_cmd.text, Some("Hello, world!".to_string()));
}