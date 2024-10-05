// TODO: document this entire thing
use serde::Deserialize;
use std::fmt::Display;
use crate::client::{OddsClient, Endpoint};
use crate::sports::Sport;

type UnixTime = u64;

#[derive(Debug, Deserialize)]
pub struct Outcome {
    /// The name of the outcome. Usually the name of the victor or
    /// 'Draw' in the case of h2h markets, analogous for other markets.
    name: String,
    /// The price at which this outcome is being offered in decimal format.
    price: f64,
}

impl Outcome {
    /// Getter for `name` field.
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Getter for `price` field.
    pub fn price(&self) -> &f64 {
        &self.price
    }
}

#[derive(Debug, Deserialize)]
pub struct Market {
    key: String,
    outcomes: Vec<Outcome>,
}

impl Market {
    pub fn outcomes(&self) -> &Vec<Outcome> {
        &self.outcomes
    }
}

#[derive(Debug, Deserialize)]
pub struct Bookmaker {
    key: String,
    title: String,
    last_update: UnixTime,
    markets: Vec<Market>,
}

impl Bookmaker {
    pub fn key(&self) -> &String {
        &self.key
    }
    pub fn markets(&self) -> &Vec<Market> {
        &self.markets
    }
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

impl Match {
    pub fn sport_key(&self) -> &String {
        &self.sport_key
    }

    pub fn commence_time(&self) -> &UnixTime {
        &self.commence_time
    }

    pub fn home_team(&self) -> &String {
        &self.home_team
    }

    pub fn away_team(&self) -> &String {
        &self.away_team
    }

    pub fn bookmakers(&self) -> &Vec<Bookmaker> {
        &self.bookmakers
    }
}

/// The region in which we are looking for bookmakers.
/// 
/// See the [API docs](https://the-odds-api.com/sports-odds-data/bookmaker-apis.html)
/// for what bookmakers are included in what region.
#[derive(Copy, Clone)]
pub enum Region {
    US,
    US2,
    UK,
    AU,
    EU,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self { 
            Self::US =>  "us",
            Self::US2 => "us2",
            Self::UK =>  "uk",
            Self::AU =>  "au",
            Self::EU =>  "eu",
        })
    }
}

pub async fn get(sport: Sport, client: &OddsClient, region: Region) -> Result<Vec<Match>, &str> {
    let sport_key = sport.key();

    let response = client
        .get(&Endpoint::Odds(sport))
        .query(&[
            ("sport", sport_key),
            ("regions", region.to_string()),
            ("oddsformat", "decimal".to_string()),
            ("dateFormat", "unix".to_string()),
        ])
        .send()
        .await;
    // FIXME: this sometimes randomly returns American format odds (?)
    match response {
        Ok(response) if response.status().is_success() => {
            response.json::<Vec<Match>>().await.map_or(
                Err("failed to get parse match"),
                |result| Ok(result),
            )
        }
        _ => Err("Failed to fetch odds")
    }
}