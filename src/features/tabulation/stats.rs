use crate::common::models::github_releases::GithubReleases;
use crate::common::models::settings::SettingsRepoConfig;

use prettytable::{Table};

pub fn print_stats(repo: &SettingsRepoConfig, releases_data: GithubReleases) {
    let latest = match releases_data.first() {
        None => panic!("no github release data found for {}/{}", repo
            .username, repo.name),
        Some(d) => d
    };

    let mut table = Table::new();
    table.add_row(row!["Version:", latest.name]);
    table.add_row(row!["Asset", "Download Count"]);
    for asset in &latest.assets {
        table.add_row(row![asset.name, asset.download_count]);
    }

    table.printstd();
}
