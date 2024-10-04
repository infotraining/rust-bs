use std::{cell::RefCell, rc::Rc};

use rlox::{interpreter, parser};

struct TerminalConsole {}

impl interpreter::Console for TerminalConsole {
    fn write(&mut self, value: &str) {
        println!("{}", value);
    }
}

fn main() {
    let source = r#"print 2 * 3;
                          print (9 / 3) * ((5 - 2) / 2);
                          print "Hello" + " " + "World";"#;
    
    let mut parser = parser::Parser::new(source);
    let statements = parser.parse_source();

    let terminal = Rc::new(RefCell::new(TerminalConsole {}));
    let mut interpreter = interpreter::Interpreter::new(terminal);
    
    interpreter.interpret_statements(&statements);
}
