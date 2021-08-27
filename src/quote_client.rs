use serde::Deserialize;
use reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
pub struct Message {
    data: Vec<String>
}


pub struct QuoteClient{
    url: String
}

impl QuoteClient{
    pub async fn get(&self) -> String {
        println!("{}", self.url);
        let response = reqwest::get(&self.url).await.expect("Failed to get response");
    
        let message : Message = response.json().await.expect("Failed to convert to json!");
    
        
        println!("{:?}", message);

        return String::from("Nullion!");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::quote_client::QuoteClient;

    #[tokio::test]
    async fn test_get_has_quote_lenth_greater_than_zero() {
        let client = QuoteClient{url: format!("http://loremricksum.com/api/?paragraphs={paragraphs}&quotes={quotes}", paragraphs=1, quotes=1)};
        let quote = client.get().await;

        assert!(quote.len() > 0);

        
    }

}