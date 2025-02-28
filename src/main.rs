use fibbot::fib::fibonacci_iterative;
use fibbot::github::post_comment;
use fibbot::reg::extract_numbers_from_text;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <PR_URL> <GITHUB_TOKEN>", args[0]);
        std::process::exit(1);
    }

    let pr_url = &args[1];
    let token = &args[2];

    let pr_content = "We need to calculate Fibonacci for 10, 15, and 20."; // Replace with actual PR content fetching logic

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