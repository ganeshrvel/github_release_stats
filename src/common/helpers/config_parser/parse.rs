use std::error::Error;
use std::fs::File;
use crate::common::models::settings;


pub fn repo_source() -> Result<settings::SettingsRepo, Box<dyn Error>> {
    let f = File::open("settings.yaml")?;

    println!("Reading the YAML file:...");

    let data: settings::Settings = serde_yaml::from_reader(f)?;
    let sr = data.settings.sources.first();

    let r = match sr {
        None => {
            panic!("No repo config found")
        }
        Some(d) => d.clone()
    };

    Ok(r)
}
