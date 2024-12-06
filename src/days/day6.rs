use std::{
    collections::HashSet,
    fmt::{Display, Write},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

use crate::AdventOfCodeDay;

pub struct Day;

impl AdventOfCodeDay for Day {
    fn day(&self) -> u8 {
        6
    }

    fn part1(&self, input: &str) -> String {
        let mut layout = Layout::from_str(input).unwrap();

        // Step through
        while layout.step() {}

        layout.visited_squares().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let layout = Layout::from_str(input).unwrap();

        // let mut threads = Vec::with_capacity(10);

        let mut success = 0;
        for row in 0..layout.height {
            for col in 0..layout.width {
                // let layout_clone = layout.clone();
                // TODO fix this code
                if !is_finite(layout.clone(), (row, col)) {
                    success += 1;
                }
                // let handle = thread::spawn(move || (!is_finite(layout_clone, (row, col))) as isize);
                // let t = thread::spawn(move || {
                //     if !is_finite(&layout, (row, col)) {
                //         let arc = Arc::clone(&success);
                //         let mut success = arc.lock().unwrap();
                //         *success += 1;
                //     }
                // });
                // threads.push(handle);
                //
                // if threads.len() >= 10 {
                //     for t in threads.drain(..) {
                //         success += t.join().unwrap();
                //     }
                //     threads.clear();
                // }
            }
        }
        // for t in threads.drain(..) {
        //     success += t.join().unwrap();
        // }
        // threads.clear();

        success.to_string()
    }
}

fn is_finite(mut layout: Layout, (row, col): (isize, isize)) -> bool {
    if row == layout.character.0 && col == layout.character.1 {
        return true;
    }
    if layout.obstacle_at(row, col) {
        return true;
    }
    layout.extra_obstacle = Some((row, col));

    // Step through
    while layout.step() {
        if layout.has_visited() {
            // success += 1;
            return true;
        }
    }
    false
}

#[derive(Debug, Clone)]
struct Layout {
    pub obstacles: HashSet<(isize, isize)>,
    original_character: (isize, isize),
    character: (isize, isize),
    character_direction: Direction,
    pub width: isize,
    pub height: isize,
    pub visited: HashSet<(isize, isize, Direction)>,
    pub extra_obstacle: Option<(isize, isize)>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn go(&self, row: isize, col: isize) -> (isize, isize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Layout {
    pub fn clear(&mut self) {
        self.extra_obstacle = None;
        self.visited.clear();
        self.character = self.original_character;
        self.character_direction = Direction::Up;
    }

    pub fn obstacle_at(&self, row: isize, col: isize) -> bool {
        if let Some((r, c)) = self.extra_obstacle {
            if row == r && col == c {
                return true;
            }
        }
        self.obstacles.contains(&(row, col))
    }

    pub fn save_char(&mut self) {
        self.visited
            .insert((self.character.0, self.character.1, self.character_direction));
    }

    pub fn has_visited(&self) -> bool {
        self.visited
            .contains(&(self.character.0, self.character.1, self.character_direction))
    }

    pub fn is_out_of_bounds(&self) -> bool {
        let (row, col) = self.character;

        row < 0 || col < 0 || row >= self.height || col >= self.width
    }
    pub fn move_char(&mut self, row: isize, col: isize) {
        self.character = (row, col);
    }

    pub fn visited_squares(&self) -> usize {
        self.visited
            .iter()
            .map(|(row, col, _)| (row, col))
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn step(&mut self) -> bool {
        if self.is_out_of_bounds() {
            return false;
        }

        self.save_char();
        let (row, col) = self.character;
        let (next_row, next_col) = self.character_direction.go(row, col);

        if self.obstacle_at(next_row, next_col) {
            self.character_direction = self.character_direction.turn_right();
            return true;
        }
        self.move_char(next_row, next_col);

        true
    }

    pub fn get_symbol(&self, row: isize, col: isize) -> char {
        if self.obstacles.contains(&(row, col)) {
            return '#';
        }
        if self.character == (row, col) {
            return match self.character_direction {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            };
        }

        if self.extra_obstacle == Some((row, col)) {
            return 'O';
        }

        if (self.visited.contains(&(row, col, Direction::Up))
            || self.visited.contains(&(row, col, Direction::Down)))
            && (self.visited.contains(&(row, col, Direction::Left))
                || self.visited.contains(&(row, col, Direction::Right)))
        {
            return '+';
        }
        if self.visited.contains(&(row, col, Direction::Up))
            || self.visited.contains(&(row, col, Direction::Down))
        {
            return '|';
        }
        if self.visited.contains(&(row, col, Direction::Left))
            || self.visited.contains(&(row, col, Direction::Right))
        {
            return '-';
        }

        '.'
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut results = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let s = self.get_symbol(row, col);
                results.write_str(&format!("{s} "))?;
            }
            results.write_str("\n")?;
        }
        write!(f, "{results}")
    }
}

impl FromStr for Layout {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut character = None;
        let mut obstacles = HashSet::new();

        let mut height = 0;
        let mut width = 0;

        for (row, line) in s.lines().enumerate() {
            let mut inner_width = 0;
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert((row as isize, col as isize));
                    }
                    '^' => {
                        character = Some((row as isize, col as isize));
                    }
                    _ => (),
                }
                inner_width += 1;
            }
            height += 1;
            width = inner_width;
        }

        let character = character.ok_or("No character found in input".to_string())?;

        let visited = HashSet::new();
        // visited.insert((character.0, character.1, Direction::Up));

        Ok(Self {
            obstacles,
            original_character: character,
            character,
            character_direction: Direction::Up,
            width,
            height,
            visited,
            extra_obstacle: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = Day.part1(INPUT);

        assert_eq!(result, "41");
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = Day.part2(INPUT);

        assert_eq!(result, "6");
    }
}
