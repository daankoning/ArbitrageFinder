//! Various utility tools to enable interacting with the API.
use crate::sports::Sport;
use reqwest::{Client, RequestBuilder};

/// The URL where the API lives.
///
/// All requests (should) append to this base.
const BASE_URL: &str = "https://api.the-odds-api.com/v4/";

/// A single endpoint in the API that may also contain data required
/// to specify the specific endpoint.
pub enum Endpoint {
    /// The endpoint for requesting a list of sports.
    Sports,
    /// The endpoint for getting the odds for a specific sport.
    Odds(Sport),
}

impl Endpoint {
    /// Returns the URL that corresponds to this endpoint.
    /// 
    /// Takes into account the data contained within the endpoint
    /// so that the returned URL can immediately be requested from.
    fn to_url(&self) -> String {
        // TODO: this should use Reqwest::URL
        let suffix = match self {
            Self::Sports => "sports".to_string(),
            Self::Odds(sport) => format!("sports/{}/odds", sport.key()),
        };
        format!("{BASE_URL}{suffix}")
    }
}

/// Wraps an instance of [`reqwest::Client`] along with an associated
/// API key to enable connecting with the Odds API.
/// 
/// # Examples
/// A list of sports provided by the API can be requested as follows.
/// ```
/// let client = Client::new("example_key".to_string());
///
/// let result = client.get(&Endpoint::Sports).send().await?; 
/// ```
/// Please do note that this usage is unidiomatic and [`sports::get`]
/// should be used to parse this into a vec of [`sports::Sport`].
pub struct OddsClient {
    client: Client,
    api_key: String,
}

impl OddsClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        Self { client, api_key }
    }

    /// Use `client` to prepare a GET request to `endpoint`.
    /// 
    /// The request is complete and `.send()` can immediately be called
    /// on the returned object. Some endpoints might still benefit 
    /// from additional mutation.
    pub fn get(&self, endpoint: &Endpoint) -> RequestBuilder {
        self.client
            .get(endpoint.to_url())
            .query(&[("apiKey", &self.api_key)])
    }
}
