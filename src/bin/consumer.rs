use std::{thread, time};

use ampq_rust_demo::options::Options;

use env_logger::Env;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use structopt::StructOpt;


use log::{info,warn};


#[tokio::main]
async fn main() -> Result<()>{
    println!("Hello, world!");
    let cli_options = Options::from_args();


    info!("server: {}", cli_options.server);
    info!("level: {}", cli_options.level);
    env_logger::Builder::from_env(Env::default().default_filter_or(cli_options.level)).init();



    // Open connection.
    let mut connection = Connection::insecure_open(&cli_options.server)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Declare the "hello" queue.
    let queue = channel.queue_declare("messages", QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    info!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                info!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery)?;
            }
            other => {
                warn!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()

}
