use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution to [SuDoku](https://projecteuler.net/problem=96). (Problem 96)
fn main() {
    let mut answer = 0;

    let input_puzzles = File::open("data/096.txt").expect("should be able to read input file");
    let puzzles_buffer = BufReader::new(input_puzzles);

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

    let mut puzzles_iterator = puzzles_buffer.lines();

    // Skip the first line, "Grid 01".
    puzzles_iterator.next();

    let mut x = 0;
    while let Some(line) = puzzles_iterator.next() {
        let line = line.expect("should be able to read input file line");

        if line.starts_with("Grid") {
            answer += puzzle_answer(&mut puzzle);

            x = 0;
            continue;
        }

        for (y, number) in line.split("").filter(|&char| !char.is_empty()).enumerate() {
            puzzle[x][y] = number
                .parse::<u8>()
                .expect("should be able to parse grid number");
        }

        x += 1;
    }

    answer += puzzle_answer(&mut puzzle);

    println!("Answer: {}", answer);
}

fn puzzle_answer(puzzle: &mut [[u8; 9]; 9]) -> u32 {
    solve_sudoku_puzzle(puzzle, 0, 0);

    let mut answer = 0;
    for i in 0..3 {
        let cell_value = puzzle[0][i] as u32;

        answer += match i {
            0 => cell_value * 100,
            1 => cell_value * 10,
            2 => cell_value,
            _ => 0,
        };
    }

    answer
}

fn solve_sudoku_puzzle(puzzle: &mut [[u8; 9]; 9], row: usize, column: usize) -> bool {
    // Avoid further backtracking if the 8th row and 9th column has been reached.
    if row == 8 && column == 9 {
        return true;
    }

    // Move to the next row and reset column index if the 9th column has been reached.
    let mut row = row;
    let mut column = column;
    if column == 9 {
        row += 1;
        column = 0;
    }

    // Move to the next column if the current cell has a value greater than 0.
    if puzzle[column][row] != 0 {
        return solve_sudoku_puzzle(puzzle, row, column + 1);
    }

    for number in 1..=9 {
        if is_legal(puzzle, row, column, number) {
            // Assign guess to the cell at the given row and column.
            puzzle[column][row] = number;

            // Move to the next column to check the assigned guess.
            if solve_sudoku_puzzle(puzzle, row, column + 1) {
                return true;
            }
        }

        // Reset the assigned guess due to it being found incorrect.
        puzzle[column][row] = 0;
    }

    false
}

/// Determine if the number can be assigned to the cell at the given row and column.
fn is_legal(puzzle: &[[u8; 9]; 9], row: usize, column: usize, number: u8) -> bool {
    // Return false if the number is found within the row or column.
    for i in 0..9 {
        if puzzle[i][row] == number {
            return false;
        }

        if puzzle[column][i] == number {
            return false;
        }
    }

    // Return false if the number is found within the section.
    let section_start_row = row - row % 3;
    let section_start_column = column - column % 3;
    for y in 0..3 {
        for x in 0..3 {
            if puzzle[section_start_column + y][section_start_row + x] == number {
                return false;
            }
        }
    }

    true
}
