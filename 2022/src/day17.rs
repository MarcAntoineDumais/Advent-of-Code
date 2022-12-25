use std::{collections::HashMap, fs};

use crate::utils::Point2Di64;

pub fn part1() {
    let sequence = read_sequence();

    let mut grid = Grid::new();
    let mut shape = RockShape::LineHorizontal;
    let mut sequence_it = sequence.iter().peekable();
    for z in 0..2022 {
        if z % 10000 == 0 {
            println!("{z}");
        }
        let mut rock = Rock::new(shape, Point2Di64(2, grid.highest() + 3));
        loop {
            if sequence_it.peek() == None {
                sequence_it = sequence.iter().peekable();
            }
            // println!("After fall");
            // draw(&grid, &rock);
            grid.push(&mut rock, *sequence_it.next().unwrap());
            // println!("After push");
            // draw(&grid, &rock);
            if grid.fall(&mut rock) {
                break;
            }
        }
        shape = shape.next();
    }
    println!("{}", grid.highest());
}

pub fn part2() {
    let sequence = read_sequence();

    let mut patterns = HashMap::new();

    let mut grid = Grid::new();
    let mut shape = RockShape::LineHorizontal;
    let mut sequence_it = sequence.iter().enumerate().peekable();
    let mut jumped = false;
    let mut sequence_complete = false;
    let n_rocks = 1000000000000i64;
    let mut rock_index = 0i64;
    while rock_index < n_rocks {
        if rock_index % 10000 == 0 && rock_index > 0 {
            println!("{rock_index}");
        }

        // Cycle detection
        let seq_peek = sequence_it.peek().map(|(i, _)| *i);
        if !jumped && sequence_complete && seq_peek != None {
            let seq_index = seq_peek.unwrap();
            let highest = grid.highest();
            let pattern_key = (grid.rows.clone(), seq_index, shape);

            if patterns.contains_key(&pattern_key) {
                let (old_highest, old_rock_index) = patterns[&pattern_key];
                let period_height = highest - old_highest;
                let period_rocks = rock_index - old_rock_index;
                let n_periods = (n_rocks - rock_index) / period_rocks;
                let jump_rocks = n_periods * period_rocks;
                let jump_height = n_periods * period_height;

                println!("Skipping {jump_rocks} rocks (period {period_rocks}) and {jump_height} height (period {period_height})");
                grid.bottom += jump_height;

                rock_index += jump_rocks;
                jumped = true;
            } else {
                patterns.insert(pattern_key, (highest, rock_index));
            }
        }

        let mut rock = Rock::new(shape, Point2Di64(2, grid.highest() + 3));
        loop {
            if sequence_it.peek() == None {
                sequence_it = sequence.iter().enumerate().peekable();
                sequence_complete = true;
            }
            let (_, left) = sequence_it.next().unwrap();
            //println!("After fall");
            //draw(&grid, &rock);

            grid.push(&mut rock, *left);
            //println!("After push");
            //draw(&grid, &rock);
            if grid.fall(&mut rock) {
                break;
            }
        }
        shape = shape.next();
        rock_index += 1;
    }
    println!("{}", grid.highest());
}

