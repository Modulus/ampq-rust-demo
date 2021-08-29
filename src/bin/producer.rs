use std::{thread, time};

use env_logger::Env;
use amiquip::{Connection, Exchange, Publish, Result};

use log::{info, debug};
use ampq_rust_demo::quote_client::QuoteClient;
use ampq_rust_demo::options::Options;
use structopt::StructOpt;



#[tokio::main]
async fn main() -> Result<()>{
    info!("Producer started!");
    let cli_options = Options::from_args();

    debug!("server: {}", cli_options.server);
    debug!("level: {}", cli_options.level);
    env_logger::Builder::from_env(Env::default().default_filter_or(cli_options.level)).init();



        // Open connection.
        let mut connection = Connection::insecure_open(&cli_options.server)?;

        // Open a channel - None says let the library choose the channel ID.
        let channel = connection.open_channel(None)?;
    
        // Get a handle to the direct exchange on our channel.
        let exchange = Exchange::direct(&channel);

        loop  {
            let client = QuoteClient{url: format!("http://loremricksum.com/api/?paragraphs={paragraphs}&quotes={quotes}", paragraphs=1, quotes=1)};
            let quote = client.get().await;
            info!("Sending quote: {}", quote);
            exchange.publish(Publish::new(quote.as_bytes(), "messages"))?;

            debug!("Waiting...");
            thread::sleep(time::Duration::from_secs(2));
        }

    
      

}