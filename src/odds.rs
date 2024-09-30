use serde::Deserialize;
use crate::client::{OddsClient, Endpoint};
use crate::sports::Sport;

type UnixTime = u64;

#[derive(Debug, Deserialize)]
struct Outcome {
    name: String,
    price: f64,
}

#[derive(Debug, Deserialize)]
struct Market {
    key: String,
    outcomes: Vec<Outcome>,
}

#[derive(Debug, Deserialize)]
struct Bookmaker {
    key: String,
    title: String,
    last_update: UnixTime,
    markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    id: String,
    sport_key: String,
    commence_time: UnixTime,
    home_team: String,
    away_team: String,
    bookmakers: Vec<Bookmaker>,
}

pub async fn get(sport: Sport, client: &OddsClient) -> Result<Vec<Match>, &str> {
    let sport_key = sport.key();

    let response = client
        .get(&Endpoint::Odds(sport))
        .query(&[
            ("sport", sport_key),
            ("regions", "eu".to_string()), // TODO: argumentify
            ("oddsformat", "decimal".to_string()),
            ("dateFormat", "unix".to_string()),
        ])
        .send()
        .await;

    match response {
        Ok(response) if response.status().is_success() => {
            match response.json::<Vec<Match>>().await {
                Ok(result) => Ok(result),
                Err(_) => Err("failed to get parse match"),
            }
        }
        _ => Err("Failed to fetch odds")
    }
}