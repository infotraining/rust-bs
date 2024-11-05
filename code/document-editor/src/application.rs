use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::commands::Command;
use crate::console::Console;

pub struct Application {
    //document: Rc<RefCell<&'a mut Document>>,
    console: Rc<RefCell<dyn Console>>,
    commands: HashMap<String, Rc<RefCell<dyn Command>>>,
}

impl Application {
    pub fn run(&mut self) {
        loop {
            self.console
                .as_ref()
                .borrow_mut()
                .print_line("Enter a command: ");

            let line = self.console.as_ref().borrow_mut().read_line();
            let tokens: Vec<_> = line.splitn(2, ' ').collect();
            let command_name = tokens[0].to_string();

            if command_name == "Exit" {
                break;
            }

            if let Some(cmd) = self.commands.get(&command_name) {
                cmd.as_ref().borrow_mut().parse(&line);
                cmd.as_ref().borrow_mut().execute();
            } else {
                self.console
                    .as_ref()
                    .borrow_mut()
                    .print_line(&format!("Unknown command: {}", command_name));
            }
        }
    }

    pub fn add_command(&mut self, command_name: String, command: Rc<RefCell<dyn Command>>) {
        self.commands.insert(command_name, command);
    }
}

pub struct ApplicationBuilder {
    console: Option<Rc<RefCell<dyn Console>>>,
}

impl ApplicationBuilder {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            //document: None,
            console: None,
        }
    }

    pub fn with_console(mut self, console: Rc<RefCell<dyn Console>>) -> ApplicationBuilder {
        self.console = Some(console.clone());
        self
    }

    pub fn build(self) -> Application {
        Application {
            //document: self.document.unwrap(),
            console: self.console.unwrap(),
            commands: HashMap::new(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests_application {
    use crate::application::ApplicationBuilder;
    use crate::commands::{Command, MockCommand};    
    use crate::console::{Console, MockConsole, tests_console::mock_console};
    use mockall::{Sequence, predicate::eq};
    use rstest::rstest;
    use std::{cell::RefCell, rc::Rc};


    #[rstest]
    fn run_asks_for_command(mock_console: Rc<RefCell<MockConsole>>) {
        {
            let mut console = mock_console.as_ref().borrow_mut();
    
            console
                .expect_read_line()
                .returning(|| "Exit".to_string())
                .times(1..);
        }
        let mut app = ApplicationBuilder::new()
            .with_console(mock_console.clone())
            .build();
        app.run();
    }
    
    #[rstest]
    fn run_exit_in_console_exits_the_loop(mock_console: Rc<RefCell<MockConsole>>) {
        {
            let mut console = mock_console.as_ref().borrow_mut();
    
            let mut seq = Sequence::new();
            console
                .expect_read_line()
                .times(1)
                .returning(|| "Cmd".to_string())
                .in_sequence(&mut seq);
    
            console
                .expect_read_line()
                .times(1)
                .returning(|| "Exit".to_string())
                .in_sequence(&mut seq);
    
            console.expect_print_line().times(1..).returning(|_| ());
        }
    
        let mut app = ApplicationBuilder::new()
            .with_console(mock_console.clone())
            .build();
    
        app.run();
    }
    
    #[rstest]
    fn run_executes_commands(mock_console: Rc<RefCell<MockConsole>>) {
        {
            let mut console = mock_console.as_ref().borrow_mut();
            let mut seq = Sequence::new();
    
            console
                .expect_read_line()
                .times(2)
                .returning(|| "Cmd".to_string())
                .in_sequence(&mut seq);
    
            console
                .expect_read_line()
                .times(1)
                .returning(|| "Exit".to_string())
                .in_sequence(&mut seq);
        }
    
        let mut app = ApplicationBuilder::new()
            .with_console(mock_console.clone())
            .build();
    
        {
            let mock_cmd_rc: Rc<RefCell<MockCommand>> = Rc::new(RefCell::new(MockCommand::new()));
            let mut mock_cmd = mock_cmd_rc.as_ref().borrow_mut();
    
            mock_cmd.expect_execute().returning(|| ());
    
            mock_cmd                
                .expect_parse()
                .with(eq("Cmd"))
                .returning(|_| Ok(()));
    
            app.add_command("Cmd".to_string(), mock_cmd_rc.clone());
        }
    
        app.run();
    }
    
    #[rstest]
    fn run_unknown_command_prints_message_on_console(
        mock_console: Rc<RefCell<MockConsole>>,
    ) {
        {
            let mut console = mock_console.as_ref().borrow_mut();
    
            let mut seq = Sequence::new();
            console
                .expect_read_line()
                .times(1)
                .returning(|| "Unknown".to_string())
                .in_sequence(&mut seq);
    
            console
                .expect_read_line()
                .times(1)
                .returning(|| "Exit".to_string())
                .in_sequence(&mut seq);
    
            console
                .expect_print_line()
                .withf(|line| line.contains("Unknown command: Unknown"))
                .times(1)
                .returning(|_| ());
        }
    
        let mut app = ApplicationBuilder::new()
            .with_console(mock_console.clone())
            .build();
    
        app.run();
    }
    
    #[rstest]
    fn run_commands_are_parsed_for_arguments(mock_console: Rc<RefCell<MockConsole>>) {
        {
            let mut console = mock_console.as_ref().borrow_mut();
            let mut seq = Sequence::new();
    
            console
                .expect_read_line()
                .times(1)
                .returning(|| "Cmd arg1 arg2".to_string())
                .in_sequence(&mut seq);
    
            console
                .expect_read_line()
                .times(1)
                .returning(|| "Exit".to_string())
                .in_sequence(&mut seq);
        }
    
        let mut app = ApplicationBuilder::new()
            .with_console(mock_console.clone())
            .build();
    
        {
            let mock_cmd_rc: Rc<RefCell<MockCommand>> = Rc::new(RefCell::new(MockCommand::new()));
            let mut mock_cmd = mock_cmd_rc.as_ref().borrow_mut();
    
            mock_cmd
                .expect_parse()
                .with(eq("Cmd arg1 arg2"))
                .times(1)
                .returning(|_| Ok(()));
    
            mock_cmd                
                .expect_execute()
                .times(1)
                .returning(|| ());
    
            app.add_command("Cmd".to_string(), mock_cmd_rc.clone());
        }
    
        app.run();
    }
    
}

