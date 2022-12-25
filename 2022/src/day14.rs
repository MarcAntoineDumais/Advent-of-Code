use std::{collections::VecDeque, fs};

use crate::utils::Point2D;

pub fn run() {
    let data = fs::read_to_string("data/day14.txt").unwrap();
    let paths: Vec<Vec<Point2D>> = data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_path)
        .collect();

    let mut lowest = None;
    let mut grid = Grid {
        grid: VecDeque::new(),
        leftmost: None,
        lowest: None,
    };
    for path in paths {
        if path.len() < 2 {
            panic!("path too short");
        }
        for i in 1..path.len() {
            for step in path[i - 1].path_to(path[i]) {
                grid.mark(step, true);
                lowest = match lowest {
                    Some(prev) => {
                        if step.1 > prev {
                            Some(step.1)
                        } else {
                            Some(prev)
                        }
                    }
                    None => Some(step.1),
                }
            }
        }
    }

    lowest = match lowest {
        Some(floor) => Some(floor + 2),
        None => Some(2),
    };
    grid.lowest = lowest;

    let mut fallen_sand = 0;
    let sand_origin = Point2D(500, 0);
    let mut sand = sand_origin.clone();
    loop {
        let mut new_pos = Point2D(sand.0, sand.1 + 1);
        if grid.occupied(sand_origin) {
            break;
        }
        if new_pos.1 > lowest.unwrap() {
            break;
        }
        if grid.occupied(new_pos) {
            new_pos.0 -= 1;
        }
        if grid.occupied(new_pos) {
            new_pos.0 += 2;
        }
        if grid.occupied(new_pos) {
            grid.mark(sand, true);
            fallen_sand += 1;
            sand = sand_origin.clone();
        } else {
            sand = new_pos;
        }
    }
    println!("{fallen_sand}");
}

impl Point2D {
    fn path_to(&self, end: Point2D) -> Vec<Point2D> {
        let mut increment = Point2D(end.0 - self.0, end.1 - self.1);
        if increment.0 > 1 {
            increment.0 = 1;
        }
        if increment.0 < -1 {
            increment.0 = -1;
        }
        if increment.1 > 1 {
            increment.1 = 1;
        }
        if increment.1 < -1 {
            increment.1 = -1;
        }
        if increment.0.abs() + increment.1.abs() > 1 {
            panic!("can't handle diagonal paths");
        }
        let mut path = vec![self.clone()];
        let mut cur = self.clone();
        loop {
            cur.0 += increment.0;
            cur.1 += increment.1;
            path.push(cur.clone());
            if cur == end {
                break;
            }
        }
        path
    }
}

struct Grid {
    grid: VecDeque<VecDeque<bool>>,
    leftmost: Option<i32>,
    lowest: Option<i32>,
}

impl Grid {
    fn occupied(&self, point: Point2D) -> bool {
        if point.1 < 0 {
            return false;
        }
        if let Some(floor) = self.lowest {
            if point.1 >= floor {
                return true;
            }
        }
        if point.1 > self.grid.len() as i32 - 1 {
            return false;
        }
        match self.leftmost {
            Some(left) => {
                if point.0 < left {
                    false
                } else if point.0 > self.rightmost().unwrap() {
                    false
                } else {
                    self.grid[point.1 as usize][(point.0 - left) as usize]
                }
            }
            None => false,
        }
    }

    fn mark(&mut self, point: Point2D, value: bool) {
        if self.occupied(point) == value {
            return;
        }
        if point.1 < 0 {
            panic!("negative y coord is invalid");
        }
        if point.1 > self.grid.len() as i32 - 1 {
            let width = match self.grid.is_empty() {
                true => 0,
                false => self.grid[0].len(),
            };
            let cur_height = self.grid.len() as i32;
            for _ in 0..(point.1 - cur_height + 1) {
                let mut new_row = VecDeque::new();
                for _ in 0..width {
                    new_row.push_back(false);
                }
                self.grid.push_back(new_row);
            }
        }
        match self.leftmost {
            Some(left) => {
                let missing_width = left - point.0;
                for _ in 0..missing_width {
                    for row in &mut self.grid {
                        row.push_front(false);
                    }
                }
                let missing_width = point.0 - self.rightmost().unwrap();
                for _ in 0..missing_width {
                    for row in &mut self.grid {
                        row.push_back(false);
                    }
                }
                if point.0 < left {
                    self.leftmost = Some(point.0);
                }
            }
            None => {
                for row in &mut self.grid {
                    row.push_back(false);
                }
                self.leftmost = Some(point.0);
            }
        }

        self.grid[point.1 as usize][(point.0 - self.leftmost.unwrap()) as usize] = value;
    }

    fn rightmost(&self) -> Option<i32> {
        match self.leftmost {
            Some(l) => Some(l + self.grid[0].len() as i32 - 1),
            None => None,
        }
    }
}

fn parse_path(path: &str) -> Vec<Point2D> {
    path.split(" -> ")
        .map(|point| {
            let mut coords = point.split(",");
            Point2D(
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}
