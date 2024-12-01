use clap::Parser;
use project_euler::iterators::Fibonacci;

/// Solution to Problem 2: [Even Fibonacci Numbers](https://projecteuler.net/problem=2)
///
/// Returns sum of even-valued Fibonacci terms below given threshold.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Threshold of solution.
    #[arg[short, long, default_value_t = 4_000_000]]
    threshold: u64,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.threshold));
}

fn solution(threshold: u64) -> u64 {
    Fibonacci::new()
        .take_while(|&term| term < threshold)
        .filter(|term| term % 2 == 0)
        .fold(0, |answer, term| answer + term)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(44, solution(90));
    }
}
