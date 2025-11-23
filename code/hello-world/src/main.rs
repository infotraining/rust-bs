use std::io::Write;

const GREETING: &'static str = "Hello, ";

fn input(prompt: &str) -> String {
    static mut COUNTER: u32 = 0;

    unsafe {
        COUNTER += 1;
        println!("Function called {} times", COUNTER);
    }

    print!("{}: ", prompt);
    std::io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string()
}

fn main() {
    let name = input("Get your name: ");
    let full_greeting = GREETING.to_string() + name.trim() + "!";
    println!("{}", full_greeting);

    let age_str = input("Get your age: ");

    match age_str.parse::<u32>() {
        Ok(age) => println!("You are {} years old.", age),
        Err(_) => println!("Invalid age input."),
    }
}
