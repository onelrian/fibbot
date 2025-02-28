use reqwest::Client;

pub async fn get_pr_body(pr_number: u32, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let repo = std::env::var("GITHUB_REPOSITORY")?;
    let url = format!("https://api.github.com/repos/{}/pulls/{}", repo, pr_number);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .bearer_auth(token)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    if let Some(body) = json.get("body") {
        return Ok(body.as_str().unwrap_or("").to_string());
    }

    Err("Failed to get pull request body".into())
}