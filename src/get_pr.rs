use crate::reg::extract_numbers;

pub async fn get_pr(pr_number: u64) -> Vec<u32> {
    let files = octocrab::instance()
        .pulls("onelrian", "fibbot")
        .list_files(pr_number)
        .await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}", files);
    let nums = extract_numbers(&files.as_str().to_string());
    println!("Collected Nums: {nums:?}");
    nums
}
