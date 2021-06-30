use crate::common::models::settings::{SettingsRepoConfig};
use crate::common::api::api_client::get;
use crate::common::models::github_releases::GithubReleases;

pub async fn fetch_releases(repo: &SettingsRepoConfig) -> Result<GithubReleases, Box<dyn std::error::Error>> {
    let url = format!("repos/{}/{}/releases", repo.username, repo.name);

    let r: GithubReleases = get(&url).await?.json().await?;

    Ok(r)
}
