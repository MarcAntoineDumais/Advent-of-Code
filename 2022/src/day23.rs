use std::{collections::VecDeque, fs};

use crate::utils::Point2D;

pub fn part1() {
    run(Some(10));
}

pub fn part2() {
    run(None);
}

fn run(max_round: Option<i32>) {
    let data = fs::read_to_string("data/day23.txt").unwrap();
    let mut grid = Grid::new();
    let mut elves = Vec::new();
    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                grid.set(Point2D(col as i32, row as i32), 1);
                elves.push(Point2D(col as i32, row as i32));
            }
        }
    }

    let mut first_direction = 0;
    let mut round = 1;
    loop {
        // println!("new round");
        // grid.draw();
        let mut proposal_counts = Grid::new();
        let mut proposals = Vec::new();
        for (elf_index, pos) in elves.iter().enumerate() {
            if !grid.should_move(*pos) {
                continue;
            }
            if let Some(new_pos) = grid.get_proposal(pos, first_direction) {
                proposal_counts.set(new_pos, proposal_counts.get(new_pos) + 1);
                proposals.push((elf_index, pos.clone(), new_pos));
            }
        }

        let mut moved = false;
        for (elf_index, pos, new_pos) in proposals {
            if proposal_counts.get(new_pos) == 1 {
                elves[elf_index] = new_pos;
                grid.set(pos, 0);
                grid.set(new_pos, 1);
                moved = true;
            }
        }
        first_direction += 1;
        round += 1;

        match max_round {
            Some(max) => {
                if round > max {
                    break;
                }
            }
            None => {
                if !moved {
                    round -= 1;
                    break;
                }
            }
        }
    }

    if max_round == None {
        println!("{}", round);
    } else {
        grid.shrink();
        println!("{}", grid.count_empty_tiles());
    }
}

