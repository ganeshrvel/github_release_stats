mod features;
mod common;
mod urls;

use common::helpers::config_parser::parse;
use crate::common::helpers::fetch_request::github::fetch_releases;
use crate::common::models::github_releases::GithubReleases;
use crate::common::models::settings::{SettingsRepoConfig};
use crate::features::tabulation::stats::print_stats;

#[macro_use]
extern crate prettytable;


fn main() {
    let r = parse::repo_source();

    match r {
        Err(e) => {
            eprintln!("Some error occured");
            if let Some(err) = e.downcast_ref::<std::io::Error>() {
                eprintln!("IO error occured: {}", err);
            } else if let Some(err) = e.downcast_ref::<serde_yaml::Error>() {
                eprintln!("Serde yaml parse error occured: {}", err);
            }
        }
        Ok(c) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(start_processing(&c.repo));
        }
    }
}

async fn start_processing(repo: &SettingsRepoConfig) {
    let releases_data = fetch_github_api(repo).await;

    print_stats(repo, releases_data)
}

async fn fetch_github_api(repo: &SettingsRepoConfig) -> GithubReleases {
    let d = fetch_releases(repo).await;

    let releases_data: GithubReleases = match d {
        Err(e) => panic!("api fetch error: {}", e),
        Ok(r) => {
            println!("received data from: {}/{}", repo
                .username, repo.name);

            r
        }
    };

    releases_data
}
