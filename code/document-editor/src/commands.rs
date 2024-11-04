use mockall::{automock, predicate::eq, Sequence};
use rstest::rstest;
use crate::console::{Console, MockConsole};
use crate::document::{document, Document};

/// A command that can be executed by the application.
#[automock]
pub trait Command {
    fn execute(&mut self);
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
