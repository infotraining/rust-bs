//extern crate primes_lib;
use primes_lib::primes::all_primes;

fn main() {
    for prime in all_primes().take(100) {
        println!("Prime: {}", prime);
    }
}
