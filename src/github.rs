use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
pub struct Comment {
    pub body: String,
}

pub async fn post_comment(pr_url: &str, token: &str, comment: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let api_url = format!("{}/comments", pr_url);
    let res = client
        .post(&api_url)
        .bearer_auth(token)
        .json(&Comment { body: comment.to_string() })
        .send()
        .await?;

    if res.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        let error_message = res.text().await?;
        eprintln!("Failed to post comment: {}", error_message);
    }

    Ok(())
}