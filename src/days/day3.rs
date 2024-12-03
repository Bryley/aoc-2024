use regex::Regex;

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        3
    }

    fn part1(&self, input: &str) -> String {
        let regex = Regex::new(r#"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)"#).unwrap();

        let mut total = 0;
        for capture in regex.captures_iter(input) {
            let num1 = capture.name("num1").unwrap().as_str();
            let num2 = capture.name("num2").unwrap().as_str();

            total += num1.parse::<u64>().unwrap() * num2.parse::<u64>().unwrap();
        }

        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let regex =
            Regex::new(r#"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)|(?P<do>do\(\)|don't\(\))"#)
                .unwrap();

        let mut is_ignore = false;
        let mut total = 0;
        for capture in regex.captures_iter(input) {
            if let Some(do_val) = capture.name("do").map(|x| x.as_str()) {
                match do_val {
                    "do()" => {
                        is_ignore = false;
                        continue;
                    }
                    "don't()" => {
                        is_ignore = true;
                        continue;
                    }
                    _ => (),
                }
            };

            if is_ignore {
                continue;
            }

            let num1 = capture.name("num1").unwrap().as_str();
            let num2 = capture.name("num2").unwrap().as_str();

            total += num1.parse::<u64>().unwrap() * num2.parse::<u64>().unwrap();
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result =
            Day.part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, "161".to_owned())
    }

    #[test]
    fn test_part2() {
        let result =
            Day.part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, "48".to_owned())
    }
}
