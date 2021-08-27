
use serde::Deserialize;
use reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
pub struct Message {
    data: Vec<String>
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = format!("http://loremricksum.com/api/?paragraphs={paragraphs}&quotes={quotes}", paragraphs=1, quotes=1);

    println!("{}", url);
    let response = reqwest::get(&url).await?;

    let message : Message = response.json().await?;

    
    println!("{:?}", message);

    // println!("{:?}", response.text().await?);

    Ok(())
}