use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub settings: SettingsSource,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsSource {
    pub sources: Vec<SettingsRepo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsRepo {
    pub repo: SettingsRepoConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsRepoConfig {
    pub username: String,
    pub name: String,
}
