use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        8
    }

    fn part1(&self, input: &str) -> String {
        run_day(input, false)
    }

    fn part2(&self, input: &str) -> String {
        run_day(input, true)
    }
}

fn run_day(input: &str, part2: bool) -> String {
    let mut map = Map::from_str(input).unwrap();

    let antennas = map.get_all_antennas();
    for antenna in antennas {
        let positions = map.get_common_places(antenna);

        let pos_pairs = positions
            .iter()
            .enumerate()
            .flat_map(|(i, x)| positions.iter().skip(i + 1).map(move |y| (x, y)));

        for (pos1, pos2) in pos_pairs {
            let freqs = map.get_frequencies(*pos1, *pos2, part2);
            for freq in freqs {
                map.add_freq(freq);
            }
        }
    }

    println!("{map}");
    let result = map.count_freq();

    result.to_string()
}

#[derive(Debug)]
struct Map {
    places: HashMap<(isize, isize), char>,
    freqs: HashSet<(isize, isize)>,
    width: isize,
    height: isize,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Place {
//     Antenna(char),
//     Freq,
// }

// impl Place {
//     pub fn get_char(&self) -> Option<char> {
//         match self {
//             Place::Antenna(x) => Some(*x),
//             Place::Freq => None,
//         }
//     }
// }

impl Map {
    pub fn get_all_antennas(&self) -> HashSet<char> {
        self.places.values().copied().collect()
    }

    pub fn get_common_places(&self, antenna: char) -> Vec<(isize, isize)> {
        let places: Vec<_> = self
            .places
            .iter()
            .filter(|(_, place)| **place == antenna)
            .map(|(k, _)| *k)
            .collect();

        places
    }

    pub fn add_freq(&mut self, pos: (isize, isize)) {
        if pos.0 < 0 || pos.0 >= self.height || pos.1 < 0 || pos.1 >= self.width {
            return;
        }
        self.freqs.insert(pos);
    }

    pub fn count_freq(&self) -> usize {
        self.freqs.len()
    }

    pub fn out_of_bounds(&self, (row, col): (isize, isize)) -> bool {
        row < 0 || row >= self.height || col < 0 || col >= self.width
    }

    fn get_freqs_left(
        &self,
        (pos1_row, pos1_col): (isize, isize),
        (pos2_row, pos2_col): (isize, isize),
        recurse: bool,
    ) -> HashSet<(isize, isize)> {
        let row_offset = (pos2_row - pos1_row).abs();
        let col_offset = (pos2_col - pos1_col).abs();

        let mut results = HashSet::new();
        let val = (pos1_row - row_offset, pos1_col - col_offset);

        if recurse && !self.out_of_bounds(val) {
            results = results
                .union(&self.get_freqs_left(val, (pos1_row, pos1_col), recurse))
                .copied()
                .collect();
        }

        results.insert(val);
        results
    }

    fn get_freqs_right(
        &self,
        (pos1_row, pos1_col): (isize, isize),
        (pos2_row, pos2_col): (isize, isize),
        recurse: bool,
    ) -> HashSet<(isize, isize)> {
        let row_offset = (pos2_row - pos1_row).abs();
        let col_offset = (pos2_col - pos1_col).abs();

        let mut results = HashSet::new();
        let val = (pos2_row + row_offset, pos2_col + col_offset);

        if recurse && !self.out_of_bounds(val) {
            results = results
                .union(&self.get_freqs_right((pos2_row, pos2_col), val, recurse))
                .copied()
                .collect();
        }

        results.insert(val);
        results
    }

    pub fn get_frequencies(
        &self,
        og_pos1: (isize, isize),
        og_pos2: (isize, isize),
        recurse: bool,
    ) -> HashSet<(isize, isize)> {
        let mut result = HashSet::new();

        // println!("{og_pos1:?} {og_pos2:?}");


        // TODO trying to get this working, it sometimes works sometimes doesn't I assume this is
        // because of the order
        let pos1 = if og_pos1.0 > og_pos2.0 {
            og_pos1
        } else {
            og_pos2
        };
        let pos2 = if og_pos1.0 > og_pos2.0 {
            og_pos2
        } else {
            og_pos1
        };

        // println!("{pos1:?} {pos2:?}");

        result = result
            .union(&self.get_freqs_left(pos1, pos2, recurse))
            .copied()
            .collect();
        result = result
            .union(&self.get_freqs_right(pos1, pos2, recurse))
            .copied()
            .collect();

        result

        // let mut points = HashSet::new();
        //
        // let mut pos1 = original_pos1;
        // let mut pos2 = original_pos2;
        // loop {
        //     let new_pos1 = self.get_freq_left(pos1, pos2);
        //     if !recurse || self.out_of_bounds(new_pos1) {
        //         break;
        //     }
        //     points.insert(new_pos1);
        //     pos1 = pos2;
        //     pos2 = new_pos1;
        // }
        //
        // let mut pos1 = original_pos1;
        // let mut pos2 = original_pos2;
        // loop {
        //     let new_pos2 = self.get_freq_right(pos1, pos2);
        //     if !recurse || self.out_of_bounds(new_pos2) {
        //         break;
        //     }
        //     points.insert(new_pos2);
        //     pos1 = pos2;
        //     pos2 = new_pos2;
        // }
        //
        // points
        //
        //
        // let row_offset = pos2_row - pos1_row;
        // let col_offset = pos2_col - pos1_col;
        //
        // let pos1 = (pos1_row - row_offset, pos1_col - col_offset);
        // let pos2 = (pos2_row + row_offset, pos2_col + col_offset);
        //
        // let mut positions = HashSet::new();
        //
        // if !self.out_of_bounds(pos1) && !self.places.contains_key(&pos1) {
        //     positions.insert(pos1);
        //     if recurse {
        //         positions = positions
        //             .union(&self.get_frequencies(pos1, (pos1_row, pos1_col), recurse))
        //             .copied()
        //             .collect();
        //     }
        // }
        // println!("Checking: {pos2:?} {:?}", (pos2_row, pos2_col));
        // if !self.out_of_bounds(pos2) && !self.places.contains_key(&pos2) {
        //     println!("Worked!");
        //     positions.insert(pos2);
        //     if recurse {
        //         positions = positions
        //             .union(&self.get_frequencies(pos2, (pos2_row, pos2_col), recurse))
        //             .copied()
        //             .collect();
        //     }
        // }
        //
        // positions
    }

    fn get_freq_left(
        &self,
        (pos1_row, pos1_col): (isize, isize),
        (pos2_row, pos2_col): (isize, isize),
    ) -> (isize, isize) {
        let row_offset = pos2_row - pos1_row;
        let col_offset = pos2_col - pos1_col;

        (pos1_row - row_offset, pos1_col - col_offset)
    }
    fn get_freq_right(
        &self,
        (pos1_row, pos1_col): (isize, isize),
        (pos2_row, pos2_col): (isize, isize),
    ) -> (isize, isize) {
        let row_offset = pos2_row - pos1_row;
        let col_offset = pos2_col - pos1_col;

        (pos2_row + row_offset, pos2_col + col_offset)
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut places = HashMap::new();

        let mut height = 0;
        let mut width = 0;

        for (row, line) in s.lines().enumerate() {
            width = 0;
            height += 1;
            for (col, c) in line.chars().enumerate() {
                width += 1;
                if c == '.' {
                    continue;
                }

                places.insert((row as isize, col as isize), c);
            }
        }

        Ok(Self {
            places,
            freqs: HashSet::new(),
            height,
            width,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for row in 0..self.height {
            for col in 0..self.width {
                if self.freqs.contains(&(row, col)) {
                    s.push('#');
                    continue;
                }
                let c = self.places.get(&(row, col)).unwrap_or(&'.');
                s.push(*c);
            }
            s.push('\n');
        }

        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    //     #[test]
    //     fn test_part1() {
    //         const INPUT: &str = "............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............";
    //
    //         let results = Day.part1(INPUT);
    //         assert_eq!(results, "14");
    //     }

    #[test]
    fn test_part2() {
        const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        println!("About to run");
        let results = Day.part2(INPUT);
        assert_eq!(results, "34");
    }
}
