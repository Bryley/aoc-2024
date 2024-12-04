use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        2
    }

    fn part1(&self, input: &str) -> String {
        let result = input
            .lines()
            .map(|x| x.split_whitespace().map(|x| x.parse::<i64>().unwrap()))
            .filter(|iter| is_valid_part1(iter.clone()))
            .count();

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let result = input
            .lines()
            .map(|x| x.split_whitespace().map(|x| x.parse::<i64>().unwrap()))
            .filter(|iter| is_valid_part2(iter.clone()))
            .count();

        result.to_string()
    }
}

fn is_valid_part1<I: Iterator<Item = i64>>(mut x: I) -> bool {
    let mut is_negative = None;

    let Some(mut prev) = x.next() else {
        return true;
    };

    for curr in x {
        let distance = curr - prev;

        if distance.abs() > 3 || distance == 0 {
            return false;
        }
        let num_negative = distance < 0;
        // dbg!(&prev, &curr, &distance, &num_negative, &is_negative);
        match is_negative {
            None => {
                is_negative = Some(num_negative);
            }
            Some(is_negative) => {
                if is_negative != num_negative {
                    return false;
                }
            }
        }

        prev = curr;
    }
    true
}

fn is_valid_part2<I: Iterator<Item = i64>>(x: I) -> bool {
    let original: Vec<i64> = x.collect();

    if is_valid_part1(original.clone().into_iter()) {
        return true;
    }

    let mut permutations = Vec::new();
    for (index, _) in original.iter().enumerate() {
        let mut new_perm = original.clone();
        new_perm.remove(index);
        permutations.push(new_perm);
    }

    permutations.into_iter().any(|x| is_valid_part1(x.into_iter()))
}

#[cfg(test)]
mod tests {
    use super::{is_valid_part1, is_valid_part2};

    #[test]
    fn test_day2_part1() {
        let valid = is_valid_part1([48, 46, 47, 49, 51, 54, 56].into_iter());
        assert!(!valid);
    }

    #[test]
    fn test_day2_part2() {
        let valid = is_valid_part2([48, 46, 47, 49, 51, 54, 56].into_iter());
        assert!(valid);
    }
}
