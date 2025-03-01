use crate::{ fibonacci::fibonacci, get_pull_request::get_pr,
    post_comment_to_github::post_comment
};

use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let enable_fib = args.get(1).unwrap_or(&"true".to_string()).to_lowercase() == "true";
    let max_threshold: u8 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);

    println!("FibBot application is running...");
    println!("Fibonacci Calculation Enabled: {}", enable_fib);
    println!("Max Threshold is: {}", max_threshold);

    let pr_number: u64 = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u64>()
        .expect("Invalid PR_NUMBER");

    let pr_numbers = get_pr(pr_number).await;
    println!("Extracted numbers: {:?}", pr_numbers);

    if pr_numbers.is_empty() {
        println!("No numbers found in this pull_request.");
    }
    let mut response =
        String::from("#### Fibonacci output of each number in the pull_request is:\n");
    for &num in &pr_numbers {
        let fib = fibonacci(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }
        if let Err(e) = post_comment(&response).await {
            eprintln!("Error posting comment: {}", e);
        }
    }

mod extract_text;
mod fibonacci;
mod get_pull_request;
mod post_comment_to_github;
mod test;
