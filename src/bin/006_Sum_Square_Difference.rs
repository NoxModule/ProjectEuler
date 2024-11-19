use clap::Parser;

/// Solution to Problem 6: [Sum Square Difference](https://projecteuler.net/problem=6)
///
/// Returns difference between sum of squares and square of sum equal to given threshold.
#[derive(Parser)]
struct Args {
    /// Threshold of solution.
    #[arg[short, long, default_value_t = 100]]
    threshold: u32,
}

fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(args.threshold));
}

fn solution(threshold: u32) -> u32 {
    let sums = (1..=threshold).fold((0, 0), |(squares, sum), i| (squares + i.pow(2), sum + i));

    sums.1.pow(2) - sums.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(2_640, solution(10));
    }
}
