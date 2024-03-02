use aws_sdk_sqs as sqs;
use config::Config;
use tokio::time::{sleep, Duration};
use tokio::task;
mod config;
use std::process;
mod runner;
use runner::CommandError;
use log::{debug, error, info};

#[::tokio::main]
async fn main() -> Result<(), sqs::Error> {
    env_logger::init();
    let config = match config::load_config() {
        Ok(res) => res,
        Err(_) => {
            // bail here - already logged error
            info!("Exiting");
            process::exit(1);
        }
    };

    let aws_config = aws_config::load_from_env().await;
    let client = aws_sdk_sqs::Client::new(&aws_config);

    loop {
        // we shouldn't block as it means message visibility timeout will be meaningless
        // this assumes a single consumer
        receive_messages(&client, &config).await; // blocking time to receive messages from queue
        info!("Waiting {:?}", &config.poll_ms);
        sleep(Duration::from_millis(config.poll_ms as u64)).await;
    }
}

/**
 * Poll for messages on the queue
 * When message is found, run the configured command with the body passed to the command
 * If the command returns a success code, delete the message from the queue
 */
async fn receive_messages(client: &sqs::Client, config: &Config) {
    let rcv_message_output = match client.receive_message().queue_url(&config.queue_url).send().await {
        Ok(res) => res,
        Err(error) => {
            error!("Failed to recieve messages from SQS {:?}", error);
            process::exit(1);
        }
    };


    info!("Getting messages on queue: {}", &config.queue_url);

    for message in rcv_message_output.messages.unwrap_or_default() {
        info!("On message: {:#?}", &message);

        let receipt_handle = match message.receipt_handle() {
            Some(receipt_handle) => receipt_handle,
            None => {
                error!("Invalid receipt handle on message");
                continue;
            },
        };

        let command = config.command.clone();

        let body = match message.body() {
            Some(body) => body,
            None => {
                error!("Invalid body on message");
                continue;
            },
        }.to_owned();

        let join = task::spawn(process(command, body)).await;
        match join {
            Ok(process_result) => {
                match process_result {
                    Ok(_) => {
                        info!("Process completed with success");
                        delete_message(&client, &config.queue_url, &receipt_handle).await;
                    },
                    Err(err) => {
                        error!("Process failed with error {:?}", err);
                    }
                }
                
            },
            Err(err) => {
                error!("Process task join error {:?}", err);
            }
        }

        // get success off 
    }
}

async fn process(command: String, body: String) -> Result<(), CommandError> {
    // call like $ <command> <body>
    // users should probably b64 to avoid expansion issues
    let mut command = String::from(command);
    command.push_str(" ");
    command.push_str(&body);

    return runner::run_on_shell(&command);
}

/**
 * Attempt to delete the message
 * Do nothing if fails
 */
async fn delete_message(client: &sqs::Client, queue_url: &str, receipt_handle: &str) {
    if let Err(delete_error) = client.delete_message()
        .queue_url(queue_url)
        .receipt_handle(receipt_handle)
        .send().await {
            error!("Failed to delete processed message with receipt handle {}", &receipt_handle);
            // this will cause the command to be run again
            debug!("{:?}", delete_error.raw_response());
    }
}