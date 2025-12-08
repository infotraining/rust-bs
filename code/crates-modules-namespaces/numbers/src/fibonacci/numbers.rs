/// Returns the n-th Fibonacci number using a recursive approach.
/// Examples:
/// ```
/// use numbers::fibonacci::fibonacci_recursive;
/// assert_eq!(fibonacci_recursive(10), 55);
/// ```
pub fn fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// Returns the n-th Fibonacci number using an iterative approach.
/// Examples:
/// ```
/// use numbers::fibonacci::fibonacci_iterative;
/// assert_eq!(fibonacci_iterative(10), 55);
/// ```
pub fn fibonacci_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}