use crate::reg::extract_numbers_from_text;
use crate::fib::fibonacci_iterative;
use std::fs;

pub fn process_modified_files(file_paths: &str) -> String {
    let mut all_numbers = Vec::new();

    for file_path in file_paths.split(',') {
        if let Ok(content) = fs::read_to_string(file_path) {
            let numbers = extract_numbers_from_text(&content);
            all_numbers.extend(numbers);
        }
    }

    if all_numbers.is_empty() {
        return "No numbers found in the modified files.".to_string();
    }

    let mut response = String::from("#### Fibonacci Results:\n");
    for &num in &all_numbers {
        let fib = fibonacci_iterative(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }

    response
}