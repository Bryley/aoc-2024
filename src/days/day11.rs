//! I found Day 11 quite fun, I started with a solution that used a BTreeSet to maintain order as
//! when I read the info writeup about it as it seemed like the order mattered a lot, however after
//! completing part 1 and reading part 2 I noticed that the order didn't matter.
//!
//! My solution also could not do part 2 as it would just take too long. The first thing I did was
//! simplify my solution and do less heap allocations. Because order didn't matter I thought if I
//! just have a function where it looks at a single stone and the number of blinks that will be
//! done, I can call that function recursively only allocating the the heap. This helped make my
//! part 1 solution go from over 300 ms to 30 ms. However part 2 still didn't work.
//!
//! My solution to part 2 was to introduce memoization. I created a RefCell inside my `Stones`
//! struct that provided interior mutability, this makes things a bit easier since I am not using
//! a multithreaded solution. I then added a hashmap in it and then wrapped my logic inside an
//! inner function that I call only if the cache doesn't have a solution, otherwise it will
//! calculate it, store in cache and return the value.
//!
//! This fix let me solve part 2 in 145 ms and part 1 in 45 ms.

use std::{cell::RefCell, collections::HashMap, num::ParseIntError, str::FromStr};

use derive_more::derive::Display;

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        11
    }

    fn part1(&self, input: &str) -> String {
        let stones = Stones::from_str(input).unwrap();
        let sum = stones.get_number_of_stones(25);
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let stones = Stones::from_str(input).unwrap();
        let sum = stones.get_number_of_stones(75);
        sum.to_string()
    }
}

#[derive(Display)]
#[display("{}", stones.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))]
struct Stones {
    stones: Vec<usize>,
    cache: RefCell<HashMap<(usize, usize), usize>>,
}

impl Stones {
    pub fn get_number_of_stones(&self, times: usize) -> usize {
        let stones = &self.stones;
        let sum: usize = stones
            .iter()
            .map(|stone| self.blink_get_stones_count(*stone, times))
            .sum();

        sum
    }
    fn blink_get_stones_count(&self, num: usize, times: usize) -> usize {
        let inner = || {
            if times == 0 {
                return 1;
            }

            if num == 0 {
                return self.blink_get_stones_count(1, times - 1);
            }

            let digits = ((num as f64).log10().floor() as usize) + 1;

            if digits % 2 == 0 {
                let ten_pow = 10_usize.pow((digits as u32) / 2);

                let num1 = num / ten_pow;
                let num2 = num % ten_pow;

                return self.blink_get_stones_count(num1, times - 1)
                    + self.blink_get_stones_count(num2, times - 1);
            }

            self.blink_get_stones_count(num * 2024, times - 1)
        };
        if let Some(res) = self.cache.borrow().get(&(num, times)) {
            return *res;
        };
        let result = inner();
        self.cache.borrow_mut().insert((num, times), result);
        result
    }
}

impl FromStr for Stones {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split_whitespace()
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Stones {
            stones,
            cache: RefCell::new(HashMap::new()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "125 17";

        let result = Day.part1(INPUT);
        assert_eq!(result, "55312");
    }
}
