use std::fs;

use clap::Parser;

/// Solution to Problem 8: [Largest Product in a Series](https://projecteuler.net/problem=8)
///
/// Returns largest product made up of adjacent digits in a 1,000-digit number.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Number of adjacent digits.
    #[arg[short, long, default_value_t = 13]]
    window_size: usize,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.window_size));
}

fn solution(window_size: usize) -> u64 {
    let input_digits =
        fs::read_to_string("../data/008.txt").expect("failed to read input file `../data/008.txt`");

    let input_digits = input_digits
        .chars()
        .map(|digit| {
            digit
                .to_digit(10)
                .expect(&format!("failed to parse digit from `{}`", digit)) as u64
        })
        .collect::<Vec<_>>();

    input_digits
        .windows(window_size)
        .map(|digits| digits.iter().product())
        .max()
        .expect("input digits should not be empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(5832, solution(4));
    }
}
