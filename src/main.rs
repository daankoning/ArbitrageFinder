//! **Note:** This documentation is intended for development use.
//! If you only want to use the script, check out `README.md`
//! or the [repository](https://github.com/daankoning/ArbitrageFinder/).
use std::env;
mod client;
mod sports;
mod odds;
mod arbs;

#[tokio::main]
async fn main() {
    let key = match env::var("API_KEY") {
        Ok(val) => val,
        Err(_e) => panic!("Please ensure the API key is set"),
    };
    let client = client::OddsClient::new(key);
    
    let y = arbs::arbitrage(&client).await;
    
    println!("Found {} profitable arbs", y.len());
    for x in &y {
        println!("{x}");
    }
}
