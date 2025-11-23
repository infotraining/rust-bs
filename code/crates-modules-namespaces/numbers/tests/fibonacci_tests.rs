use numbers::fibonacci::generators::fibonacci_sequence;
use numbers::fibonacci::numbers::*;


#[test]
fn test_fibonacci_sequence() {
    let fibs: Vec<u32> = fibonacci_sequence().take(10).collect();
    assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}

#[test]
fn test_fibonacci_recursive() {
    assert_eq!(fibonacci_recursive(0), 0);
    assert_eq!(fibonacci_recursive(1), 1);
    assert_eq!(fibonacci_recursive(2), 1);
    assert_eq!(fibonacci_recursive(3), 2);
    assert_eq!(fibonacci_recursive(4), 3);
    assert_eq!(fibonacci_recursive(5), 5);
    assert_eq!(fibonacci_recursive(6), 8);
    assert_eq!(fibonacci_recursive(7), 13);
    assert_eq!(fibonacci_recursive(8), 21);
    assert_eq!(fibonacci_recursive(9), 34);
    assert_eq!(fibonacci_recursive(10), 55);
}

#[test]
fn test_fibonacci_iterative() {
    assert_eq!(fibonacci_iterative(0), 0);
    assert_eq!(fibonacci_iterative(1), 1);
    assert_eq!(fibonacci_iterative(2), 1);
    assert_eq!(fibonacci_iterative(3), 2);
    assert_eq!(fibonacci_iterative(4), 3);
    assert_eq!(fibonacci_iterative(5), 5);
    assert_eq!(fibonacci_iterative(6), 8);
    assert_eq!(fibonacci_iterative(7), 13);
    assert_eq!(fibonacci_iterative(8), 21);
    assert_eq!(fibonacci_iterative(9), 34);
    assert_eq!(fibonacci_iterative(10), 55);
}

