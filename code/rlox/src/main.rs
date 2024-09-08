fn main() {
    use rlox::scanner::Scanner;

    let source = "var text = \"Text\";\r\nvar a = 4 * 3.14;";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();


    println!("Source:\n{}", source);
    println!("-------------------\nTokens:\n");
    println!("{:#?}", tokens);
}
