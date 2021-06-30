use reqwest::header::USER_AGENT;
use crate::urls::GITHUB_BASE_URL;

pub async fn get(endpoint: &str) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("{}{}", GITHUB_BASE_URL, endpoint);

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;

    println!("fetching the url: {}", url);

    let re = client.get(url).send().await?;

    Ok(re)
}
