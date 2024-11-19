use std::{
    fs::{self},
    path::Path,
};

use clap::Parser;
use project_euler::Iterators;

/// Solution to Problem 59: [XOR Decryption](https://projecteuler.net/problem=59)
///
/// Returns sum of original message's ASCII values in given file path.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Path of file containing comma separated XOR-encrypted message ASCII values.
    #[arg[short, long, default_value_t = String::from("data/059.txt")]]
    path: String,
}

#[cfg(any(not(test), rust_analyzer))]
fn main() {
    let args = Args::parse();
    println!("Answer: {}", solution(&args.path));
}

fn solution<P>(path: P) -> u32
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let message_bytes = fs::read_to_string(path)
        .expect(&format!("failed to read input file `{}`", path.display()))
        .split(",")
        .map(|number| {
            number
                .parse::<u8>()
                .expect(&format!("failed to parse number from `{}`", number))
        })
        .collect::<Vec<_>>();

    let the_bytes = b" the ";

    (('a' as u8)..=('z' as u8))
        .ranged_permutations(3)
        .find_map(|key| {
            let decrypted_bytes = message_bytes
                .iter()
                .zip(key.iter().cycle())
                .map(|(message_byte, key_byte)| message_byte ^ key_byte)
                .collect::<Vec<_>>();

            let the_count = decrypted_bytes
                .windows(the_bytes.len())
                .filter(|window_bytes| window_bytes == the_bytes)
                .count();

            if the_count > 0 {
                Some(decrypted_bytes.iter().map(|&byte| byte as u32).sum())
            } else {
                None
            }
        })
        .expect(&format!("failed to find an answer in `{}`", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(4_057, solution("data/tests/059.txt"));
    }
}
