use clap::Parser;
use project_euler::{traits::StringExtensions, Iterators};

/// Solution to Problem 4: [Largest Palindrome Product](https://projecteuler.net/problem=4)
///
/// Returns largest palindrome made from the product of two numbers of given digit count.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Number of digits to find solution for.
    #[arg[short, long, default_value_t = 3]]
    digit_count: u8,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.digit_count));
}

fn solution(digit_count: u8) -> u32 {
    let range_start = 10_u32.pow(digit_count as u32 - 1);
    let range_end = (range_start * 10) - 1;

    (range_start..=range_end)
        .ranged_permutations(2)
        .map(|permutation| permutation.iter().product::<u32>())
        .filter(|product| product.to_string().is_palindrome())
        .max()
        .expect(&format!(
            "failed to find a palindrome using `{}` digit(s)",
            digit_count,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(9009, solution(2));
    }
}
