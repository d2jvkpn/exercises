#![allow(dead_code)]
use std::fs;

use anyhow::Result;
use iroh::SecretKey;
//use rand::RngCore;
use rand::prelude::*;
use serde_yaml::Value;

pub fn load_yaml(path: &str) -> Result<Value> {
    let contents = fs::read_to_string(path)?;
    let yaml: Value = serde_yaml::from_str(&contents)?;
    Ok(yaml)
}

pub fn config_get<'a>(yaml: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').fold(Some(yaml), |acc, key| acc?.get(key))
}

pub fn iroh_secret_key() -> SecretKey {
    // let secret_key = SecretKey::generate(rand::rngs::ThreadRng); // !!! rand 0.8
    // let endpoint =
    // Endpoint::builder().secret_key(secret_key.clone()).discovery_n0().bind().await?;
    // dbg!(&secret_key);

    //let yaml = load_yaml(&args.config).await?;
    //let secret_key = config_get(&yaml, "iroh.secret_key").and_then(|v| v.as_str()).unwrap();
    //let secret_key = SecretKey::from_str(secret_key).unwrap();
    //let endpoint = Endpoint::builder().secret_key(secret_key).discovery_n0().bind().await?;

    let mut rng = rand::rng();
    let mut buf = [0u8; 32];
    rng.fill_bytes(&mut buf);

    SecretKey::from_bytes(&buf)
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
