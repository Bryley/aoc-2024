//! Day 9 I found was quite easy for day 1 and became quite difficult with part 2.
//!
//! As part 1 was quite slow I decided to rewrite my underlying datastructure to
//! solve part 2.
//!
//! I originally used a VecDeque<Option<usize>> (you can see this solution in one
//! of my previous commits). This was around 17 to 18 seconds for solving part1.
//! Which was decent but still fairly slow and made dealing with part 2 harder and slower.
//!
//! This is when I decided to change to what I have currently (BTreeSet<Chunk>). This was my first
//! time playing around with BTreeSet in Rust but was quite useful, it will act like a set but keep
//! all items in order. I then defined the order of chunks based off of their index.
//!
//! This actually worked really well for part 2 and can solve it in about 8 seconds or so.
//! I also wrote a part1 solution based on this datastructure and unfortunately it is much slower
//! than my first attempt taking over 2 mins. This is most likely due to having to break up the
//! chunks into aditional chunks instead of what I had previously which was 1 chunk per item in the
//! collection I used. Meaning it has to do additional work for each.
//!
//! If I were to come back I think the best solution to fix part 1's slowness is to try to eliminate aditional
//! work. For example maybe add a cache that will store where the last found free space was as it
//! will fill from left to right and not check before that? Also maybe store the data differently
//! like I did before for quicker lookup of the data using an index.

use std::{collections::BTreeSet, fmt::Display, str::FromStr};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        9
    }

    // TODO maybe come back and improve the performance of part1
    fn part1(&self, input: &str) -> String {
        let mut hd = Hardrive::from_str(input).unwrap();
        for id in (0..hd.len).rev() {
            hd.part1_fragment_once(id);
        }
        hd.checksum().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut hd = Hardrive::from_str(input).unwrap();
        for id in (0..(hd.last_id() + 1)).rev() {
            hd.part2_fragment_once(id);
        }
        hd.checksum().to_string()
    }
}

#[derive(Debug)]
struct Hardrive {
    // (Optional ID (None = Freespace), length)
    chunks: BTreeSet<Chunk>,
    pub len: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Chunk {
    index: usize,
    size: usize,
    id: usize,
}

impl PartialOrd for Chunk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Chunk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl Hardrive {
    pub fn find_free_space_index(&self, size: usize) -> Option<usize> {
        let mut iter = self.chunks.iter();
        let mut prev_chunk = iter.next()?;
        for curr_chunk in iter {
            let start_free = prev_chunk.index + prev_chunk.size;
            let free_space_size = curr_chunk.index - start_free;

            if free_space_size >= size {
                return Some(start_free);
            }

            prev_chunk = curr_chunk;
        }
        None
    }

    pub fn last_id(&self) -> usize {
        self.chunks.iter().next_back().unwrap().id
    }

    pub fn part1_fragment_once(&mut self, index: usize) {
        let Some(free_space_index) = self.find_free_space_index(1) else {
            return;
        };
        let Some(chunk) = self
            .chunks
            .iter()
            .rev()
            .find(|c| index < c.index + c.size && index >= c.index)
            .copied()
        else {
            return;
        };

        let mut chunk = self.chunks.take(&chunk).unwrap();

        let mut tiny_chunk = chunk.break_off_last();

        if free_space_index > tiny_chunk.index {
            // Leave chunk where it is
            if !chunk.is_empty() {
                self.chunks.insert(chunk);
            }
            self.chunks.insert(tiny_chunk);
            return;
        }

        tiny_chunk.index = free_space_index;
        if !chunk.is_empty() {
            self.chunks.insert(chunk);
        }
        self.chunks.insert(tiny_chunk);
    }

    pub fn part2_fragment_once(&mut self, id: usize) {
        let chunk = *self.chunks.iter().find(|c| c.id == id).unwrap();
        let free_space_index = self.find_free_space_index(chunk.size);

        let mut chunk = self.chunks.take(&chunk).unwrap();

        let Some(free_index) = free_space_index else {
            // Insert chunk back in same place
            self.chunks.insert(chunk);
            return;
        };

        if free_index > chunk.index {
            // Leave chunk where it is
            self.chunks.insert(chunk);
            return;
        }

        chunk.index = free_index;
        self.chunks.insert(chunk);
    }

    pub fn checksum(&self) -> usize {
        let mut sum = 0;
        for chunk in &self.chunks {
            sum += chunk.checksum();
        }
        sum
    }
}

impl Chunk {
    pub fn checksum(&self) -> usize {
        let mut sum = 0;
        for index in self.index..(self.size + self.index) {
            sum += index * self.id;
        }
        sum
    }

    pub fn break_off_last(&mut self) -> Chunk {
        self.size -= 1;

        Chunk {
            index: self.index + self.size,
            size: 1,
            id: self.id,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl FromStr for Hardrive {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks_index_counter = 0;
        let mut chunks = BTreeSet::new();
        for (index, c) in s.trim().chars().enumerate() {
            let occupy = index % 2 == 0;

            let size: usize = c
                .to_string()
                .parse()
                .map_err(|_| format!("Failed to convert digit '{c}'"))?;

            if !occupy {
                chunks_index_counter += size;
                continue;
            }

            let id = index / 2;

            chunks.insert(Chunk {
                index: chunks_index_counter,
                size,
                id,
            });
            chunks_index_counter += size;
        }

        Ok(Self {
            chunks,
            len: chunks_index_counter,
        })
    }
}

impl Display for Hardrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        let mut index_counter = 0;

        for chunk in &self.chunks {
            let free_space = chunk.index - index_counter;
            s.push_str(&".".repeat(free_space));
            index_counter = chunk.index + chunk.size;

            let id_char = chunk.id.to_string().chars().next_back().unwrap();

            s.push_str(&id_char.to_string().repeat(chunk.size));
        }

        let free_space = self.len - index_counter;
        s.push_str(&".".repeat(free_space));

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
