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

    pub fn primes_sequence() -> impl Iterator<Item = u32> {
        (2..).filter(|&n| is_prime(n))
    }
}