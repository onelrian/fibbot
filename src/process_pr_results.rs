use crate::reg::extract_numbers_from_text;
use crate::fib::fibonacci_iterative;

pub fn process_pr_content(pr_content: &str) -> String {
    let numbers = extract_numbers_from_text(pr_content);
    
    if numbers.is_empty() {
        return "No numbers found in the pull request.".to_string();
    }
    
    let mut response = String::from("#### Fibonacci Results:\n");
    for &num in &numbers {
        let fib = fibonacci_iterative(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }

    response
}