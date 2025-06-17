#![allow(dead_code)]
use std::fs;

use anyhow::Result;
use serde_yaml::Value;

pub fn load_yaml(path: &str) -> Result<Value> {
    let contents = fs::read_to_string(path)?;
    let yaml: Value = serde_yaml::from_str(&contents)?;
    Ok(yaml)
}

pub fn config_get<'a>(yaml: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').fold(Some(yaml), |acc, key| acc?.get(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config() {
        let yaml = load_yaml("configs/local.yaml").unwrap();
        let secret_key = config_get(&yaml, "iroh.secret_key").and_then(|v| v.as_str()).unwrap();
        dbg!(&secret_key);
    }
}
