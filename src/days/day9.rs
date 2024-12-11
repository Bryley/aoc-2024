use std::{collections::VecDeque, fmt::Display, iter::repeat, str::FromStr};

use regex::Regex;

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        9
    }

    fn part1(&self, input: &str) -> String {
        let mut hd = Hardrive::from_str(input).unwrap();
        println!("{hd} | {}", hd.is_fragmented());
        let taken = hd.take_chunk_from_back();
        println!("{hd} | {} | {taken:?}", hd.is_fragmented());
        // hd.fragment();
        // let checksum = hd.checksum();
        // checksum.to_string()
        "".to_string()
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

struct Hardrive {
    // (Optional ID (None = Freespace), length)
    nums: VecDeque<(Option<usize>, usize)>,
}

impl Hardrive {
    /// This function is O(n) so maybe try to make it cheaper to check, or check less frequently?
    pub fn is_fragmented(&self) -> bool {
        let mut hit_none = false;
        for (id, len) in &self.nums {
            if id.is_none() && *len != 0 {
                hit_none = true;
            }
            if id.is_some() && hit_none {
                return false;
            }
        }
        true
    }

    pub fn take_chunk_from_back(&mut self) -> (usize, usize, usize) {
        let (index, (id, len)) = self
            .nums
            .iter_mut()
            .enumerate()
            .rev()
            .find(|(_, (id, _))| id.is_some())
            .unwrap();
        let id = id.take().unwrap();

        (index, id, *len)
    }

    // TODO NTS: Working on this, trying to think of best way to only move it if there is space for
    // it.
    pub fn add_chunk_to_front(&mut self, id: usize, len: usize) {
        let Some(x) = self
            .nums
            .iter_mut()
            .find(|(id, l)| *l >= len && id.is_none())
        else {
            return;
        };
    }
}

// impl Hardrive {
//     pub fn is_fragmented(&self) -> bool {
//         let mut hit_none = false;
//         for num in &self.nums {
//             if num.is_none() {
//                 hit_none = true;
//             }
//             if num.is_some() && hit_none {
//                 return false;
//             }
//         }
//         true
//     }
//
//     pub fn take_from_back(&mut self) -> usize {
//         self.nums
//             .iter_mut()
//             .rev()
//             .find_map(|x| x.take())
//             .expect("Empty list")
//     }
//
//     pub fn add_to_front(&mut self, val: usize) {
//         let item = self
//             .nums
//             .iter_mut()
//             .find(|x| x.is_none())
//             .expect("Empty list");
//         *item = Some(val);
//     }
//
//     pub fn fragment(&mut self) {
//         while !self.is_fragmented() {
//             let taken = self.take_from_back();
//             self.add_to_front(taken);
//         }
//     }
//
//     pub fn take_chunk_from_back(&mut self) -> (usize, usize) {
//         todo!()
//         // let mut iter = self.nums.iter_mut().rev().skip_while(|x| x.is_none());
//         //
//         // let num = iter.next().expect("Empty list").take().unwrap();
//         //
//         // let mut size = 1;
//         // while let Some(opt_val) = iter.next() {
//         //     let Some(n) = opt_val.take() else {
//         //         break;
//         //     };
//         //     if n != num {
//         //         break;
//         //     }
//         //     size += 1;
//         // }
//         //
//         // (num, size)
//     }
//
//     pub fn add_to_front_chunk(&mut self, num: usize, len: usize) {
//         todo!()
//     }
//
//     pub fn checksum(&self) -> usize {
//         let mut checksum = 0;
//         for (index, val) in self.nums.iter().enumerate() {
//             let Some(val) = val else {
//                 break;
//             };
//             checksum += index * val;
//         }
//
//         checksum
//     }
// }

impl FromStr for Hardrive {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = VecDeque::new();
        for (index, c) in s.trim().chars().enumerate() {
            let occupy = index % 2 == 0;

            let value: usize = c
                .to_string()
                .parse()
                .map_err(|_| format!("Failed to convert digit '{c}'"))?;

            let insert_val = if occupy { Some(index / 2) } else { None };
            nums.push_back((insert_val, value));
        }

        Ok(Self { nums })
    }
}

impl Display for Hardrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for num in &self.nums {
            let (id, len) = num;

            let id = id
                .map(|x| x.to_string().chars().next_back().unwrap())
                .unwrap_or('.');

            s.push_str(&id.to_string().repeat(*len));
        }

        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "2333133121414131402";

        let results = Day.part1(INPUT);
        assert_eq!(results, "1928");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "2333133121414131402";

        let results = Day.part2(INPUT);
        assert_eq!(results, "2858");
    }
}
