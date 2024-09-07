fn main() {
    use rlox::scanner::Scanner;

    let source = "(){},.-+*;";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    println!("{:#?}", tokens);
}
