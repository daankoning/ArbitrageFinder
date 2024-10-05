//! Provides utilities for fetching and manipulating types of sports.
//! 
//! The primary use of this module is to export the [`get`] function.
//! It is used to grab the sports that the API has odds for.
//! ```
//! let client = Client::new("example_key".to_string());
//! 
//! let result = sports::get(&client).await?;
//! ```
use serde::Deserialize;
use crate::client::{OddsClient, Endpoint};


/// A single sport recognized by the API.
/// 
/// A sport here is defined in the sense that the API uses. This means
/// that a 'sport' is more akin to a league because odds are categorized
/// according to division as well. Football played in the Premier league
/// is thus a different `Sport` from football played in the Bundesliga. 
#[derive(Debug, Deserialize)]
pub struct Sport {
    /// The key use by the API to identify this sport, also contains 
    /// league information.
    key: String,
}

impl Sport {
    /// Getter for `key` field.
    pub fn key(&self) -> String {
        self.key.clone()
    }
}


/// Returns the list of sports that the API is currently offering odds for.
/// 
/// The definition of 'sports' here is conforming to that in [`Sport`].
/// That is, it identifies leagues as well.
/// Additionally, the only guarantee about the returned sports is that
/// the API is aware of at least one game in this sport. 
/// It **does not** guarantee that it has any
/// - markets for the sport,
/// - bookmakers engaging in those markets,
/// - outcomes offered by those bookmakers, or
/// - bookmakers operating in any specific markets.
/// 
/// Especially the last one is fairly common. The API often knows 
/// about bookmakers operating in the US but not in Europe for college
/// games (and vice versa for the more obscure EU football leagues). 
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
