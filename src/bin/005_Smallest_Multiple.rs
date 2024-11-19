use clap::Parser;

/// Solution to Problem 5: [Smallest Multiple](https://projecteuler.net/problem=5)
///
/// Returns smallest number that is evenly divisible from 1 to below given threshold.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Threshold of solution.
    #[arg[short, long, default_value_t = 20]]
    threshold: u32,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.threshold));
}

fn solution(threshold: u32) -> u32 {
    (1..)
        .find(|i| (1..=threshold).all(|j| i % j == 0))
        .expect("failed to find a solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(2520, solution(10));
    }
}
