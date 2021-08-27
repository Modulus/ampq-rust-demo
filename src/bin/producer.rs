use std::{thread, time};

use structopt::StructOpt;
use env_logger::Env;
use amqp_manager::prelude::*;

use log::{info};
use ampq_rust_demo::quote_client::QuoteClient;

#[derive(Debug, StructOpt)]
#[structopt(name="demo", about="Application variables")]
struct Options {
    #[structopt(short= "s" , long = "server", default_value="locathost:5672")]
    server: String,

    #[structopt(short ="l", long = "level", default_value="INFO")]
    level: String
}


#[tokio::main]
async fn main() {
    info!("Hello, world!");
    let cli_options = Options::from_args();

    info!("server: {}", cli_options.server);
    info!("level: {}", cli_options.level);
    env_logger::Builder::from_env(Env::default().default_filter_or(cli_options.level)).init();

    let manager = AmqpManager::new(&cli_options.server, ConnectionProperties::default());
    let conn = manager.connect().await.expect("Failed to connect to ampq server!");
    let amqp_session = AmqpManager::create_session(&conn).await.expect("Failed to create AMPQ session!");

    let queue_name = "messages";

    let create_queue_op = CreateQueue {
        queue_name: &queue_name,
        options: QueueDeclareOptions {
            auto_delete: false,
            ..Default::default()
        },
        ..Default::default()
    };
    amqp_session.create_queue(create_queue_op.clone()).await.expect("create_queue");
    loop {
        let client = QuoteClient{url: format!("http://loremricksum.com/api/?paragraphs={paragraphs}&quotes={quotes}", paragraphs=1, quotes=1)};
        let quote = client.get().await;
        info!("Sending quote: {}", quote);
        amqp_session
        .publish_to_routing_key(PublishToRoutingKey {
            routing_key: &queue_name,
            payload: quote.as_bytes(),
            ..Default::default()
        })
        .await
        .expect("publish_to_queue");
        info!("Waiting...");
        thread::sleep(time::Duration::from_secs(2));
    }


}