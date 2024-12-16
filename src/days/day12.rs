use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        12
    }

    fn part1(&self, input: &str) -> String {
        let garden = Garden::from_str(input).unwrap();
        let sum: usize = garden
            .plots()
            .iter()
            .map(|plot| plot.area() * plot.perimeter())
            .sum();

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let garden = Garden::from_str(input).unwrap();
        let sum: usize = garden
            .plots()
            .iter()
            .map(|plot| plot.area() * plot.advanced_perimeter())
            .sum();

        sum.to_string()
    }
}

struct Garden {
    squares: Vec<Vec<char>>,
}

impl Garden {
    pub fn get(&self, row: isize, col: isize) -> Option<char> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = self.squares.get(row as usize)?;
        row.get(col as usize).copied()
    }

    pub fn plots(&self) -> Vec<Plot> {
        let mut plots = Vec::<Plot>::new();
        for (row, cols) in self.squares.iter().enumerate() {
            for (col, c) in cols.iter().enumerate() {
                if plots
                    .iter()
                    .any(|plot| plot.contains(row as isize, col as isize))
                {
                    continue;
                }
                let plot = self.plot(*c, row as isize, col as isize);
                plots.push(plot);
            }
        }
        plots
    }

    fn plot(&self, c: char, row: isize, col: isize) -> Plot {
        let set = self.find(HashSet::new(), c, row, col);

        Plot {
            grow: c,
            squares: set,
        }
    }

    fn find(
        &self,
        mut set: HashSet<(isize, isize)>,
        c: char,
        row: isize,
        col: isize,
    ) -> HashSet<(isize, isize)> {
        let Some(current_char) = self.get(row, col) else {
            return set;
        };

        if set.contains(&(row, col)) {
            return set;
        }

        if current_char != c {
            return set;
        }

        if !set.insert((row, col)) {
            return set;
        }

        set = self.find(set, c, row - 1, col); // Up
        set = self.find(set, c, row, col + 1); // Right
        set = self.find(set, c, row + 1, col); // Down
        self.find(set, c, row, col - 1) // Left
    }
}

struct Plot {
    grow: char,
    squares: HashSet<(isize, isize)>,
}

impl Plot {
    pub fn contains(&self, row: isize, col: isize) -> bool {
        self.squares.contains(&(row, col))
    }

    pub fn area(&self) -> usize {
        self.squares.len()
    }

    pub fn perimeter(&self) -> usize {
        self.squares
            .iter()
            .map(|(row, col)| {
                let mut count = 0;

                count += !self.contains(*row - 1, *col) as usize; // Up
                count += !self.contains(*row, *col + 1) as usize; // Right
                count += !self.contains(*row + 1, *col) as usize; // Down
                count += !self.contains(*row, *col - 1) as usize; // Left

                count
            })
            .sum()
    }

    pub fn advanced_perimeter(&self) -> usize {
        let checked_squares: HashMap<(isize, isize), (bool, bool, bool, bool)> = self
            .squares
            .iter()
            .map(|pos| (*pos, (false, false, false, false)))
            .collect();

        for (row, col) in &self.squares {
            // Top
            let top_edge = !self.contains(*row - 1, *col);
            if top_edge {
                let mut offset = 0;
                loop {
                    offset += 1;
                    if !self.contains(*row - 1, *col) {

                    };
                }
            }
        }

        todo!()
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            rows.push(row);
        }

        Ok(Self { squares: rows })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "AAAA
BBCD
BBCC
EEEC";

    const INPUT2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const INPUT3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1_input1() {
        let result = Day.part1(INPUT1);

        assert_eq!(result, "140");
    }
    #[test]
    fn test_part1_input2() {
        let result = Day.part1(INPUT2);

        assert_eq!(result, "772");
    }

    #[test]
    fn test_part1_input3() {
        let result = Day.part1(INPUT3);

        assert_eq!(result, "1930");
    }

    #[test]
    fn test_part2_input1() {
        let result = Day.part2(INPUT1);

        assert_eq!(result, "80");
    }
    #[test]
    fn test_part2_input2() {
        let result = Day.part2(INPUT2);

        assert_eq!(result, "436");
    }

    #[test]
    fn test_part2_input3() {
        let result = Day.part2(INPUT3);

        assert_eq!(result, "1206");
    }
}
