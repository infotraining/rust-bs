/// Crate for various number-related utilities.
/// Includes functions for prime number checking and Fibonacci number generation.
/// Modules:
/// - `primes`: Functions related to prime numbers.
/// - `fibonacci`: Functions related to Fibonacci numbers.
/// # Examples:
/// ```
/// use numbers::prelude::*;
/// assert!(is_prime(13));
/// let mut fib_iter = fibonacci_sequence();
/// assert_eq!(fib_iter.next(), Some(0));
/// assert_eq!(fib_iter.next(), Some(1));
/// assert_eq!(fib_iter.next(), Some(1));
/// assert_eq!(fib_iter.next(), Some(2));
/// ```

pub mod primes;
pub mod fibonacci;

pub mod prelude {
    pub use crate::primes::is_prime;
    pub use crate::primes::generators::primes_sequence;
    pub use crate::fibonacci::generators::fibonacci_sequence;
}