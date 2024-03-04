use std::process::Command;
use log::{error, info};
use std::fmt;

#[derive(Debug, Clone)]
pub enum CommandError {
    NonZeroExit,
    IOError
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::NonZeroExit => 
                write!(f, "Command failed with non zero exit code"),
            CommandError::IOError => 
                write!(f, "Failed to execute command. IOError"),
        }
    }
}

pub fn run_on_shell(command: &String) -> Result<(), CommandError> {
    info!("Running command {:?}", command);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output().map_err(|_| CommandError::IOError);

    if let Ok(output) = output {
        if !output.status.success() {
            error!("Task failed with exit code {:?}", output.status.code().unwrap());
            return Err(CommandError::NonZeroExit);
        }
    }

    Ok(())
}