pub struct Sudoku;

impl Sudoku {
    pub fn solve(puzzle: &mut [[u8; 9]; 9]) -> bool {
        Sudoku::recursive_solve(puzzle, 0, 0)
    }

    fn recursive_solve(puzzle: &mut [[u8; 9]; 9], row: usize, column: usize) -> bool {
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
            return Sudoku::recursive_solve(puzzle, row, column + 1);
        }

        for number in 1..=9 {
            if Sudoku::is_legal(puzzle, row, column, number) {
                // Assign guess to the cell at the given row and column.
                puzzle[column][row] = number;

                // Move to the next column to check the assigned guess.
                if Sudoku::recursive_solve(puzzle, row, column + 1) {
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
}
