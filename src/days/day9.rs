use std::{collections::VecDeque, fmt::Display, str::FromStr};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        9
    }

    fn part1(&self, input: &str) -> String {
        let mut hd = Hardrive::from_str(input).unwrap();
        hd.fragment();
        let checksum = hd.checksum();
        checksum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

struct Hardrive {
    nums: VecDeque<Option<usize>>,
}

impl Hardrive {
    pub fn is_fragmented(&self) -> bool {
        let mut hit_none = false;
        for num in &self.nums {
            if num.is_none() {
                hit_none = true;
            }
            if num.is_some() && hit_none {
                return false;
            }
        }
        true
    }

    pub fn take_from_back(&mut self) -> usize {
        self.nums
            .iter_mut()
            .rev()
            .find_map(|x| x.take())
            .expect("Empty list")
    }

    pub fn add_to_front(&mut self, val: usize) {
        let item = self
            .nums
            .iter_mut()
            .find(|x| x.is_none())
            .expect("Empty list");
        *item = Some(val);
    }

    pub fn fragment(&mut self) {
        while !self.is_fragmented() {
            let taken = self.take_from_back();
            self.add_to_front(taken);
        }
    }

    pub fn take_chunk_from_back(&mut self) -> (usize, usize) {
        todo!()
        // let mut iter = self.nums.iter_mut().rev().skip_while(|x| x.is_none());
        //
        // let num = iter.next().expect("Empty list").take().unwrap();
        //
        // let mut size = 1;
        // while let Some(opt_val) = iter.next() {
        //     let Some(n) = opt_val.take() else {
        //         break;
        //     };
        //     if n != num {
        //         break;
        //     }
        //     size += 1;
        // }
        //
        // (num, size)
    }

    pub fn add_to_front_chunk(&mut self, num: usize, len: usize) {
        todo!()
    }

    pub fn checksum(&self) -> usize {
        let mut checksum = 0;
        for (index, val) in self.nums.iter().enumerate() {
            let Some(val) = val else {
                break;
            };
            checksum += index * val;
        }

        checksum
    }
}

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
            for _ in 0..value {
                nums.push_back(insert_val);
            }
        }

        Ok(Self { nums })
    }
}

impl Display for Hardrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for num in &self.nums {
            let val = num
                .map(|x| x.to_string().chars().next_back().unwrap())
                .unwrap_or('.');
            s.push(val);
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
