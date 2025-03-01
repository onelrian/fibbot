use std::env;

use reqwest::Client;
// use serde_json::json;

pub async fn post_comment(pr_number: u64, github_token: &str, pr_content: &str) -> Result<(), reqwest::Error> {
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .json(&serde_json::json!({ "body": pr_content }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Comment posted successfully.");
    } else {
        eprintln!(
            "❌ Failed to post comment. Status: {}, Body: {:?}",
            response.status(),
            response.text().await?
        );
    }

    Ok(())
}