use serde_derive::Deserialize;
use std::{error::Error, fmt, fs, path::PathBuf};
use toml;
use home::home_dir;
use::log::{debug, error};
use url::Url;

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
            error!("Failed parsing config file {}, is it valid toml? Does it contain a [config] block?", &path.to_str().unwrap_or("err"));
            debug!("{:?}", e);
            return Err(e.into());
        }
    };

    // Manually validate
    match validate(&data.config) {
        Ok(_) => Ok(data.config),
        Err(e) => {
            error!("{}", e);
            Err(e.into())
        }
    }
}

#[derive(Debug)]
struct ValidationError {
    message: String,
}

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation error: {}", self.message)
    }
}

fn validate(config: &Config) -> Result<&Config, ValidationError> {
    let mut error = String::from("");

    if Url::parse(config.queue_url.as_str()).is_err() {
        error.push_str("\"queue_url\" must be a valid url.");
    }

    if config.poll_ms < 0 {
        if error.len() > 0 {
            error.push_str(" ");
        }
        error.push_str(format!("\"poll_ms\" must be a number larger than 0 and smaller than {:?}.", i32::MAX).as_str());
    }

    if config.command.len() == 0 {
        if error.len() > 0 {
            error.push_str(" ");
        }
        error.push_str("\"command\" must be a valid string.")
    }

    if error.len() > 0 {
        Err(ValidationError {
            message: error 
        })
    } else {
        Ok(config)
    }
}