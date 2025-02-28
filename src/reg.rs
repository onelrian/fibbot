use regex::Regex;

pub fn extract_numbers_from_text(text: &str) -> Vec<i32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(text)
        .map(|mat| mat.as_str().parse().unwrap())
        .collect()
}
