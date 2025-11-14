use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Document {
    content: String,
}

impl Document {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn with_content(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn content(&self) -> &str {
        &self.content
    }
}

trait Command: Any {
    fn execute(&mut self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

trait ReversibleCommand: Command {
    fn undo(&mut self);

    fn clone(&self) -> Box<dyn ReversibleCommand>;
}

struct CommandHistory {
    history: Vec<Box<dyn ReversibleCommand>>,
}

impl CommandHistory {
    fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    fn push(&mut self, cmd: Box<dyn ReversibleCommand>) {
        self.history.push(cmd);
    }

    fn pop(&mut self) -> Option<Box<dyn ReversibleCommand>> {
        self.history.pop()
    }
}

#[derive(Debug)]
struct PrintCommand {
    document: Rc<RefCell<Document>>,
}

impl PrintCommand {
    fn new(document: Rc<RefCell<Document>>) -> Self {
        Self { document }
    }
}

impl Command for PrintCommand {
    fn execute(&mut self) {
        let doc = self.document.borrow();
        println!("Document content: {}", doc.content());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct PrettyPrintCommand {
    document: Rc<RefCell<Document>>,
}

impl PrettyPrintCommand {
    fn new(document: Rc<RefCell<Document>>) -> Self {
        Self { document }
    }
}

impl Command for PrettyPrintCommand {
    fn execute(&mut self) {
        let doc = self.document.borrow();
        println!("{}", "-".repeat(doc.content().len() + 4));
        println!("| {} |", doc.content());
        println!("{}", "-".repeat(doc.content().len() + 4));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
struct AddTextCommand {
    document: Rc<RefCell<Document>>,
    command_history: Rc<RefCell<CommandHistory>>,
    text: String,
    prev_length: Option<usize>,
}

impl AddTextCommand {
    fn new(document: Rc<RefCell<Document>>, command_history: Rc<RefCell<CommandHistory>>, text: &str) -> Self {
        Self {
            document,
            command_history,
            text: text.to_string(),
            prev_length: None,
        }
    }
}

impl Command for AddTextCommand {
    fn execute(&mut self) {
        self.prev_length = Some(self.document.borrow().content().len());
        self.command_history.borrow_mut().push(ReversibleCommand::clone(self));
        self.document.borrow_mut().add_text(&self.text);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ReversibleCommand for AddTextCommand {
    fn undo(&mut self) {
        println!("Undoing AddTextCommand for text: {}", self.document.borrow().content());
        let mut doc = self.document.borrow_mut();
        if let Some(length) = self.prev_length {
            doc.content.truncate(length);
        }
    }

    fn clone(&self) -> Box<dyn ReversibleCommand> {
        Box::new(Clone::clone(self))
    }
}

fn downcasting_commands() {
    let document = Rc::new(RefCell::new(Document::with_content("Demo Document")));

    let cmd_print: Box<dyn Command> = Box::new(PrintCommand::new(Rc::clone(&document)));
    let cmd_pretty_print: Box<dyn Command> = Box::new(PrettyPrintCommand::new(document.clone()));

    let mut commands: HashMap<_, _> = vec![("print", cmd_print), ("pretty_print", cmd_pretty_print)].into_iter().collect();

    let cmd: &mut Box<dyn Command> = commands.get_mut("pretty_print").unwrap();
    cmd.execute();

    // downcast to PrettyPrintCommand
    if let Some(pretty_cmd) = cmd.as_any_mut().downcast_mut::<PrettyPrintCommand>() {
        println!("Downcasted to PrettyPrintCommand successfully.");
        pretty_cmd.execute();
    } else {
        println!("Failed to downcast to PrettyPrintCommand.");
    }   
}

fn upcasting_commands() {
    let document = Rc::new(RefCell::new(Document::new()));
    let command_history = Rc::new(RefCell::new(CommandHistory::new()));

    // First with &mut dyn ReversibleCommand
    let mut rev_cmd_add_text = AddTextCommand::new(document.clone(), command_history.clone(), "Some content...");

    let rev_cmd_trait_object: &mut dyn ReversibleCommand = &mut rev_cmd_add_text;
    let cmd_trait_object: &mut dyn Command = rev_cmd_trait_object;

    cmd_trait_object.execute();
    println!("Document content after execute: {:?}", document.borrow().content());
    
    rev_cmd_trait_object.undo();
    println!("Document content after undo: {:?}", document.borrow().content());

    // Now with Box<dyn ReversibleCommand>
    let rev_cmd_add_hello: Box<dyn ReversibleCommand> = Box::new(AddTextCommand::new(document.clone(), command_history.clone(),  "Hello, "));
    let rev_cmd_add_world: Box<dyn ReversibleCommand> = Box::new(AddTextCommand::new(document.clone(), command_history.clone(),"World!"));

    let cmd_add_hello: Box<dyn Command> = rev_cmd_add_hello; // upcasting to Command from ReversibleCommand

    let mut commands = std::collections::HashMap::<&str, Box<dyn Command>>::new();
    commands.insert("add_hello", cmd_add_hello);
    commands.insert("add_world", rev_cmd_add_world);

    commands.get_mut("add_hello").unwrap().execute();
    commands.get_mut("add_world").unwrap().execute();

    println!("Document content after execution: {:?}", document.borrow().content());
}


fn execute_undo_command() {
    let document = Rc::new(RefCell::new(Document::new()));
    let cmd_history = Rc::new(RefCell::new(CommandHistory::new()));

    let add_hello = AddTextCommand::new(document.clone(), cmd_history.clone(), "Hello, ");
    let add_world = AddTextCommand::new(document.clone(), cmd_history.clone(), "World!");

    let mut commands: Vec<Box<dyn Command>> = vec![
        Box::new(add_hello),
        Box::new(add_world),
    ];

    for cmd in commands.iter_mut() {
        cmd.execute();
    }

    println!("Document content after execution: {:?}", document.borrow().content());

    while let Some(mut cmd) = cmd_history.borrow_mut().pop() {
        cmd.undo();
    }

    println!("Document content after undoing all commands: {:?}", document.borrow().content());
}

fn main() {
    downcasting_commands();
    println!("--------------------------");
    upcasting_commands();
    println!("--------------------------");
    execute_undo_command();
}
