#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use num_bigint::BigUint;
    use tickets::{fib::fibonacci, reg::extract_numbers};

    

    #[test]
    fn test_extract_numbers() {
        assert_eq!(
            extract_numbers("The numbers are 42, 13, and 7.", 100),
            vec![42, 13, 7]
        );
        assert_eq!(extract_numbers("100abc200def300", 250), vec![100, 200]);
        assert_eq!(extract_numbers("   99  ", 100), vec![99]);
        assert_eq!(
            extract_numbers("1.5 is not an integer, but 2 is.", 10),
            vec![1, 5, 2]
        );
        assert_eq!(
            extract_numbers("Multiple 007 numbers 0042 here.", 50),
            vec![7, 42]
        );
    }

    #[test]
    fn test_fibonacci() {
        let mut memo = HashMap::new();
        assert_eq!(fibonacci(0, &mut memo), BigUint::from(0u32));
        assert_eq!(fibonacci(1, &mut memo), BigUint::from(1u32));
        assert_eq!(fibonacci(2, &mut memo), BigUint::from(1u32));
        assert_eq!(fibonacci(3, &mut memo), BigUint::from(2u32));
        assert_eq!(fibonacci(10, &mut memo), BigUint::from(55u32));
    }
}