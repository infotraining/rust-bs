pub fn fibonacci_sequence() -> impl Iterator<Item = u32> {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;
            let current = self.curr;
            self.curr = self.next;
            self.next = new_next;
            Some(current)
        }
    }

    Fibonacci { curr: 0, next: 1 }
}