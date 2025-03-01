#[cfg(test)]
mod tests {
    use fibbot::{fib::fibonacci, reg::extract_numbers};

    #[test]
    fn test_extract_numbers() {
        assert_eq!(
            extract_numbers("The numbers are 42, 13, and 7."),
            vec![42, 13, 7]
        );
        assert_eq!(extract_numbers("100abc200def300"), vec![100, 200, 300]);
        assert_eq!(extract_numbers("   99  "), vec![99]);
        assert_eq!(
            extract_numbers("1.5 is not an integer, but 2 is."),
            vec![1, 5, 2]
        );
        assert_eq!(
            extract_numbers("Multiple 007 numbers 0042 here."),
            vec![7, 42]
        );
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }
}
