#[cfg(test)]
mod tests {

    use crate::extract_text::extract_numbers;

    #[test]
    fn test_extract_numbers() {
        let text = "Numbers: 123, 456 and 789.".to_string();
        assert_eq!(extract_numbers(&text), vec![123, 456, 789]);

        let text = "No numbers here!".to_string();
        assert_eq!(extract_numbers(&text), Vec::<u32>::new());

        let text = "One number at the end 99".to_string();
        assert_eq!(extract_numbers(&text), vec![99]);
    }
}
