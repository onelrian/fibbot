#[cfg(test)]
mod tests {
    use crate::fibonacci::fibonacci;
      

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(89), 1779979416004714189);
        assert_eq!(fibonacci(99), 218922995834555169026);
    }

}
