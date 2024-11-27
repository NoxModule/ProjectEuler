use clap::Parser;
use project_euler::iterators::Primes;

/// Solution to Problem 7: [10,001st Prime](https://projecteuler.net/problem=7)
///
/// Returns the prime number at given prime index.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Index of prime number to get.
    #[arg[short, long, default_value_t = 10_001]]
    prime_index: usize,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.prime_index));
}

fn solution(prime_index: usize) -> u64 {
    Primes::new().skip(prime_index - 1).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(13, solution(6));
    }
}
