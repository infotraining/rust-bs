use numbers::primes::*;
use numbers::primes::generators::primes_sequence;

#[test]
fn test_is_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
}   

#[test]
fn test_n_primes() {
    let first_5_primes = n_primes(5);
    assert_eq!(first_5_primes, vec![2, 3, 5, 7, 11]);
}

#[test]
fn test_primes_in_range() {
    let primes = primes_in_range(10..30);
    assert_eq!(primes, vec![11, 13, 17, 19, 23, 29]);
}

#[test]
fn test_primes_sequence() {
    let primes: Vec<u32> = primes_sequence().take(10).collect();
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}