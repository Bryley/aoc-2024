//! Day 4 part 1 was solved using a versatile solution using recursion.
//!
//! This recursive function took a string slice, row, column and transformation functions to those
//! rows and columns, then would check if the the first character of the string slice matched the
//! given row and column then run the function again except with the string slice's first character
//! removed and the transformations applied to the rows and columns.
//!
//! This means this code could be reused for words other than "XMAS" simply by changing the input
//! to the function.
//!
//! Part 2 was a bit less versatile, it still uses the recursive function, but this time instead of
//! having versatile input "MAS" is hard coded in. This could easily be changed by getting the
//! input, finding the middle character and changing the start row and col to match however I feel
//! this code will over complicate the simple problem and the simple diagonal check I do is enough.

use std::str::{Chars, FromStr};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        4
    }

    fn part1(&self, input: &str) -> String {
        let board = Board::from_str(input).unwrap();

        let mut total = 0;
        for (row, col, _) in board.iter() {
            total += board.look_for("XMAS", row, col);
        }
        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let board = Board::from_str(input).unwrap();
        let mut total = 0;
        for (row, col, _) in board.iter() {
            total += board.find_mas(row, col) as usize;
        }
        total.to_string()
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<char>>,
    pub width: usize,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut rows = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            width = row.len();
            rows.push(row);
        }

        Ok(Self {
            rows,
            width,
        })
    }
}

impl Board {
    pub fn get(&self, row: i64, col: i64) -> Option<char> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;

        self.rows.get(row).and_then(|v| v.get(col)).copied()
    }

    /// Looks at pos (row, col) for word in all 8 directions returning the number
    /// of occurances
    pub fn look_for(&self, word: &str, row: i64, col: i64) -> usize {
        let mut count = 0;
        // Top Left
        count += self.look_for_recursive(word.chars(), row, col, |r| r - 1, |c| c - 1) as usize;
        // Top
        count += self.look_for_recursive(word.chars(), row, col, |r| r - 1, |c| c) as usize;
        // Top Right
        count += self.look_for_recursive(word.chars(), row, col, |r| r - 1, |c| c + 1) as usize;
        // Left
        count += self.look_for_recursive(word.chars(), row, col, |r| r, |c| c - 1) as usize;
        // Right
        count += self.look_for_recursive(word.chars(), row, col, |r| r, |c| c + 1) as usize;
        // Bottom Left
        count += self.look_for_recursive(word.chars(), row, col, |r| r + 1, |c| c - 1) as usize;
        // Bottom
        count += self.look_for_recursive(word.chars(), row, col, |r| r + 1, |c| c) as usize;
        // Bottom Right
        count += self.look_for_recursive(word.chars(), row, col, |r| r + 1, |c| c + 1) as usize;

        count
    }

    /// Find the 'MAS' in the X shape for a given row and col, false if not
    /// there true if there
    pub fn find_mas(&self, row: i64, col: i64) -> bool {
        let Some(current_char) = self.get(row, col) else {
            return false;
        };
        if current_char != 'A' {
            return false;
        }

        let diagonal1 =
            self.look_for_recursive("MAS".chars(), row - 1, col - 1, |r| r + 1, |c| c + 1)
                || self.look_for_recursive("SAM".chars(), row - 1, col - 1, |r| r + 1, |c| c + 1);
        let diagonal2 =
            self.look_for_recursive("MAS".chars(), row - 1, col + 1, |r| r + 1, |c| c - 1)
                || self.look_for_recursive("SAM".chars(), row - 1, col + 1, |r| r + 1, |c| c - 1);

        diagonal1 && diagonal2
    }

    /// Recursively checks for word in a direction using transforms to row and col
    fn look_for_recursive(
        &self,
        mut word: Chars<'_>,
        row: i64,
        col: i64,
        row_fn: fn(i64) -> i64,
        col_fn: fn(i64) -> i64,
    ) -> bool {
        // Base check
        let Some(expected_char) = word.next() else {
            return true;
        };
        // Out of bounds
        let Some(current_char) = self.get(row, col) else {
            return false;
        };
        if current_char != expected_char {
            return false;
        }
        self.look_for_recursive(word, row_fn(row), col_fn(col), row_fn, col_fn)
    }

    pub fn iter(&self) -> BoardIter<'_> {
        BoardIter {
            board: self,
            index: 0,
        }
    }
}

struct BoardIter<'a> {
    board: &'a Board,
    index: usize,
}

impl Iterator for BoardIter<'_> {
    type Item = (i64, i64, char);

    fn next(&mut self) -> Option<Self::Item> {
        let row = (self.index / self.board.width) as i64;
        let col = (self.index % self.board.width) as i64;

        self.index += 1;

        let current_char = self.board.get(row, col)?;

        Some((row, col, current_char))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        const TEST_INP: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = Day.part1(TEST_INP);

        assert_eq!(result, "18")
    }

    #[test]
    fn test_part2() {
        const TEST_INP: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = Day.part2(TEST_INP);

        assert_eq!(result, "9")
    }
}
