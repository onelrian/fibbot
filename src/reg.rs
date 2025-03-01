use regex::Regex;

pub fn extract_numbers(text: &str, max_threshold: u32) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(text)
        .filter_map(|m| m.as_str().parse().ok())
        .filter(|&num| num <= max_threshold)
        .collect()
}