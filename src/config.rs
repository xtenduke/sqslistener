use serde_derive::Deserialize;
use std::{error::Error, fs, path::PathBuf};
use toml;
use home::home_dir;
use::log::{debug, error};

/**
 * [config]
 * queue_url=https://sqs.some-region-1.amazonaws.com/12345678/queuename
 * poll_ms=30000
 * script_path=/home/me/deployment/deploy.sh
 */
#[derive(Deserialize)]
struct Data {
    config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub queue_url: String,
    pub poll_ms: i32,
    pub command: String
}

// Create path from home
fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut path: PathBuf = match home_dir() {
        Some(res) => res,
        None => {
            return Err(Box::from("Failed getting config dir"));
        }
    };

    path.push(".config/sqslistener.toml");
    Ok(path)
}


// TODO: Validate config
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let path = match get_config_path() {
        Ok(res) => res,
        Err(e) => {
            error!("Failed getting config dir, are you running in a compatible environment?");
            debug!("{:?}", e);
            return Err(e.into());
        }
    };

    let file_content = match fs::read_to_string(&path) {
        Ok(res) => res,
        Err(e) => {
            error!("Failed reading config file at {}", &path.to_str().unwrap_or("err"));
            debug!("{:?}", e);
            return Err(e.into());
        }
    };

    let data: Data = match toml::from_str(&file_content) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed parsing config file {}, is it valid toml?", &path.to_str().unwrap_or("err"));
            debug!("{:?}", e);
            return Err(e.into());
        }
    };

    Ok(data.config)
}