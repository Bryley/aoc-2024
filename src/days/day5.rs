//! Day 5 was solved by implementing the Rust std library's `Ord` trait on a custom type I called
//! `Entry`.
//!
//! This was super useful for part 2 as it allowed me to sort the updates just by calling `.sort()`
//! on them and Rust took care of the rest.
//!
//! Also checking if the list was in the right order was as simple as calling `.is_sorted()` on the
//! updates.

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        5
    }

    fn part1(&self, input: &str) -> String {
        let parts = input.split("\n\n").collect::<Vec<_>>();
        let [ordering, updates] = parts.as_slice() else {
            panic!("Invalid input");
        };

        let ordering: OrderingMap = (*ordering).try_into().unwrap();

        let updates: Vec<_> = updates
            .lines()
            .map(|line| Updates::new(&ordering, line))
            .filter(|x| x.is_sorted())
            .collect();

        let middle_sum: u64 = updates.iter().map(Updates::middle_val).sum();

        middle_sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let parts = input.split("\n\n").collect::<Vec<_>>();
        let [ordering, updates] = parts.as_slice() else {
            panic!("Invalid input");
        };

        let ordering: OrderingMap = (*ordering).try_into().unwrap();

        let updates: Vec<_> = updates
            .lines()
            .map(|line| Updates::new(&ordering, line))
            .filter(|x| !x.is_sorted())
            .map(|mut updates| {
                updates.sort();
                updates
            })
            .collect();

        let middle_sum: u64 = updates.iter().map(Updates::middle_val).sum();

        middle_sum.to_string()
    }
}

/// Updates stores a list of Entries that can be sorted
struct Updates<'a>(Vec<Entry<'a>>);

impl<'a> Updates<'a> {
    pub fn new(ordering: &'a OrderingMap, line: &'a str) -> Self {
        let x = line
            .split(",")
            .map(|val| Entry::new(ordering, val))
            .collect::<Vec<_>>();
        Self(x)
    }

    pub fn middle_val(&self) -> u64 {
        let middle = self.len() / 2;

        let middle: &str = self.get(middle).unwrap().into();

        middle.parse::<u64>().unwrap()
    }
}

impl<'a> Deref for Updates<'a> {
    type Target = Vec<Entry<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Updates<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Entry represents an entry to be sorted with from the input
#[derive(Debug, Clone)]
struct Entry<'a> {
    ordering: &'a OrderingMap<'a>,
    value: &'a str,
}

impl<'a> Entry<'a> {
    pub fn new(ordering: &'a OrderingMap, value: &'a str) -> Self {
        Self { value, ordering }
    }
}

impl PartialEq for Entry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Entry<'_> {}

impl PartialOrd for Entry<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ordering.order(self.value, other.value) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl<'a> From<&Entry<'a>> for &'a str {
    fn from(value: &Entry<'a>) -> Self {
        value.value
    }
}

#[derive(Debug)]
struct OrderingMap<'a>(HashMap<&'a str, HashSet<&'a str>>);

/// Had to use `TryFrom` instead of `FromStr` because `FromStr` doesn't retain
/// lifetime data of the given string input
impl<'a> TryFrom<&'a str> for OrderingMap<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // Parse the ordering into a map
        let mut ordering_map: HashMap<&str, HashSet<&str>> = HashMap::new();
        for line in value.lines() {
            let parts = line.split("|").collect::<Vec<_>>();

            let [num1, num2] = parts.as_slice() else {
                return Err(format!("Invalid ordering {line}"));
            };

            let values = ordering_map.entry(num1).or_default();
            values.insert(num2);
        }

        Ok(Self(ordering_map))
    }
}

impl OrderingMap<'_> {
    pub fn order(&self, num1: &str, num2: &str) -> bool {
        let Some(values) = self.0.get(num1) else {
            return false;
        };

        values.contains(num2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = Day.part1(INPUT);

        assert_eq!(result, "143");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = Day.part2(INPUT);

        assert_eq!(result, "123");
    }
}
