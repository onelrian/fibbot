use octocrab::{Octocrab, models::pulls::PullRequest};
use std::env;

pub async fn get_pr(pr_number: u64) -> Result<String, Box<dyn std::error::Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(env::var("GITHUB_TOKEN")?)
        .build()?;

    let pr: PullRequest = octocrab.pulls("Nkwenti-severian-Ndongtsop", "Fibonacci-bot")
        .get(pr_number)
        .await?;

    Ok(pr.body.unwrap_or_default())  // Assuming the PR body contains the numbers to extract
}
