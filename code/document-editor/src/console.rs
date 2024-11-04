use std::{cell::RefCell, rc::Rc};

use mockall::{automock, predicate::str::contains};
use rstest::fixture;

#[automock]
pub trait Console {
    fn read_line(&mut self) -> String;
    fn print_line(&mut self, line: &str);
}

#[fixture]
pub(crate) fn mock_console() -> Rc<RefCell<MockConsole>> {
    let mock = Rc::new(RefCell::new(MockConsole::new()));
    mock.as_ref()
        .borrow_mut()
        .expect_print_line()
        .with(contains("Enter a command: "))
        .times(1..)
        .returning(|_| ());
    mock
}
