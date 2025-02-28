use std::env;
use fibbot::reg::extract_numbers_from_text;
use fibbot::fib::{fibonacci_iterative};
use fibbot::github::post_comment;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // GitHub PR URL and token (passed as arguments)
    let pr_url = args.get(1).expect("PR URL is required");
    let token = args.get(2).expect("GitHub token is required");

    let pr_content = "We need to calculate Fibonacci for 10, 15, and 20."; // Simulate PR content

    // Extract numbers from PR content
    let numbers = extract_numbers_from_text(pr_content);
    println!("Numbers extracted from PR: {:?}", numbers); 

    // Calculate Fibonacci values for extracted numbers
    let mut fibonacci_results = Vec::new();
    for &num in &numbers {
        let result = fibonacci_iterative(num);
        fibonacci_results.push((num, result));
    }

    // Create a result comment
    let mut comment_body = String::from("Fibonacci Computations:\n");
    for (num, result) in fibonacci_results {
        comment_body.push_str(&format!("Fibonacci of {} is: {}\n", num, result));
    }

    // Post the result as a comment on the PR
    if let Err(e) = post_comment(pr_url, token, &comment_body).await {
        eprintln!("Error posting comment: {}", e);
    }
}
