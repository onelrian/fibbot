use num_bigint::BigUint;
use std::collections::HashMap;
use tickets::{fib::fibonacci, github::GhAPIClient, reg::extract_numbers};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        panic!("Mismatch required params, exactly two params are required: enable_fib, max_threshold");
    }

    println!("Args: {:?}", args);

    let gh_token = env::var("GITHUB_TOKEN").expect("Could not read GITHUB_TOKEN from env.");
    let gh_repo = env::var("GITHUB_REPOSITORY").expect("Could not read GITHUB_REPOSITORY from env.");
    let pr_number = env::var("PR_NUMBER").expect("Could not read PR_NUMBER from env.");

    let gh_api = GhAPIClient::new(&gh_token, &gh_repo);

    let [_, enable_fib, max_threshold] = args.as_slice() else {
        panic!("Failed to read args");
    };

    let enable_fib = parse_bool(&enable_fib.trim()).expect("Could not parse enable_fib from params");
    let max_threshold = max_threshold
        .trim()
        .parse()
        .expect("Could not parse max_threshold from params");

    let pr_number = pr_number.parse().expect("Could not parse PR number");
    let pr_diff_entry = gh_api.get_pull_request_files(pr_number).await?;

    let mut numbers = Vec::new();

    for item in pr_diff_entry {
        if let Some(text) = item.patch {
            numbers.append(&mut extract_numbers(&text, max_threshold));
        }
    }

    println!("Numbers from PR content: {:?}", numbers);

    if enable_fib {
        let mut memo = HashMap::new();
        let fibonaccies = numbers
            .iter()
            .map(|num| (*num, fibonacci(*num, &mut memo)))
            .collect::<Vec<(u32, BigUint)>>();

        let comment = if fibonaccies.is_empty() {
            format!("Numberless PR: Nothing to Compute...")
        } else {
            format!("Fibonaccies: {:?}", fibonaccies)
        };

        println!("{}", comment);

        gh_api.post_issue_comment(pr_number, &comment).await?;
    } else {
        println!("Fibbot was not enabled.");
    }

    Ok(())
}

fn parse_bool(s: &str) -> Result<bool, &'static str> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err("Invalid boolean value"),
    }
}