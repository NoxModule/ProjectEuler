use std::{
    fs::{self},
    path::Path,
};

use clap::Parser;
use project_euler::Sudoku;

/// Solution to Problem 96: [Sudoku](https://projecteuler.net/problem=96)
///
/// Returns sum of 3-digit numbers in top-left corner of each puzzle solution.
#[cfg(any(not(test), rust_analyzer))]
#[derive(Parser)]
struct Args {
    /// Path of file containing sudoku puzzles.
    #[arg[short, long, default_value_t = String::from("data/096.txt")]]
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
    let mut answer = 0;

    let mut puzzle: [[u8; 9]; 9] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let path = path.as_ref();
    let grid_lines =
        fs::read_to_string(path).expect(&format!("failed to read input file `{}`", path.display()));

    let grid_lines = grid_lines
        .split('\n')
        .filter(|line| !line.starts_with("Grid"))
        .enumerate();

    for (y, row) in grid_lines {
        for (x, number) in row.split("").filter(|char| !char.is_empty()).enumerate() {
            let puzzle_y = y % 9;

            if puzzle_y == 0 && y != 0 {
                answer += puzzle_answer(&mut puzzle);
            }

            puzzle[puzzle_y][x] = number.parse::<u8>().expect(&format!(
                "should be able to parse grid number at ({}, {})",
                x, puzzle_y,
            ));
        }
    }

    answer += puzzle_answer(&mut puzzle);

    answer
}

fn puzzle_answer(puzzle: &mut [[u8; 9]; 9]) -> u32 {
    Sudoku::solve(puzzle);

    let answer = puzzle[0][0..3]
        .iter()
        .map(|cell_value| cell_value.to_string())
        .collect::<String>();

    answer
        .parse()
        .expect(&format!("failed to parse number from `{}`", answer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_returns_expected_answer() {
        // assert
        assert_eq!(3030, solution("data/tests/096.txt"));
    }
}
