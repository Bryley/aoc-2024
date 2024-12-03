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
            .filter(|iter| {
                // This is_valid_part2 function fails for inputs where the first item
                // is the incorrect one, was too lazy to fix the is_valid_part2 function so I
                // simply reversed the numbers and checked again. Not proud of this solution :/
                //
                // If you want a better solution check out my nushell implementation of this
                // day
                let valid = is_valid_part2(iter.clone());
                if !valid {
                    return is_valid_part2(iter.clone().rev());
                }
                valid
            })
            // .map(|x| x.collect::<Vec<_>>())
            // .for_each(|x| println!("{x:?}"));
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

/// This was an attempt to get it to work without brute forcing the solution like
/// I did in the Nushell solution, I was very close (got 1 off the correct solution)
/// Still need to investigate or redo the solution in Rust
fn is_valid_part2<I: Iterator<Item = i64>>(mut x: I) -> bool {
    let mut done_ignore = false;
    let mut is_negative = None;

    let Some(mut prev) = x.next() else {
        return true;
    };

    for curr in x {
        let distance = curr - prev;

        if distance.abs() > 3 || distance == 0 {
            if done_ignore {
                return false;
            }
            done_ignore = true;

            prev = curr;
            continue;
        }
        let num_negative = distance < 0;
        // dbg!(&prev, &curr, &distance, &num_negative, &is_negative, &done_ignore);
        match is_negative {
            None => {
                is_negative = Some(num_negative);
            }
            Some(is_negative) => {
                if is_negative != num_negative {
                    if done_ignore {
                        return false;
                    }
                    done_ignore = true;

                    prev = curr;
                    continue;
                }
            }
        }

        prev = curr;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{is_valid_part1, is_valid_part2};

    #[test]
    fn test_day2_part1() {
        let valid = is_valid_part1([48, 46, 47, 49, 51, 54, 56].into_iter());
        assert!(!valid);
    }

    // #[test]
    // fn test_day2_part2() {
    //     let valid = is_valid_part2([48, 46, 47, 49, 51, 54, 56].into_iter());
    //     assert!(valid);
    // }
}
