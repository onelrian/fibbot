use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let enable_fib = args.get(1).map_or(false, |v| v == "true");
    let max_threshold: u32 = args
        .get(2)
        .unwrap_or(&"10".to_string()) // Default value
        .parse()
        .unwrap_or(10); // Fallback value in case of error

    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    if enable_fib {
        println!("Fibonacci computation enabled up to {}", max_threshold);
    } else {
        println!("Fibonacci computation disabled.");
    }
}
