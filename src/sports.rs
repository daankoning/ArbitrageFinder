use serde::Deserialize;
use crate::client::{OddsClient, Endpoint};


#[derive(Debug, Deserialize)]
pub struct Sport {
    key: String,
}

impl Sport {
    pub fn key(&self) -> String {
        self.key.to_owned()
    }
}


pub async fn get(client: &OddsClient) -> Result<Vec<Sport>, &str> {
    let response = client
        .get(&Endpoint::Sports)
        .send()
        .await;

    match response {
        Ok(response) if response.status().is_success() => {
            Ok(response.json().await.unwrap())
        }
        _ => Err("Failed to fetch sports")
    }
}
