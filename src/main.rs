use std::env;
use fibbot::reg::extract_numbers_from_text;

fn main() {
    let pr_content = "We need to calculate Fibonacci for 10 and 15.";

    let numbers = extract_numbers_from_text(pr_content);
    println!("{:?}", numbers);

    let args: Vec<String> = env::args().collect();
    let enable_fib = args.get(1).map_or(false, |v| v == "true");
    let max_threshold: u32 = args
        .get(2)
        .unwrap_or(&"10".to_string())
        .parse()
        .unwrap_or(10);

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    if enable_fib {
        println!("Fibonacci computation enabled up to {}", max_threshold);
    } else {
        println!("Fibonacci computation disabled.");
    }
}
