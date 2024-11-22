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

pub fn all_primes() -> impl Iterator<Item = u32> {
    (2..).filter(|&n| is_prime(n))
}

#[test]
fn test_is_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(13));
    assert!(is_prime(17));
    assert!(is_prime(19));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
    assert!(!is_prime(10));
    assert!(is_prime(113));
}

#[test]
fn test_n_primes() {
    let primes = n_primes(0);
    assert_eq!(primes, Vec::new());

    let primes = n_primes(10);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

    let primes = n_primes(20);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);
}

#[test]
fn test_primes_in_range() {
    let primes = primes_in_range(2..100);
    
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 
                            31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 
                            73, 79, 83, 89, 97]);
}

#[test]
fn test_all_primes() {
    let primes = all_primes().take(10).collect::<Vec<u32>>();
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}