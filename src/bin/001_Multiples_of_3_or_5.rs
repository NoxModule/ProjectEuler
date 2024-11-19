use clap::Parser;

/// Solution to Problem 1: [Multiples of 3 or 5](https://projecteuler.net/problem=1)
///
/// Returns sum of all multiples of 3 or 5 below given threshold.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Threshold of solution.
    #[arg[short, long, default_value_t = 1000]]
    threshold: u64,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.threshold));
}

fn solution(threshold: u64) -> u64 {
    (1..threshold)
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .fold(0, |answer, i| answer + i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(23, solution(10));
    }
}
