use clap::Parser;

/// Solution to Problem 30: [Digit Fifth Powers](https://projecteuler.net/problem=30)
///
/// Returns sum of all numbers that can be written as the sum of given power of their digits.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Power to find solution for.
    #[arg[short, long, default_value_t = 5]]
    power: u32,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.power));
}

fn solution(power: u32) -> u32 {
    let search_threshold = (1..)
        .map_while(|digit_count| {
            let power_max = 9_u32.pow(power) * digit_count;

            if power_max.to_string().len() >= digit_count as usize {
                Some(power_max)
            } else {
                None
            }
        })
        .last()
        .expect("failed to find a search threshold");

    (2..=search_threshold)
        .filter(|i| {
            *i == i.to_string().split("").fold(0, |i_sum, digit| {
                let digit = digit.parse::<u32>().unwrap_or(0).pow(power);

                i_sum + digit
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(19_316, solution(4));
    }
}
