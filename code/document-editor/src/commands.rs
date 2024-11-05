use std::cell::RefCell;
use std::rc::Rc;
use std::string::ParseError;

use crate::console::{Console, MockConsole};
use crate::document::{document, Document};
use mockall::{automock, predicate::eq, Sequence};
use rstest::{fixture, rstest};

/// A command that can be executed by the application.
#[automock]
pub trait Command {
    fn execute(&mut self);
    fn parse(&mut self, _command: &str) -> Result<(), CommandParseError>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommandParseError {
    pub message: String,
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

    fn parse(&mut self, command: &str) -> Result<(), CommandParseError> {
        match command {
            "Print" => Ok(()),
            _ => Err(CommandParseError {
                message: format!("Unknown command: {}", command),
            }),
        }
    }
}

/// Tests for the PrintCommand

#[cfg(test)]
mod tests_print_command {
    use crate::commands::{Command, CommandParseError, PrintCommand};
    use crate::console::MockConsole;
    use crate::document::{document, Document};
    use mockall::{predicate::eq, Sequence};
    use rstest::{fixture, rstest};
    use std::{cell::RefCell, rc::Rc};

    #[fixture]
    fn mock_console() -> Rc<RefCell<MockConsole>> {
        let mock = Rc::new(RefCell::new(MockConsole::new()));
        mock
    }

    #[rstest]
    fn execute_prints_document_on_console(
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

        let mut print_cmd =
            PrintCommand::new(Rc::new(RefCell::new(document)), mock_console.clone());

        print_cmd.execute();
    }

    #[rstest]
    fn parsing_correct_arguments() {
        let mut print_cmd = PrintCommand::new(
            Rc::new(RefCell::new(Document::new())),
            Rc::new(RefCell::new(MockConsole::new())),
        );

        assert_eq!(print_cmd.parse("Print"), Ok(()));
    }

    #[rstest]
    fn parsing_incorrect_arguments() {
        let mut print_cmd = PrintCommand::new(
            Rc::new(RefCell::new(Document::new())),
            Rc::new(RefCell::new(MockConsole::new())),
        );

        assert_eq!(
            print_cmd.parse("Print Invalid"),
            Err(CommandParseError {
                message: "Unknown command: Print Invalid".to_string()
            })
        );
    }
}

/// AddTextCommand adds a line of text at the end of document.
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

    fn parse(&mut self, command: &str) -> Result<(), CommandParseError> {
        let parts: Vec<&str> = command.splitn(2, ' ').collect();
        if parts.len() == 2 {
            self.text = Some(parts[1].to_string());
            Ok(())
        } else {
            Err(CommandParseError {
                message: "Invalid command format".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests_add_text_command {
    use crate::commands::{AddTextCommand, Command};
    use crate::document::{document, Document};
    use rstest::rstest;
    use std::{cell::RefCell, rc::Rc};

    #[rstest]
    fn execute_adds_line_at_the_end_of_document(document: Document) {
        let doc = Rc::new(RefCell::new(document));
        let mut add_text_cmd = AddTextCommand::new(doc.clone()).with_text("Hello, world!");

        add_text_cmd.execute();

        assert_eq!(
            doc.as_ref().borrow().content(),
            vec!["Line1", "Line2", "Line3", "Hello, world!"]
        );
    }

    #[rstest]
    fn parsing_correct_arguments() {
        let mut add_text_cmd = AddTextCommand::new(Rc::new(RefCell::new(Document::new())));

        add_text_cmd.parse("AddText Hello, world!");

        assert_eq!(add_text_cmd.text, Some("Hello, world!".to_string()));
    }

    #[rstest]
    fn parsing_incorrect_arguments() {
        let mut add_text_cmd = AddTextCommand::new(Rc::new(RefCell::new(Document::new())));

        assert_eq!(
            add_text_cmd.parse("AddText"),
            Err(crate::commands::CommandParseError {
                message: "Invalid command format".to_string()
            })
        );
    }
}

pub struct ReplaceTextCommand {
    document: Rc<RefCell<Document>>,
    old_text: Option<String>,
    new_text: Option<String>,
}

impl ReplaceTextCommand {
    pub fn new(document: Rc<RefCell<Document>>) -> ReplaceTextCommand {
        ReplaceTextCommand {
            document,
            old_text: None,
            new_text: None,
        }
    }

    pub fn with_args(mut self, old_text: &str, new_text: &str) -> ReplaceTextCommand {
        self.old_text = Some(old_text.to_string());
        self.new_text = Some(new_text.to_string());
        self
    }
}

impl Command for ReplaceTextCommand {
    fn execute(&mut self) {
        let old_text = self.old_text.as_ref().unwrap();
        let new_text = self.new_text.as_ref().unwrap();

        let mut doc = self.document.borrow_mut();
        doc.replace_text(old_text, new_text);
    }

    fn parse(&mut self, _command: &str) -> Result<(), CommandParseError> {
        let parts: Vec<&str> = _command.splitn(3, ' ').collect();

        if parts.len() == 3 {
            self.old_text = Some(parts[1].to_string());
            self.new_text = Some(parts[2].to_string());
            return Ok(());
        }

        Err(CommandParseError {
            message: "Invalid command format".to_string(),
        })
    }
}

#[cfg(test)]
mod tests_replace_text_command {
    use crate::commands::{ReplaceTextCommand, Command};
    use crate::document::{document, Document};
    use rstest::rstest;
    use std::{cell::RefCell, rc::Rc};

    #[rstest]
    fn execute_replaces_text_fragment_in_document(document: Document) {
        let doc = Rc::new(RefCell::new(document));
        let mut replace_text_cmd =
            ReplaceTextCommand::new(doc.clone()).with_args("Line2", "Replaced");

        replace_text_cmd.execute();

        assert_eq!(
            doc.as_ref().borrow().content(),
            vec!["Line1", "Replaced", "Line3"]
        );
    }

    #[rstest]
    fn parsing_correct_arguments() {
        let mut replace_text_cmd = ReplaceTextCommand::new(Rc::new(RefCell::new(Document::new())));

        replace_text_cmd.parse("ReplaceText Line2 Replaced");

        assert_eq!(replace_text_cmd.old_text, Some("Line2".to_string()));
        assert_eq!(replace_text_cmd.new_text, Some("Replaced".to_string()));
    }

    #[rstest]
    fn parsing_incorrect_arguments() {
        let mut replace_text_cmd = ReplaceTextCommand::new(Rc::new(RefCell::new(Document::new())));

        assert_eq!(
            replace_text_cmd.parse("ReplaceText"),
            Err(crate::commands::CommandParseError {
                message: "Invalid command format".to_string()
            })
        );
    }
}
