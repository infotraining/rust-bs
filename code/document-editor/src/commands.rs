use std::cell::RefCell;
use std::default;
use std::rc::Rc;

use crate::console::Console;
use crate::document::{Document, DocumentSnapshot};

use mockall::automock;

/// A command that can be executed by the application.
#[automock]
pub trait Command {
    fn execute(&mut self);
    fn parse(&mut self, _command: &str) -> Result<(), CommandParseError>;
}

/// A command that can be undone.
pub trait ReversibleCommand: Command {
    fn undo(&mut self);
    fn clone(&self) -> Box<dyn ReversibleCommand>;
}

#[derive(Default)]
pub struct CommandHistory {
    pub commands: Vec<Box<dyn ReversibleCommand>>,    
}

impl CommandHistory {
    pub fn new() -> CommandHistory {
        CommandHistory {
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, command: Box<dyn ReversibleCommand>) {
        self.commands.push(command);
    }

    pub fn undo(&mut self) {
        if let Some(mut cmd) = self.commands.pop() {
            cmd.undo();
        }
    }
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
mod test_commands_helpers {
    use std::{cell::RefCell, rc::Rc};
    use rstest::{rstest, fixture};
    use crate::{console::MockConsole, document::Document};

    #[fixture]
    pub(super) fn mock_console() -> Rc<RefCell<MockConsole>> {
        let mock = Rc::new(RefCell::new(MockConsole::new()));
        mock
    }

    #[fixture]
    pub(super) fn document() -> Rc<RefCell<Document>> {
        let mut doc = Document::new();
        doc.add_line("Line1".to_string());
        doc.add_line("Line2".to_string());
        doc.add_line("Line3".to_string());
        Rc::new(RefCell::new(doc))
    }

    #[fixture]
    pub(super) fn command_history() -> Rc<RefCell<crate::commands::CommandHistory>> {
        Rc::new(RefCell::new(crate::commands::CommandHistory::new()))
    }
}

#[cfg(test)]
mod tests_print_command {
    use crate::commands::{Command, CommandParseError, PrintCommand};
    use crate::console::MockConsole;
    use crate::document::{Document};
    use mockall::{predicate::eq, Sequence};
    use rstest::{fixture, rstest};
    use std::{cell::RefCell, rc::Rc};
    use super::test_commands_helpers::{document, mock_console};

    
    #[rstest]
    fn execute_prints_document_on_console(
        document: Rc<RefCell<Document>>,
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
            PrintCommand::new(document.clone(), mock_console.clone());

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
    command_history: Rc<RefCell<CommandHistory>>,
    text: Option<String>,
    snapshot: Option<DocumentSnapshot>,
}

impl AddTextCommand {
    pub fn new(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) -> AddTextCommand {
        AddTextCommand {
            document,
            command_history,
            text: None,
            snapshot: None,
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
            self.snapshot = Some(self.document.borrow().create_snapshot());
            self.command_history.borrow_mut().add(self.clone());
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

impl ReversibleCommand for AddTextCommand {
    fn undo(&mut self) { 
        self.document.borrow_mut().restore_snapshot(self.snapshot.take().unwrap());       
    }

    fn clone(&self) -> Box<dyn ReversibleCommand> {
        Box::new(AddTextCommand {
            document: self.document.clone(),
            command_history: self.command_history.clone(),
            text: self.text.clone(),
            snapshot: self.snapshot.clone(),
        })
    }
}

#[cfg(test)]
mod tests_add_text_command {
    use crate::commands::{AddTextCommand, Command, CommandHistory, ReversibleCommand};
    use crate::document::{Document};
    use super::test_commands_helpers::{document, command_history};
    use rstest::rstest;
    use std::{cell::RefCell, rc::Rc};

    #[rstest]
    fn execute_adds_line_at_the_end_of_document(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) {        
        let mut add_text_cmd = AddTextCommand::new(document.clone(), command_history.clone()).with_text("Hello, world!");

        add_text_cmd.execute();

        assert_eq!(
            document.as_ref().borrow().content(),
            &vec!["Line1", "Line2", "Line3", "Hello, world!"]
        );
    }

    #[rstest]
    fn execute_registers_itself_in_command_history(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) {
        let mut add_text_cmd = AddTextCommand::new(document.clone(), command_history.clone()).with_text("Hello, world!");

        add_text_cmd.execute();

        assert_eq!(command_history.as_ref().borrow().commands.len(), 1);
    }

    #[rstest]
    fn parsing_correct_arguments(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) {
        let mut add_text_cmd = AddTextCommand::new(document.clone(), command_history.clone());

        add_text_cmd.parse("AddText Hello, world!");

        assert_eq!(add_text_cmd.text, Some("Hello, world!".to_string()));
    }

    #[rstest]
    fn parsing_incorrect_arguments(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) {
        let mut add_text_cmd = AddTextCommand::new(document.clone(), command_history.clone());

        assert_eq!(
            add_text_cmd.parse("AddText"),
            Err(crate::commands::CommandParseError {
                message: "Invalid command format".to_string()
            })
        );
    }

    #[rstest]
    fn undo_erases_added_line(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) {        
        let mut add_text_cmd = AddTextCommand::new(document.clone(), command_history.clone()).with_text("Hello, world!");

        add_text_cmd.execute();

        add_text_cmd.undo();

        assert_eq!(document.as_ref().borrow().content(), &vec!["Line1", "Line2", "Line3"]);
    }
}

pub struct ReplaceTextCommand {
    document: Rc<RefCell<Document>>,
    command_history: Rc<RefCell<CommandHistory>>,
    old_text: Option<String>,
    new_text: Option<String>,
}

impl ReplaceTextCommand {
    pub fn new(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) -> ReplaceTextCommand {
        ReplaceTextCommand {
            document,
            command_history,
            old_text: None,
            new_text: None,
        }
    }

    pub(self) fn with_args(mut self, old_text: &str, new_text: &str) -> ReplaceTextCommand {
        self.old_text = Some(old_text.to_string());
        self.new_text = Some(new_text.to_string());
        self
    }
}

impl Command for ReplaceTextCommand {
    fn execute(&mut self) {
        let old_text = self.old_text.as_ref().unwrap();
        let new_text = self.new_text.as_ref().unwrap();

        self.command_history.borrow_mut().add(self.clone());

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

impl ReversibleCommand for ReplaceTextCommand {
    fn undo(&mut self) {
        let old_text = self.old_text.as_ref().unwrap();
        let new_text = self.new_text.as_ref().unwrap();

        let mut doc = self.document.borrow_mut();
        doc.replace_text(new_text, old_text);
    }

    fn clone(&self) -> Box<dyn ReversibleCommand> {
        Box::new(ReplaceTextCommand {
            document: self.document.clone(),
            command_history: self.command_history.clone(),
            old_text: self.old_text.clone(),
            new_text: self.new_text.clone(),
        })
    }
}

#[cfg(test)]
mod tests_replace_text_command {
    use crate::commands::{Command, ReplaceTextCommand, ReversibleCommand};
    use crate::document::{Document};
    use super::test_commands_helpers::{command_history, document};
    use super::CommandHistory;
    use rstest::{fixture, rstest};
    use std::{cell::RefCell, rc::Rc};

    #[fixture]
    fn replace_text_cmd(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) -> ReplaceTextCommand {
        ReplaceTextCommand::new(document.clone(), command_history.clone())
    }

    #[rstest]
    fn execute_replaces_text_fragment_in_document(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<crate::commands::CommandHistory>>) {
        let mut replace_text_cmd = ReplaceTextCommand::new(document.clone(), command_history.clone()).with_args("Line2", "Replaced");

        replace_text_cmd.execute();

        assert_eq!(
            document.as_ref().borrow().content(),
            &vec!["Line1", "Replaced", "Line3"]
        );
    }

    #[rstest]
    fn execute_registers_itself_in_command_history(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) {
        let mut replace_text_cmd = ReplaceTextCommand::new(document.clone(), command_history.clone()).with_args("Line2", "Replaced");

        replace_text_cmd.execute();

        assert_eq!(command_history.as_ref().borrow().commands.len(), 1);
    }

    #[rstest]
    fn parsing_correct_arguments(mut replace_text_cmd: ReplaceTextCommand) {
        replace_text_cmd.parse("ReplaceText Line2 Replaced");

        assert_eq!(replace_text_cmd.old_text, Some("Line2".to_string()));
        assert_eq!(replace_text_cmd.new_text, Some("Replaced".to_string()));
    }

    #[rstest]
    fn parsing_incorrect_arguments(mut replace_text_cmd: ReplaceTextCommand) {
        assert_eq!(
            replace_text_cmd.parse("ReplaceText"),
            Err(crate::commands::CommandParseError {
                message: "Invalid command format".to_string()
            })
        );
    }

    #[rstest]
    fn undo_restores_replaced_text_fragment(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>) {
        let mut replace_text_cmd = ReplaceTextCommand::new(document.clone(), command_history.clone()).with_args("Line2", "Replaced");

        replace_text_cmd.execute();
        assert_eq!(document.as_ref().borrow().content(), &vec!["Line1", "Replaced", "Line3"]);

        replace_text_cmd.undo();

        assert_eq!(document.as_ref().borrow().content(), &vec!["Line1", "Line2", "Line3"]);
    }
}

pub struct UndoCommand {
    command_history: Rc<RefCell<CommandHistory>>,
}

impl UndoCommand {
    pub fn new(command_history: Rc<RefCell<CommandHistory>>) -> UndoCommand {
        UndoCommand { command_history }
    }
}

impl Command for UndoCommand {
    fn execute(&mut self) {
        self.command_history.borrow_mut().undo();
    }

    fn parse(&mut self, command: &str) -> Result<(), CommandParseError> {
        match command {
            "Undo" => Ok(()),
            _ => Err(CommandParseError {
                message: format!("Unknown command: {}", command),
            }),
        }
    }
}

#[cfg(test)]
mod tests_undo_command {
    use std::{cell::RefCell, rc::Rc};
    use mockall::mock;
    use rstest::rstest;
    use super::{test_commands_helpers::command_history, Command, ReversibleCommand, UndoCommand};

    mock! {
        TestCommand {}

        impl Command for TestCommand {
            fn execute(&mut self);
            fn parse(&mut self, command: &str) -> Result<(), crate::commands::CommandParseError>;
        }
        
        impl ReversibleCommand for TestCommand {
            fn undo(&mut self);
            fn clone(&self) -> Box<dyn ReversibleCommand>;
        }
    }

    #[rstest]
    fn execute_pops_command_from_history_and_executes_undo(command_history: Rc<RefCell<crate::commands::CommandHistory>>) {
        let mut cmd = Box::new(MockTestCommand::new());
        cmd.expect_undo().times(1).returning(|| ());
        command_history.borrow_mut().add(cmd);

        let mut undo_cmd = UndoCommand::new(command_history.clone());
        undo_cmd.execute();
        
        assert_eq!(command_history.borrow().commands.len(), 0);
    }

    #[rstest]
    fn when_command_history_is_empty_execute_does_nothing(command_history: Rc<RefCell<crate::commands::CommandHistory>>) {
        let mut undo_cmd = UndoCommand::new(command_history.clone());
        undo_cmd.execute();
    }

}