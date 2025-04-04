use reqwest::Client;
use std::env;

pub async fn post_comment(pr_number: u32, token: &str, comment: &str) -> Result<(), reqwest::Error> {
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let url = format!("https://api.github.com/repos/{}/issues/{}/comments", repo, pr_number);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .json(&serde_json::json!({ "body": comment }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Comment posted successfully.");
    } else {
        eprintln!("❌ Failed to post comment: {:?}", response.text().await?);
    }
    Ok(())
}