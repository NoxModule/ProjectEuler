use clap::Parser;
use project_euler::Math;

/// Solution to Problem 3: [Largest Prime Factor](https://projecteuler.net/problem=3)
///
/// Returns largest prime factor of given number.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Number to find solution for.
    #[arg[short, long, default_value_t = 600_851_475_143]]
    number: u64,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.number));
}

fn solution(number: u64) -> u64 {
    *Math::prime_factors(number)
        .keys()
        .max()
        .expect(&format!("failed to find a prime factor for `{}`", number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(29, solution(13195));
    }
}
