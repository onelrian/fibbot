#[cfg(test)]
mod tests {
    use fibbot::{fib, reg};

    #[test]
    fn test_fibonacci_iterative() {
        assert_eq!(fib::fibonacci_iterative(0), 0);
        assert_eq!(fib::fibonacci_iterative(1), 1);
        assert_eq!(fib::fibonacci_iterative(2), 1);
        assert_eq!(fib::fibonacci_iterative(3), 2);
        assert_eq!(fib::fibonacci_iterative(4), 3);
        assert_eq!(fib::fibonacci_iterative(5), 5);
        assert_eq!(fib::fibonacci_iterative(6), 8);
    }

    #[test]
    fn test_extract_numbers_from_text() {
        let text = "We need to calculate Fibonacci for 10 and 15.";
        let numbers = reg::extract_numbers_from_text(text);
        assert_eq!(numbers, vec![10, 15]);
    }
}
