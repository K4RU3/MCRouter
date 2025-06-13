use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub domains: Vec<DomainConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DomainConfig {
    pub name: String,
    pub forward: String,
}

pub fn load_and_build_domain_map(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config.domains.iter()
        .map(|d| (d.name.clone(), d.forward.clone()))
        .collect())
}
