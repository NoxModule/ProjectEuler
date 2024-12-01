use std::{
    fs::{self},
    path::Path,
};

use clap::Parser;

/// Solution to Problem 22: [Names Scores](https://projecteuler.net/problem=22)
///
/// Returns total of all names based on their index within the sorted list and each character's
/// alphabetical index in given file.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Path of file containing names.
    #[arg[short, long, default_value_t = String::from("../data/022.txt")]]
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
    let names =
        fs::read_to_string(path).expect(&format!("failed to read input file `{}`", path.display()));
    let mut names = names.trim_matches('\"').split("\",\"").collect::<Vec<_>>();

    names.sort();

    names.iter().enumerate().fold(0, |answer, (index, name)| {
        let name_total = name.chars().fold(0, |name_total, char| {
            name_total + char.to_ascii_lowercase() as u32 - 'a' as u32 + 1
        });

        answer + (name_total * (index as u32 + 1))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(42, solution("../data/tests/022.txt"));
    }
}
