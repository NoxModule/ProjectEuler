use clap::Parser;
use project_euler::traits::StringExtensions;

/// Solution to Problem 52: [Permuted Multiples](https://projecteuler.net/problem=52)
///
/// Returns smallest integer that contains the same digits when multiplied from two to given
/// threshold.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Threshold of solution.
    #[arg[short, long, default_value_t = 6]]
    threshold: u8,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.threshold));
}

fn solution(threshold: u8) -> u32 {
    (1..)
        .find(|i| {
            (2..=threshold).all(|multiplier| {
                (i * multiplier as u32)
                    .to_string()
                    .contains_same(i.to_string())
            })
        })
        .expect(&format!("failed to find solution"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(125_874, solution(2));
    }
}