fn read_sequence() -> Vec<bool> {
    fs::read_to_string("data/day17.txt")
        .unwrap()
        .chars()
        .map(|c| c == '<')
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum RockShape {
    LineHorizontal,
    Plus,
    Corner,
    LineVertical,
    Square,
}

impl RockShape {
    fn next(&self) -> RockShape {
        match self {
            RockShape::LineHorizontal => RockShape::Plus,
            RockShape::Plus => RockShape::Corner,
            RockShape::Corner => RockShape::LineVertical,
            RockShape::LineVertical => RockShape::Square,
            RockShape::Square => RockShape::LineHorizontal,
        }
    }
}

#[derive(Clone, Copy)]
struct Rock {
    shape: RockShape,
    pos: Point2Di64,
}

impl Rock {
    fn new(shape: RockShape, pos: Point2Di64) -> Rock {
        Rock { shape, pos }
    }

    fn positions(&self) -> Vec<Point2Di64> {
        let mut points = Vec::new();
        match self.shape {
            RockShape::LineHorizontal => {
                points.push(self.pos);
                points.push(self.pos.add(&Point2Di64(1, 0)));
                points.push(self.pos.add(&Point2Di64(2, 0)));
                points.push(self.pos.add(&Point2Di64(3, 0)));
            }
            RockShape::Plus => {
                points.push(self.pos.add(&Point2Di64(1, 0)));
                points.push(self.pos.add(&Point2Di64(0, 1)));
                points.push(self.pos.add(&Point2Di64(1, 1)));
                points.push(self.pos.add(&Point2Di64(2, 1)));
                points.push(self.pos.add(&Point2Di64(1, 2)));
            }
            RockShape::Corner => {
                points.push(self.pos);
                points.push(self.pos.add(&Point2Di64(1, 0)));
                points.push(self.pos.add(&Point2Di64(2, 0)));
                points.push(self.pos.add(&Point2Di64(2, 1)));
                points.push(self.pos.add(&Point2Di64(2, 2)));
            }
            RockShape::LineVertical => {
                points.push(self.pos);
                points.push(self.pos.add(&Point2Di64(0, 1)));
                points.push(self.pos.add(&Point2Di64(0, 2)));
                points.push(self.pos.add(&Point2Di64(0, 3)));
            }
            RockShape::Square => {
                points.push(self.pos);
                points.push(self.pos.add(&Point2Di64(1, 0)));
                points.push(self.pos.add(&Point2Di64(0, 1)));
                points.push(self.pos.add(&Point2Di64(1, 1)));
            }
        }
        points
    }
}

struct Grid {
    bottom: i64,
    rows: Vec<[bool; 7]>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            bottom: 0,
            rows: Vec::new(),
        }
    }

    fn occupied(&self, pos: &Point2Di64) -> bool {
        if pos.0 < 0 || pos.0 >= 7 || pos.1 < self.bottom {
            return true;
        }
        if (pos.1 - self.bottom) >= self.rows.len() as i64 {
            return false;
        }
        return self.rows[(pos.1 - self.bottom) as usize][pos.0 as usize];
    }

    fn occupied_rock(&self, rock: &Rock) -> bool {
        for pos in rock.positions() {
            if self.occupied(&pos) {
                return true;
            }
        }
        return false;
    }

    fn mark(&mut self, pos: &Point2Di64) {
        if pos.0 < 0 || pos.0 >= 7 || pos.1 < 0 {
            return;
        }
        for _ in 0..((pos.1 - self.bottom) - self.rows.len() as i64 + 1) {
            self.rows.push([false; 7]);
        }
        self.rows[(pos.1 - self.bottom) as usize][pos.0 as usize] = true;
    }

    fn mark_rock(&mut self, rock: &Rock) {
        for pos in rock.positions() {
            self.mark(&pos);
        }

        let mut bottoms = [None; 7];
        for (y, row) in self.rows.iter().enumerate().rev() {
            for x in 0..7 {
                if bottoms[x] == None && row[x] {
                    bottoms[x] = Some(y);
                }
            }
        }
        if bottoms.iter().any(|b| *b == None) {
            return;
        }
        let rows_to_remove = bottoms.iter().map(|b| b.unwrap() + 1).min().unwrap() as i64;
        for _ in 0..rows_to_remove {
            self.rows.remove(0);
        }
        self.bottom += rows_to_remove;
    }

    fn highest(&self) -> i64 {
        if self.rows.is_empty() {
            return self.bottom;
        }
        for (i, row) in self.rows.iter().enumerate().rev() {
            for b in row {
                if *b {
                    return self.bottom + (i + 1) as i64;
                }
            }
        }
        return self.bottom;
    }

    fn fall(&mut self, rock: &mut Rock) -> bool {
        let moved_rock = Rock::new(rock.shape, rock.pos.add(&Point2Di64(0, -1)));
        if self.occupied_rock(&moved_rock) {
            self.mark_rock(&rock);
            return true;
        }
        rock.pos.1 -= 1;
        return false;
    }

    fn push(&self, rock: &mut Rock, left: bool) {
        let displacement = match left {
            true => -1,
            false => 1,
        };
        let moved_rock = Rock::new(rock.shape, rock.pos.add(&Point2Di64(displacement, 0)));
        if self.occupied_rock(&moved_rock) {
            return;
        }

        rock.pos.0 += displacement;
    }
}

fn draw(grid: &Grid, rock: &Rock) {
    let mut rows = grid.rows.clone();
    for _ in 0..7 {
        rows.push([false; 7]);
    }

    let mut chars: Vec<_> = rows
        .iter()
        .map(|row| {
            let mut row_chars = ["."; 7];
            for x in 0..7 {
                if row[x] {
                    row_chars[x] = "#";
                }
            }
            row_chars
        })
        .collect();

    for pos in rock.positions() {
        chars[pos.1 as usize][pos.0 as usize] = "@";
    }
    for row in chars.iter().rev() {
        print!("|");
        print!("{}", row.join(""));
        println!("|");
    }
    println!("+-------+");
}
