//! The hardest challenge for day 7 for me was probably trying to generate all permutations of the
//! equations that could be generated.
//!
//! The way I overcame this was to create an `add_num` function that would take a number, add it to
//! the equation using all possible Operators returning a Vec of these possible equations.
//!
//! This solution is still not perfect and the biggest performance decrease I believe is the fact
//! that the equations are not lazily evaluated. If I instead focused on generating the
//! permutations in a lazy way via an Iterator for example it could save a lot of future work and
//! speed up the application a lot. Right now it creates all possible Equations, allocates them in a list.
//! Then checks them one by one after already allocating them all. We could also run each check in
//! parallel using threads to have further improvements.
//!
//! Another thing I focused on more in this day's challenge is trying to make the storing of the
//! `Equation` data impossible to hold something that is an invalid state, in other words "Making
//! impossible states impossible". A great talk on the topic is done by Richard Feldman at Elm conf:
//! https://www.youtube.com/watch?v=IcgmSRJHu_8
//! It's target language is Elm however you can transfer those skills into most languages that
//! allow you to create custom datastructures.
//!
//! For instance my first thought to store an Equation was to have something like this:
//!
//! ```rust
//! struct Equation {
//!     nums: Vec<isize>,
//!     operators: Vec<Operator>,
//! }
//! ```
//!
//! However the problem with this is that there are lots of invalid states the Equation can end up
//! in. E.g. We expect that `operators.len() - 1 == nums.len()`, what if that condition is not met?
//! Then we can reach undefinded behaviour. We could introduce lots of checks all around our code
//! for that but there is always that chance that if we are not careful we could introduce that
//! undefinded behaviour.
//!
//! Instead I went with
//! ```rust
//! struct Equation {
//!     first_num: Option<isize>,
//!     other_nums: Vec<(Operator, isize)>
//! }
//! ```
//!
//! This works a lot better as it is now impossible to have a state of equation where there is
//! undefinded behaviour and all states are valid. `first_num` stores the first number in the
//! equation, this is separate from the `other_nums` as the first number should not have an
//! operator attached before it. It is also optional as it was the way I designed building equation
//! permutations, but I have defined that if the first num is None then the first number is 0 and
//! the other nums are continued to be read.
//!
//!
//! Finally the great thing about the way I created this was when part 2 came around it was super
//! simple to solve as all I had to do was add another variant to the enum for concattination, then
//! add a few lines here and there and define the behaviour of it in the eval function

use std::str::FromStr;

use derive_more::derive::Display;
use strum::{EnumIter, IntoEnumIterator};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        7
    }

    fn part1(&self, input: &str) -> String {
        let mut res = 0;

        for line in input.lines() {
            let input = InputLine::from_str(line).unwrap();
            let expected = input.expected_result;

            let eqs = input.get_equations(false);
            for eq in eqs {
                // println!("{eq} = {} | {expected}", eq.eval());
                if expected == eq.eval() {
                    res += expected;
                    // println!("Yes! {res}");
                    break;
                }
            }
        }
        res.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut res = 0;

        for line in input.lines() {
            let input = InputLine::from_str(line).unwrap();
            let expected = input.expected_result;

            let eqs = input.get_equations(true);
            for eq in eqs {
                // println!("{eq} = {} | {expected}", eq.eval());
                if expected == eq.eval() {
                    res += expected;
                    // println!("Yes! {res}");
                    break;
                }
            }
        }
        res.to_string()
    }
}

struct InputLine {
    expected_result: isize,
    numbers: Vec<isize>,
}

impl FromStr for InputLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect::<Vec<_>>();

        let [result, nums] = parts.as_slice() else {
            return Err(format!("Failed parsing line '{s}'"));
        };

        let nums: Vec<_> = nums
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Self {
            expected_result: result.parse().unwrap(),
            numbers: nums,
        })
    }
}

impl InputLine {
    pub fn get_equations(self, part2: bool) -> Vec<Equation> {
        let mut eqs = vec![Equation::default()];
        for num in self.numbers {
            let mut new_eqs = Vec::new();
            for eq in &eqs {
                new_eqs.append(&mut eq.add_num(part2, num));
            }
            eqs = new_eqs;
        }
        eqs
    }
}

#[derive(Debug, Default, Clone, Display)]
#[display(
    "{} {}",
    match first_num {
        Some(x) => x.to_string(),
        None => "0".into()
    },
    other_nums.iter().map(|(op, num)| format!("{op} {num}")).collect::<Vec<_>>().join(" "))
]
struct Equation {
    first_num: Option<isize>,
    other_nums: Vec<(Operator, isize)>,
}

impl Equation {
    pub fn add_num(&self, part2: bool, number: isize) -> Vec<Self> {
        if self.first_num.is_none() {
            let mut eq = self.clone();
            eq.first_num = Some(number);
            return vec![eq];
        }

        let mut eqs = Vec::new();
        for op in Operator::iter() {
            if op == Operator::Concat && !part2 {
                continue;
            }
            let mut eq = self.clone();
            eq.other_nums.push((op, number));
            eqs.push(eq);
        }
        eqs
    }

    pub fn eval(&self) -> isize {
        let mut res = self.first_num.unwrap_or_default();

        for (op, num) in self.other_nums.iter() {
            res = match op {
                Operator::Add => res + num,
                Operator::Multiply => res * num,
                Operator::Concat => (res.to_string() + &num.to_string()).parse().unwrap(),
            };
        }

        res
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Copy, Clone, Display)]
enum Operator {
    #[display("+")]
    Add,
    #[display("*")]
    Multiply,
    #[display("||")]
    Concat,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let results = Day.part1(INPUT);
        assert_eq!(results, "3749");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let results = Day.part2(INPUT);
        assert_eq!(results, "11387");
    }
}
