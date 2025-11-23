use numbers::prelude::*;

fn main() {
    println!("First 50 prime numbers:");
    for prime in primes_sequence().take(50) {
        println!("Prime: {}", prime);
    }

    println!("-------------------");

    println!("First 20 Fibonacci numbers:");
    for fib in fibonacci_sequence().take(20) {
        println!("Fibonacci: {}", fib);
    }
}
