use fibbot::fib::fibonacci_iterative;
use fibbot::github::post_comment;
use fibbot::reg::extract_numbers_from_text;
use fibbot::get_pr::get_pr_body;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <PR_NUMBER> <GITHUB_TOKEN>", args[0]);
        std::process::exit(1);
    }

    let pr_number: u32 = args[1].parse().expect("Invalid PR number");
    let token = &args[2];

    let pr_content = match get_pr_body(pr_number, token).await {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error fetching PR content: {}", e);
            std::process::exit(1);
        }
    };

    let numbers = extract_numbers_from_text(&pr_content);
    println!("Numbers extracted from PR: {:?}", numbers);

    let fibonacci_results = numbers.iter().map(|&num| (num, fibonacci_iterative(num))).collect::<Vec<_>>();

    let comment_body = fibonacci_results.iter()
        .fold(String::from("### Fibonacci Computations:\n"), |mut acc, (num, result)| {
            acc.push_str(&format!("- Fibonacci({}) = {}\n", num, result));
            acc
        });

    if let Err(e) = post_comment(pr_number, token, &comment_body).await {
        eprintln!("Error posting comment: {}", e);
    }
}
