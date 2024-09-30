use std::env;
mod client;
mod sports;
mod odds;

pub const BASE_URL: &str = "https://api.the-odds-api.com/v4";

#[tokio::main]
async fn main() {
    let key = env::var("API_KEY").unwrap();
    let client = client::OddsClient::new(key);

    let sports = sports::get(&client).await.unwrap();


    let mut count = 0;
    // let x = odds::get(sports.into_iter().next().unwrap(), &client).await.unwrap();
    for sport in sports.into_iter() {
        let result = odds::get(sport, &client).await;
        if result.is_ok() {
            println!("{:#?}", result.unwrap());
            count += 1;
        }
        if count > 1 {
            break;
        }
    }
}
