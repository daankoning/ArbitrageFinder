//! THis module is for generating arbitrage opportunities.
//! 
//! Externally, you should only need to call [`arbitrage`]:
//! ```
//! // Create a client (only necessary for the example, you should already have one)
//! let client = Client::new("example_key".to_string());
//! 
//! // Fetch the opportunities:
//! let arbitrage_opportunities = arbs::arbitrage(&client).await;
//! println!("{arbitrage_opportunities}"); // (Display is implemented)
//! ```
use crate::{odds, sports};
use crate::client::OddsClient;
use std::collections::HashMap;
use std::fmt::Display;
use futures::future::join_all;
use colored::Colorize;
use ordered_float::OrderedFloat;


/// A simple pair of a bookie and the odds they offer.
///
/// Used as way of flattening the deeply nested structure [`Match::bookmakers`](odds::Match::bookmakers).
#[derive(Debug)]
struct BookieOutcome {
    bookmaker_name: String,
    implied_odd: f64,
}

/// Structures a game together with the best odds for each outcome.
///
/// Used to structure the outputs of [`best_implied_odds`] and [`arbitrage`].
#[derive(Debug)]
pub struct GameCalculatedResults {
    /// The game to which these results belong.
    game: odds::Match,
    /// A mapping from the name of the outcome to the bookie offering
    /// the best odds for that outcome.
    ///
    /// The outcome naming conventions are inherited from [`Outcome::name`](odds::Outcome::name).
    outcomes: HashMap<String, BookieOutcome>,
}

impl GameCalculatedResults {
    /// Returns the total implied odds of the game.
    /// 
    /// This is the sum of the inverses of the best odds for each outcome.
    /// See the crate documentation for a more extensive explanation on
    /// what this means.
    fn total_implied_odds(&self) -> f64 {
        self.outcomes.values().map(|y| y.implied_odd).sum()
    }
}

impl Display for GameCalculatedResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f, "\t{} v {} in {}",
            self.game.home_team().italic().blue(),
            self.game.away_team().italic().red(),
            self.game.sport_key().italic(),
        )?;
        write!(f, "\tWith total implied odds {}", self.total_implied_odds())?;

        // TODO: this could be cleaner
        for outcome in &self.outcomes {
            write!(f, "\n\t\t{} for {} at {}",
                   outcome.0.bold(),
                   1f64 / outcome.1.implied_odd,
                   outcome.1.bookmaker_name,
            )?;
        }
        write!(f, "")
    }
}


/// Contains one function, [`sorted_by_key`](Self::sorted_by_key) which allows a container
/// to be sorted and return itself. The container should contain items of type `T`.
///
/// This is needed to be able to sort the results of [`arbitrage`]
/// whilst maintaining a functional change instead of needing to
/// resort to mutability.

// TODO: In the future, this may expand to include the more conventional sorting functions, as well as being `#[derive]`able.
trait Sorted<T> {
    fn sorted_by_key<U>(self, f: fn(&T) -> U) -> Self
    where
        U: Ord;
}

impl<T> Sorted<T> for Vec<T> {
    fn sorted_by_key<U>(mut self, f: fn(&T) -> U) -> Self
    where
        U: Ord,
    {
        self.sort_by_key(f);
        self
    }
}

/// Calculates the best odds for each outcome in a given match.
fn best_implied_odds(game: odds::Match) -> GameCalculatedResults {
    // let x = game.bookmakers()
    //     .into_iter()
    //     .map(implied_odds);
    let mut best_odds_per_outcome: HashMap<String, BookieOutcome> = HashMap::new();

    for bookmaker in game.bookmakers() {
        for market in bookmaker.markets() {
            for outcome in market.outcomes() {
                let current_odd = BookieOutcome {
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


/// Gives a Vec of profitable arbitrage opportunities, sorted by
/// profit margin.

// TODO: take bunch of extra arguments such as region and cuttofs
pub async fn arbitrage(client: &OddsClient) -> Vec<GameCalculatedResults> {
    join_all(
        sports::get(&client)
            .await
            .unwrap()
            .into_iter()
            .map(|sp| async { odds::get(sp, client).await }))
        .await
        .into_iter()
        .flat_map(
            |sport| sport.map_or_else(|_| vec![].into_iter(), IntoIterator::into_iter),
        )
        .map(best_implied_odds)
        .filter(|x| 0f64 < x.total_implied_odds() && x.total_implied_odds() < 1f64)
        .collect::<Vec<_>>()
        .sorted_by_key(|x| OrderedFloat(x.total_implied_odds()))
}
