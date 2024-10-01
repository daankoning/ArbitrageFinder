use reqwest::{Client, RequestBuilder};
use crate::sports::Sport;

const BASE_URL: &str = "https://api.the-odds-api.com/v4/";

pub enum Endpoint {
    Sports,
    Odds(Sport),
}

impl Endpoint {
    fn to_url(&self) -> String {
        // TODO: this should use Reqwest::URL
        let suffix = match self {
            Self::Sports =>  "sports".to_string(),
            Self::Odds(sport) => format!("sports/{}/odds", sport.key()),
        };
        format!("{BASE_URL}{suffix}")
    }
}

/// Wraps an instance of `reqwest::Client` to enable connecting with
/// the Odds API.
pub struct OddsClient {
    client: Client,
    api_key: String,
}

impl OddsClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        Self {
            client,
            api_key,
        }
    }

    pub fn get(&self, endpoint: &Endpoint) -> RequestBuilder {
        self.client
            .get(endpoint.to_url())
            .query(&[("apiKey", &self.api_key)])
    }
}