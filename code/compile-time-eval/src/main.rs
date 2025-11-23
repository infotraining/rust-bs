const fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

const fn create_fibonacci_sequence<const N: usize>() -> [u32; N] {
    let mut arr = [0; N];
    let mut i = 0;
    while i < N {
        arr[i] = fibonacci(i as u32);
        i += 1;
    }
    arr
}

fn main() {
    const N: u32 = 10;
    const FIB_N: u32 = fibonacci(N);
    println!("The {}th Fibonacci number is {}", N, FIB_N);

    const SMALL_FIBONACCI_SEQUENCE: [u32; 10] = [
        fibonacci(0),
        fibonacci(1),
        fibonacci(2),
        fibonacci(3),
        fibonacci(4),
        fibonacci(5),
        fibonacci(6),
        fibonacci(7),
        fibonacci(8),
        fibonacci(9)
    ];
    
    println!("Fibonacci sequence up to {}: {:?}", N, SMALL_FIBONACCI_SEQUENCE);

    const FIB_7: u32 = FIBONACCI_SEQUENCE[7];
    println!("The 7th Fibonacci number from the sequence is {}", FIB_7);

    const FIBONACCI_SEQUENCE: [u32; 20] = create_fibonacci_sequence::<20>();
    println!("Fibonacci sequence up to 20: {:?}", FIBONACCI_SEQUENCE);
}