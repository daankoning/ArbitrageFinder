use crate::{odds, sports};
use crate::client::OddsClient;
use std::collections::HashMap;
use std::fmt::Display;
use futures::future::join_all;
use colored::Colorize;

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

impl Display for GameCalculatedResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "\t{} v {} in {}\n",
            self.game.home_team().italic().blue(),
            self.game.away_team().italic().red(),
            self.game.sport_key().italic(),
        )?;
        write!(f, "\tWith total implied odds {}", self.total_implied_odds().to_string())?;

        // TODO: this could be cleaner
        for outcome in self.outcomes.iter() {
            write!(f, "\n\t\t{} for {} at {}",
                   outcome.0.bold(),
                   1f64 / outcome.1.implied_odd,
                   outcome.1.bookmaker_name,
            )?;
        }
        write!(f, "")
    }
}

pub fn best_implied_odds(game: odds::Match) -> GameCalculatedResults {
    // let x = game.bookmakers()
    //     .into_iter()
    //     .map(implied_odds);
    let mut best_odds_per_outcome: HashMap<String, TODONAME> = HashMap::new();

    for bookmaker in game.bookmakers() {
        for market in bookmaker.markets() {
            for outcome in market.outcomes() {
                let current_odd = TODONAME {
                    bookmaker_name: bookmaker.key().to_owned(),
                    implied_odd: 1f64 / outcome.price(),
                };

                match best_odds_per_outcome.get(outcome.name()) {
                    Some(stored_odd) => {
                        if current_odd.implied_odd < stored_odd.implied_odd {
                            best_odds_per_outcome.insert(
                                outcome.name().to_owned(),
                                current_odd,
                            );
                        }
                    }
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

    GameCalculatedResults { game, outcomes: best_odds_per_outcome }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub async fn arbitrage(client: &OddsClient) -> Vec<GameCalculatedResults> {
    join_all(
        sports::get(&client)
            .await
            .unwrap()
            .into_iter()
            .map(|sp| async { odds::get(sp, &client).await }))
        .await
        .into_iter()
        .flat_map(|sport| match sport {
            Ok(sp) => sp.into_iter(),
            Err(_) => vec![].into_iter(),
        })
        .map(best_implied_odds)
        .filter(|x| 0f64 < x.total_implied_odds() && x.total_implied_odds() < 1f64)
        .collect()
}
