/// Checks if a number is prime.
/// Returns true if the number is prime, false otherwise.
/// Examples:
/// ```
/// use numbers::primes::is_prime;
/// assert!(is_prime(7));
/// assert!(!is_prime(10));
/// ```
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }

    true
}


/// Returns a vector containing the first `n` prime numbers.
/// Examples:
/// ```
/// use numbers::primes::n_primes;
/// let primes = n_primes(5);
/// assert_eq!(primes, vec![2, 3, 5, 7, 11]);
/// ``` 
pub fn n_primes(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut i = 2;

    while primes.len() < n as usize {
        if is_prime(i) {
            primes.push(i);
        }
        i += 1;
    }

    primes
}

/// Returns a vector of prime numbers within the specified range.
/// Examples:
/// ```
/// use numbers::primes::primes_in_range;
/// let primes = primes_in_range(10..20);
/// assert_eq!(primes, vec![11, 13, 17, 19]);
/// ``` 
pub fn primes_in_range(range: std::ops::Range<u32>) -> Vec<u32> {
    let mut primes = Vec::new();

    for i in range {
        if is_prime(i) {
            primes.push(i);
        }
    }

    primes
}

pub mod generators {
    use super::is_prime;

    /// Returns an iterator that yields an infinite sequence of prime numbers.
    /// Examples:
    /// ```
    /// use numbers::primes::generators::primes_sequence;
    /// let mut primes_iter = primes_sequence();
    /// assert_eq!(primes_iter.next(), Some(2));
    /// assert_eq!(primes_iter.next(), Some(3));
    /// assert_eq!(primes_iter.next(), Some(5));
    /// ```
    pub fn primes_sequence() -> impl Iterator<Item = u32> {
        (2..).filter(|&n| is_prime(n))
    }
}