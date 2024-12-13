//! Day 10 I believe was a little bit simplier than the some of the previous days
//! 
//! The way I tackled it was to store the trail in a 2D array that can be easily accessed at a
//! certain row and column using the `get` method. Then I created a function called
//! `trail_score` which will return a boxed iterator over position coordinates (usize, usize).
//!
//! This function will be called recursively for each north, south, east and west directions,
//! adding to the iterator if it finds the end of a trail otherwise it returns an empty iterator.
//!
//! All these iterators are added together for each starting trail to get all finishing positions
//! and paths to them.
//!
//! Doing it this way made it quite easy to share code to solve each part. Part 1 was simply to
//! collect the iterator as a HashSet to only get the unique values, then get the length of that.
//!
//! Then for part 2 I simply just found the number of paths inside the iterator to get each path.

use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        10
    }

    fn part1(&self, input: &str) -> String {
        let map = Map::from_str(input).unwrap();
        map.find_trailhead_positions(true).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let map = Map::from_str(input).unwrap();
        map.find_trailhead_positions(false).to_string()
    }
}

struct Map {
    rows: Vec<Vec<u8>>,
}

impl Map {
    pub fn find_trailhead_positions(&self, part1: bool) -> usize {
        let mut final_scores = 0;
        for (row, cols) in self.rows.iter().enumerate() {
            for (col, height) in cols.iter().enumerate() {
                if *height != 0 {
                    continue;
                }
                let final_iter = self.trail_score(0, row as isize, col as isize);
                if part1 {
                    final_scores += final_iter.collect::<HashSet<_>>().len();
                } else {
                    final_scores += final_iter.count();
                }
            }
        }

        final_scores
    }

    /// Recursive function that will follow a trail getting all endpositions
    fn trail_score(
        &self,
        expects: u8,
        row: isize,
        col: isize,
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        // fn trail_score(&self, expects: u8, row: isize, col: isize) -> HashSet<(usize, usize)> {
        let Some(current_value) = self.get(row, col) else {
            return Box::new(std::iter::empty());
        };

        if current_value != expects {
            return Box::new(std::iter::empty());
        }

        if current_value == 9 {
            return Box::new(std::iter::once((row as usize, col as usize)));
        }

        let final_iter = std::iter::empty()
            .chain(self.trail_score(expects + 1, row - 1, col))
            .chain(self.trail_score(expects + 1, row, col + 1))
            .chain(self.trail_score(expects + 1, row + 1, col))
            .chain(self.trail_score(expects + 1, row, col - 1));

        Box::new(final_iter)
    }

    pub fn get(&self, row: isize, col: isize) -> Option<u8> {
        let row = self.rows.get(row as usize)?;
        let item = row.get(col as usize)?;

        Some(*item)
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();

        for row in s.lines() {
            let mut map_row = Vec::new();
            for col in row.chars() {
                let step = col.to_string().parse::<u8>()?;
                map_row.push(step);
            }
            map.push(map_row);
        }

        Ok(Self { rows: map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = Day.part1(INPUT);

        assert_eq!(result, "36")
    }
    #[test]
    fn test_part2() {
        const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = Day.part2(INPUT);

        assert_eq!(result, "81")
    }
}
