use serde::{Serialize, Deserialize};

pub type GithubReleases = Vec<GithubRelease>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubRelease {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: i64,
    author: Author,
    node_id: String,
    tag_name: String,
    target_commitish: TargetCommitish,
    pub name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    pub assets: Vec<Asset>,
    tarball_url: String,
    zipball_url: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    url: String,
    id: i64,
    node_id: String,
    pub name: String,
    label: Option<String>,
    uploader: Author,
    content_type: ContentType,
    state: State,
    size: i64,
    pub download_count: i64,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    author_type: Type,
    site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "application/octet-stream")]
    ApplicationOctetStream,
    #[serde(rename = "application/zip")]
    ApplicationZip,
    #[serde(rename = "text/yaml")]
    TextYaml,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "uploaded")]
    Uploaded,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    User,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum TargetCommitish {
    #[serde(rename = "master")]
    Master,
}
