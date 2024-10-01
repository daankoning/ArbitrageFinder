use std::env;
mod client;
mod sports;
mod odds;
mod arbs;

#[tokio::main]
async fn main() {
    let key = env::var("API_KEY").unwrap();
    let client = client::OddsClient::new(key);

    let y = arbs::arbitrage(&client).await;

    println!("Found {} profitable arbs", y.len());
    for x in y.iter() {
        println!("{}", x);
    }
}
