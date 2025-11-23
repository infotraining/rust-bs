pub mod primes;
pub mod fibonacci;

pub mod prelude {
    pub use crate::primes::is_prime;
    pub use crate::primes::generators::primes_sequence;
    pub use crate::fibonacci::generators::fibonacci_sequence;
}