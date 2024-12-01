use std::{
    fs::{self},
    path::Path,
};

use clap::Parser;
use project_euler::cards::Hand;

/// Solution to Problem 54: [Poker Hands](https://projecteuler.net/problem=54)
///
/// Returns number of games player 1 wins in given file.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Path of file containing random poker hands dealt to 2 players.
    #[arg[short, long, default_value_t = String::from("../data/054.txt")]]
    path: String,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(&args.path));
}

fn solution<P>(path: P) -> u32
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    fs::read_to_string(path)
        .expect(&format!("failed to read input file `{}`", path.display()))
        .split('\n')
        .filter(|line| {
            let cards = line.split_whitespace().collect::<Vec<_>>();
            let (player_1_cards, player_2_cards) = cards.split_at(cards.len() / 2);

            Hand::new(player_1_cards) > Hand::new(player_2_cards)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(4, solution("../data/tests/054.txt"));
    }
}
