use dotenv::dotenv;
use fibbot::{fib::fibonacci, get_pr::get_pr, process_pr_results::post_comment};
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "Usage: {} <enable_fib> <max_threshold> <pr_number>",
            args[0]
        );
        std::process::exit(1);
    }

    let enable_fib = args[1].to_lowercase() == "true";

    let max_threshold: u8 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid max_threshold. Using default value: 100");
        100
    });

    let pr_number: u64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid PR_NUMBER. Exiting.");
        std::process::exit(1);
    });

    let github_token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| {
        eprintln!("GITHUB_TOKEN not set. Please provide a valid GitHub token.");
        std::process::exit(1);
    });

    println!("FibBot application is running...");
    println!("Fibonacci Calculation Enabled: {}", enable_fib);
    println!("Max Threshold is: {}", max_threshold);
    println!("PR Number: {}", pr_number);

    let pr_numbers = get_pr(pr_number).await;
    println!("Extracted numbers: {:?}", pr_numbers);

    if pr_numbers.is_empty() {
        println!("No numbers found in this pull request.");
        return;
    }

    let mut response = String::from("#### Fibonacci output of each number in the pull request:\n");
    for &num in &pr_numbers {
        if enable_fib && num <= max_threshold as u32 {
            let fib = fibonacci(num);
            response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
        }
    }

    if let Err(e) = post_comment(pr_number, &github_token, &response).await {
        eprintln!("Error posting comment: {}", e);
    }
}
