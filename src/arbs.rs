use crate::{odds, sports};
use crate::client::OddsClient;
use std::collections::HashMap;
use futures::future::join_all;

// struct TODONAME {
//     bookmaker: odds::Bookmaker,
//
// }

// fn implied_odds(bookmaker: odds::Bookmaker) -> Result<f64, String> {
//     let binding = bookmaker.markets();
//     let market = match binding.first() {
//         Some(x) => x,
//         None => return Err("Bookmaker has empty market".to_string()),
//     };
//
//     let x = market
//         .outcomes()
//         .iter()
//         .map(|outcome| {
//
//         });
//     todo!()
// }

#[derive(Debug)]
pub struct TODONAME {
    bookmaker_name: String,
    implied_odd: f64,
}

#[derive(Debug)]
pub struct GameCalculatedResults {
    game: odds::Match,
    outcomes: HashMap<String, TODONAME>,
}

impl GameCalculatedResults {
    fn total_implied_odds(&self) -> f64 {
        self.outcomes.values().map(|y| y.implied_odd).sum::<f64>()
    }
}

pub fn best_implied_odds(game: odds::Match) -> GameCalculatedResults {
    // let x = game.bookmakers()
    //     .into_iter()
    //     .map(implied_odds);
    let mut best_odds_per_outcome: HashMap<String, TODONAME> = HashMap::new();

    // let include_started_matches = false; // TODO: remove
    //
    // let start_time = game.commence_time();
    // let current_time = &time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs();

    // if !include_started_matches && start_time < current_time {
    //     return None;
    // }

    for bookmaker in game.bookmakers() {
        for market in bookmaker.markets() {
            for outcome in market.outcomes() {
                let current_odd = TODONAME {
                    bookmaker_name: bookmaker.key().to_owned(),
                    implied_odd: 1f64 / outcome.price(),
                };

                match best_odds_per_outcome.get(outcome.name()) {
                    Some(stored_odd) => {
                        if current_odd.implied_odd > stored_odd.implied_odd {
                            best_odds_per_outcome.insert(
                                outcome.name().to_owned(),
                                current_odd,
                            );
                        }
                    },
                    None => {
                        best_odds_per_outcome.insert(
                            outcome.name().to_owned(),
                            current_odd,
                        );
                    }
                }
            }
        }
    }

    GameCalculatedResults {game, outcomes: best_odds_per_outcome }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub async fn arbitrage(client: &OddsClient) -> f64 {
    let sports = sports::get(&client).await.unwrap();
    // sports.truncate(3);

    let odds = join_all(sports
        .into_iter()
        .map(|sp| async { odds::get(sp, &client).await } ))
        .await
        .into_iter()
        .flat_map(|sport| match sport {
            Ok(sp) => sp.into_iter(),
            Err(_) => vec![].into_iter(),
        })
        .map(best_implied_odds)
        .filter(|x| 0f64 < x.total_implied_odds() && x.total_implied_odds() < 1f64 );


    for od in odds {
        println!("{:#?}", od);
    }

    7f64
}
