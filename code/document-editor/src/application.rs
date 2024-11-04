use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use mockall::{Sequence};
use rstest::rstest;

use crate::document::Document;
use crate::console::{Console, MockConsole};
use crate::commands::{Command, MockCommand};

struct Application<'a> {
    document: &'a mut Document,
    console: &'a mut dyn Console,
    commands: HashMap<String, Rc<RefCell<dyn Command>>>,
}

impl<'a> Application<'a> {
    fn run(&mut self) {
        loop {
            let command_name = self.console.read_line();
            if command_name == "exit" {
                break;
            }

            if let Some(cmd) = self.commands.get(&command_name) {
                cmd.as_ref().borrow_mut().execute();
            }
        }
    }

    fn add_command(&mut self, command_name: String, command: Rc<RefCell<dyn Command>>) {
        self.commands.insert(command_name, command);
    }
}

struct ApplicationBuilder<'a> {
    document: Option<&'a mut Document>,
    console: Option<&'a mut dyn Console>,
}

impl<'a> ApplicationBuilder<'a> {
    fn new() -> ApplicationBuilder<'a> {
        ApplicationBuilder {
            document: None,
            console: None,
        }
    }

    fn with_document(mut self, document: &'a mut Document) -> ApplicationBuilder<'a> {
        self.document = Some(document);
        self
    }

    fn with_console(mut self, console: &'a mut dyn Console) -> ApplicationBuilder<'a> {
        self.console = Some(console);
        self
    }

    fn build(self) -> Application<'a> {
        Application {
            document: self.document.unwrap(),
            console: self.console.unwrap(),
            commands: HashMap::new(),
        }
    }
}

use crate::document::document;

#[rstest]
fn application_asks_for_command(mut document: Document) {
    let mut mock_console = MockConsole::new();
    mock_console
        .expect_read_line()
        .returning(|| "exit".to_string())
        .times(1..);

    let mut app = ApplicationBuilder::new()
        .with_document(&mut document)
        .with_console(&mut mock_console)
        .build();
    app.run();
}

#[rstest]
fn application_exit_exits_the_loop(mut document: Document) {
    let mut seq = Sequence::new();
    let mut mock_console = MockConsole::new();

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "cmd".to_string())
        .in_sequence(&mut seq);

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "exit".to_string())
        .in_sequence(&mut seq);

    let mut app = ApplicationBuilder::new()
        .with_document(&mut document)
        .with_console(&mut mock_console)
        .build();

    app.run();
}

#[rstest]
fn application_executes_commands(mut document: Document) {
    let mut seq = Sequence::new();
    let mut mock_console = MockConsole::new();

    mock_console
        .expect_read_line()
        .times(2)
        .returning(|| "cmd".to_string())
        .in_sequence(&mut seq);

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "exit".to_string())
        .in_sequence(&mut seq);

    let mut app = ApplicationBuilder::new()
        .with_document(&mut document)
        .with_console(&mut mock_console)
        .build();

    {
        let mock_cmd_rc: Rc<RefCell<MockCommand>> = Rc::new(RefCell::new(MockCommand::new()));
        let mut mock_cmd = mock_cmd_rc.as_ref().borrow_mut();

        mock_cmd
            .borrow_mut()
            .expect_execute()
            .times(2)
            .returning(|| ());

        app.add_command("cmd".to_string(), mock_cmd_rc.clone());
    }

    app.run();
}
