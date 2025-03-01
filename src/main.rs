use fibbot::fib::fibonacci_iterative;
use fibbot::github::post_comment;
use fibbot::reg::extract_numbers_from_text;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <PR_NUMBER> <GITHUB_TOKEN> <MODIFIED_FILES>", args[0]);
        std::process::exit(1);
    }

    let pr_number: u32 = args[1].parse().expect("Invalid PR number");
    let token = &args[2];
    let modified_files = &args[3];

    let mut all_numbers = Vec::new();

    for file_path in modified_files.split(',') {
        if let Ok(content) = fs::read_to_string(file_path) {
            let numbers = extract_numbers_from_text(&content);
            all_numbers.extend(numbers);
        }
    }

    if all_numbers.is_empty() {
        if let Err(e) = post_comment(pr_number, token, "No numbers found in the modified files.").await {
            eprintln!("Error posting comment: {}", e);
        }
        return;
    }

    let fibonacci_results = all_numbers.iter().map(|&num| (num, fibonacci_iterative(num))).collect::<Vec<_>>();

    let comment_body = fibonacci_results.iter()
        .fold(String::from("### Fibonacci Computations:\n"), |mut acc, (num, result)| {
            acc.push_str(&format!("- Fibonacci({}) = {}\n", num, result));
            acc
        });

    if let Err(e) = post_comment(pr_number, token, &comment_body).await {
        eprintln!("Error posting comment: {}", e);
    }
}