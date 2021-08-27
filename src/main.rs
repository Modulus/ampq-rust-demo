use structopt::StructOpt;
use env_logger::Env;
use amqp_manager::prelude::*;
use futures::FutureExt;

use log::{info,warn};

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
    println!("Hello, world!");
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
    amqp_session
    .publish_to_routing_key(PublishToRoutingKey {
        routing_key: &queue_name,
        payload: "Hello World".as_bytes(),
        ..Default::default()
    })
    .await
    .expect("publish_to_queue");


    //Consumer

    amqp_session
    .create_consumer_with_delegate(
        CreateConsumer {
            queue_name: &queue_name,
            consumer_name: "consumer-name",
            ..Default::default()
        },
        |delivery: DeliveryResult| async {
            if let Ok(Some((channel, delivery))) = delivery {
               let payload = std::str::from_utf8(&delivery.data).unwrap();
                assert_eq!(payload, "Hello World");
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .map(|_| ())
                    .await;
            }
        },
    )
    .await
    .expect("create_consumer");


    let queue = amqp_session.create_queue(create_queue_op.clone()).await.expect("create_queue");

    if queue.message_count() <= 0 {
        info!("Message queue is: {}", queue.message_count());
        info!("All messages has been consumed!");

    }
    else {
        warn!("Messages has not been consumed!");
    }
}
