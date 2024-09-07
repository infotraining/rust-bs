fn main() {
    use rlox::scanner::Scanner;

    let source = "text = \"Text\"; a = 4 * 3.14;";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();

    println!("{:#?}", tokens);
}
