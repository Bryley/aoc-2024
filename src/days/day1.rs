use std::collections::HashMap;

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        1
    }

    fn part1(&self, input: &str) -> String {
        let mut nums1: Vec<i64> = Vec::new();
        let mut nums2: Vec<i64> = Vec::new();

        for line in input.lines() {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let [num1, num2] = parts.as_slice() else {
                unreachable!("input was invalid format");
            };
            nums1.push(num1.parse().unwrap());
            nums2.push(num2.parse().unwrap());
        }

        nums1.sort();
        nums2.sort();

        let result: i64 = nums1
            .into_iter()
            .zip(nums2)
            .map(|(num1, num2)| (num1 - num2).abs())
            .sum();

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut nums1: Vec<i64> = Vec::new();
        let mut nums2: HashMap<i64, i64> = HashMap::new();

        for line in input.lines() {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let [num1, num2] = parts.as_slice() else {
                unreachable!("input was invalid format");
            };
            let num1 = num1.parse().unwrap();
            let num2 = num2.parse().unwrap();
            nums1.push(num1);

            let count = nums2.entry(num2).or_insert(0);
            *count += 1;
        }

        let total = nums1.into_iter().fold(0, |current, num| {
            let amount = nums2.get(&num).unwrap_or(&0);
            current + (num * amount)
        });

        total.to_string()
    }
}
