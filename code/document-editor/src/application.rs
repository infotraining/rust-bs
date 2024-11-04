use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use mockall::predicate::str::contains;
use mockall::{Sequence, predicate::eq};
use rstest::{fixture, rstest};

use crate::document::Document;
use crate::console::{Console, MockConsole};
use crate::commands::{Command, MockCommand};

pub struct Application<'a> {
    document: &'a mut Document,
    console: &'a mut dyn Console,
    commands: HashMap<String, Rc<RefCell<dyn Command>>>,
}

impl<'a> Application<'a> {
    pub fn run(&mut self) {
        loop {
            self.console.print_line("Enter a command: ");

            let line = self.console.read_line();
            let mut tokens: Vec<_> = line.splitn(2, ' ').collect();
            let command_name = tokens[0].to_string();

            if command_name == "Exit" {
                break;
            }

            if let Some(cmd) = self.commands.get(&command_name) {
                cmd.as_ref().borrow_mut().parse(&line);
                cmd.as_ref().borrow_mut().execute();
            }
            else {
                self.console.print_line(&format!("Unknown command: {}", command_name));
            }
        }
    }

    pub fn add_command(&mut self, command_name: String, command: Rc<RefCell<dyn Command>>) {
        self.commands.insert(command_name, command);
    }
}

pub struct ApplicationBuilder<'a> {
    document: Option<&'a mut Document>,
    console: Option<&'a mut dyn Console>,
}

impl<'a> ApplicationBuilder<'a> {
    pub fn new() -> ApplicationBuilder<'a> {
        ApplicationBuilder {
            document: None,
            console: None,
        }
    }

    pub fn with_document(mut self, document: &'a mut Document) -> ApplicationBuilder<'a> {
        self.document = Some(document);
        self
    }

    pub fn with_console(mut self, console: &'a mut dyn Console) -> ApplicationBuilder<'a> {
        self.console = Some(console);
        self
    }

    pub fn build(self) -> Application<'a> {
        Application {
            document: self.document.unwrap(),
            console: self.console.unwrap(),
            commands: HashMap::new(),
        }
    }
}

use crate::document::document;

#[fixture]
fn mock_console() -> MockConsole {
    let mut mock = MockConsole::new();
    mock.expect_print_line().with(contains("Enter a command: ")).times(1..).returning(|_| ());
    mock
}

#[rstest]
fn application_asks_for_command(mut document: Document, mut mock_console: MockConsole) {
    //let mut mock_console = MockConsole::new();
    mock_console
        .expect_read_line()
        .returning(|| "Exit".to_string())
        .times(1..);

    let mut app = ApplicationBuilder::new()
        .with_document(&mut document)
        .with_console(&mut mock_console)
        .build();
    app.run();
}

#[rstest]
fn application_exit_exits_the_loop(mut document: Document, mut mock_console: MockConsole) {
    let mut seq = Sequence::new();

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "Cmd".to_string())
        .in_sequence(&mut seq);

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "Exit".to_string())
        .in_sequence(&mut seq);

    mock_console.expect_print_line().times(1..).returning(|_| ());

    let mut app = ApplicationBuilder::new()
        .with_document(&mut document)
        .with_console(&mut mock_console)
        .build();

    app.run();
}

#[rstest]
fn application_executes_commands(mut document: Document, mut mock_console: MockConsole) {
    let mut seq = Sequence::new();

    mock_console
        .expect_read_line()
        .times(2)
        .returning(|| "Cmd".to_string())
        .in_sequence(&mut seq);

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "Exit".to_string())
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
            .returning(|| ());

        mock_cmd
            .borrow_mut()
            .expect_parse()
            .with(eq("Cmd"))            
            .returning(|_| ());

        app.add_command("Cmd".to_string(), mock_cmd_rc.clone());
    }

    app.run();
}

#[rstest]
fn application_when_unknown_command_is_entered_message_is_printed(mut document: Document, mut mock_console: MockConsole) {
    let mut seq = Sequence::new();

    let mut seq = Sequence::new();
    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "Unknown".to_string())
        .in_sequence(&mut seq);

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "Exit".to_string())
        .in_sequence(&mut seq);

    mock_console
        .expect_print_line()
        .withf(|line| line.contains("Unknown command: Unknown"))
        .times(1)
        .returning(|_| ());

    let mut app = ApplicationBuilder::new()
        .with_document(&mut document)
        .with_console(&mut mock_console)
        .build();

    app.run();
}

#[rstest]
fn application_parses_a_command_arguments(mut document: Document, mut mock_console: MockConsole) {
    let mut seq = Sequence::new();

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "Cmd arg1 arg2".to_string())
        .in_sequence(&mut seq);

    mock_console
        .expect_read_line()
        .times(1)
        .returning(|| "Exit".to_string())
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
            .expect_parse()
            .with(eq("Cmd arg1 arg2"))
            .times(1)
            .returning(|_| ());

        mock_cmd
            .borrow_mut()
            .expect_execute()
            .times(1)
            .returning(|| ());

        app.add_command("Cmd".to_string(), mock_cmd_rc.clone());
    }

    app.run();
}