#[derive(Clone)]
struct Grid {
    grid: VecDeque<VecDeque<i32>>,
    top_left: Point2D,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            grid: VecDeque::new(),
            top_left: Point2D(0, 0),
        }
    }

    fn get(&self, pos: Point2D) -> i32 {
        if pos.1 < self.top_left.1
            || pos.1 >= self.top_left.1 + self.grid.len() as i32
            || pos.0 < self.top_left.0
            || pos.0 >= self.top_left.0 + self.grid[0].len() as i32
        {
            return 0;
        }

        self.grid[(pos.1 - self.top_left.1) as usize][(pos.0 - self.top_left.0) as usize]
    }

    fn set(&mut self, pos: Point2D, value: i32) {
        let mut width = match self.grid.len() {
            0 => 0,
            _ => self.grid[0].len(),
        };
        for _ in pos.1..self.top_left.1 {
            self.grid.push_front(VecDeque::from(vec![0; width]));
        }
        self.top_left.1 = self.top_left.1.min(pos.1);
        for _ in (self.top_left.1 + self.grid.len() as i32)..=pos.1 {
            self.grid.push_back(VecDeque::from(vec![0; width]));
        }

        for _ in pos.0..self.top_left.0 {
            for row in 0..self.grid.len() {
                self.grid[row].push_front(0);
            }
        }
        self.top_left.0 = self.top_left.0.min(pos.0);
        width = match self.grid.len() {
            0 => 0,
            _ => self.grid[0].len(),
        };
        for _ in (self.top_left.0 + width as i32)..=pos.0 {
            for row in 0..self.grid.len() {
                self.grid[row].push_back(0);
            }
        }

        self.grid[(pos.1 - self.top_left.1) as usize][(pos.0 - self.top_left.0) as usize] = value;
    }

    fn shrink(&mut self) {
        let (mut min_y, mut max_y, mut min_x, mut max_x) = (None, None, None, None);
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if self.grid[row][col] > 0 {
                    min_y = Some((self.top_left.1 + row as i32).min(match min_y {
                        Some(x) => x,
                        None => i32::MAX,
                    }));
                    min_x = Some((self.top_left.0 + col as i32).min(match min_x {
                        Some(x) => x,
                        None => i32::MAX,
                    }));
                    max_y = Some((self.top_left.1 + row as i32).max(match max_y {
                        Some(x) => x,
                        None => i32::MIN,
                    }));
                    max_x = Some((self.top_left.0 + col as i32).max(match max_x {
                        Some(x) => x,
                        None => i32::MIN,
                    }));
                }
            }
        }

        if min_y == None {
            self.grid.clear();
            return;
        }

        // Actual shrinking
        for _ in (max_y.unwrap() - self.top_left.1 + 1)..(self.grid.len() as i32) {
            self.grid.pop_back();
        }
        for _ in 0..(min_y.unwrap() - self.top_left.1) {
            self.grid.pop_front();
        }
        for _ in (max_x.unwrap() - self.top_left.0 + 1)..(self.grid[0].len() as i32) {
            for row in 0..self.grid.len() {
                self.grid[row].pop_back();
            }
        }
        for _ in 0..(min_x.unwrap() - self.top_left.0) {
            for row in 0..self.grid.len() {
                self.grid[row].pop_front();
            }
        }

        self.top_left = Point2D(min_x.unwrap(), min_y.unwrap());
    }

    fn count_empty_tiles(&self) -> i32 {
        let mut count = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                if self.grid[row][col] == 0 {
                    count += 1;
                }
            }
        }
        count
    }

    fn should_move(&self, pos: Point2D) -> bool {
        self.get(Point2D(pos.0 - 1, pos.1)) > 0
            || self.get(Point2D(pos.0 - 1, pos.1 - 1)) > 0
            || self.get(Point2D(pos.0 - 1, pos.1 + 1)) > 0
            || self.get(Point2D(pos.0, pos.1 - 1)) > 0
            || self.get(Point2D(pos.0, pos.1 + 1)) > 0
            || self.get(Point2D(pos.0 + 1, pos.1)) > 0
            || self.get(Point2D(pos.0 + 1, pos.1 - 1)) > 0
            || self.get(Point2D(pos.0 + 1, pos.1 + 1)) > 0
    }

    fn get_proposal(&self, pos: &Point2D, direction: i32) -> Option<Point2D> {
        for i in 0..4 {
            if let Some(new_pos) = self.is_proposal_valid(pos, direction + i) {
                return Some(new_pos);
            }
        }
        return None;
    }

    fn is_proposal_valid(&self, pos: &Point2D, direction: i32) -> Option<Point2D> {
        match direction % 4 {
            0 => match self.get(Point2D(pos.0, pos.1 - 1)) == 0
                && self.get(Point2D(pos.0 - 1, pos.1 - 1)) == 0
                && self.get(Point2D(pos.0 + 1, pos.1 - 1)) == 0
            {
                true => Some(Point2D(pos.0, pos.1 - 1)),
                false => None,
            },
            1 => match self.get(Point2D(pos.0, pos.1 + 1)) == 0
                && self.get(Point2D(pos.0 - 1, pos.1 + 1)) == 0
                && self.get(Point2D(pos.0 + 1, pos.1 + 1)) == 0
            {
                true => Some(Point2D(pos.0, pos.1 + 1)),
                false => None,
            },
            2 => match self.get(Point2D(pos.0 - 1, pos.1 - 1)) == 0
                && self.get(Point2D(pos.0 - 1, pos.1)) == 0
                && self.get(Point2D(pos.0 - 1, pos.1 + 1)) == 0
            {
                true => Some(Point2D(pos.0 - 1, pos.1)),
                false => None,
            },
            3 => match self.get(Point2D(pos.0 + 1, pos.1 - 1)) == 0
                && self.get(Point2D(pos.0 + 1, pos.1)) == 0
                && self.get(Point2D(pos.0 + 1, pos.1 + 1)) == 0
            {
                true => Some(Point2D(pos.0 + 1, pos.1)),
                false => None,
            },
            _ => panic!("impossible modulo"),
        }
    }

    fn draw(&self) {
        for row in self.grid.iter() {
            for val in row {
                print!(
                    "{}",
                    match *val {
                        0 => ".",
                        _ => "#",
                    }
                )
            }
            println!();
        }
        println!();
    }
}
