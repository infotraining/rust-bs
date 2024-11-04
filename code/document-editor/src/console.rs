use mockall::automock;

#[automock]
pub trait Console {
    fn read_line(&mut self) -> String;
    fn print_line(&mut self, line: &str);
}