use std::fs;

use crate::utils::{Orientation, Point2D};

pub fn part1() {
    run(false);
}

pub fn part2() {
    run(true);
}

fn run(is_cube: bool) {
    let (grid, instructions, starting_pos) = parse_input();

    let mut orientation = Orientation::Right;
    let mut pos = starting_pos.clone();
    for ele in instructions {
        match ele {
            Instruction::Straight(n) => {
                (pos, orientation) = pos.step_repeat(orientation, &grid, n, is_cube)
            }
            Instruction::RotateCW => orientation = orientation.rotate_cw(),
            Instruction::RotateCCW => orientation = orientation.rotate_ccw(),
        }
    }

    let password = (pos.1 + 1) * 1000
        + (pos.0 + 1) * 4
        + match orientation {
            Orientation::Right => 0,
            Orientation::Up => 3,
            Orientation::Left => 2,
            Orientation::Down => 1,
        };
    println!("{password}");
}

fn parse_input() -> (Vec<Vec<char>>, Vec<Instruction>, Point2D) {
    let data = fs::read_to_string("data/day22.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let height = lines.len() - 2;
    let mut width = 0;
    for i in 0..height {
        width = width.max(lines[i].chars().count());
    }

    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    for y in 0..height {
        lines[y].chars().enumerate().for_each(|(x, c)| match c {
            '.' | '#' => grid[y][x] = c,
            _ => (),
        });
    }

    let mut starting_pos = Point2D(0, 0);
    for (x, c) in lines[0].chars().enumerate() {
        if c == '.' {
            starting_pos.0 = x as i32;
            break;
        }
    }

    let instructions = parse_instructions(lines[lines.len() - 1]);
    (grid, instructions, starting_pos)
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut chars = line.chars();
    let mut instructions = Vec::new();

    let mut substring = String::new();
    loop {
        let c_opt = chars.next();
        if c_opt == None || !c_opt.unwrap().is_digit(10) {
            instructions.push(Instruction::Straight(substring.parse::<i32>().unwrap()));
            substring = String::new();
        }

        if c_opt == None {
            break;
        }

        if let Some(c) = c_opt {
            match c {
                'R' => instructions.push(Instruction::RotateCW),
                'L' => instructions.push(Instruction::RotateCCW),
                _ => substring.push(c),
            }
        }
    }

    instructions
}

enum Instruction {
    Straight(i32),
    RotateCW,
    RotateCCW,
}

impl Point2D {
    fn step(
        &self,
        orientation: Orientation,
        grid: &Vec<Vec<char>>,
        is_cube: bool,
    ) -> (Point2D, Orientation) {
        let cur_cube = (self.0 / 50, self.1 / 50);
        let displacement = match orientation {
            Orientation::Right => Point2D(1, 0),
            Orientation::Up => Point2D(0, -1),
            Orientation::Left => Point2D(-1, 0),
            Orientation::Down => Point2D(0, 1),
        };
        let mut pos = self.add(&displacement);
        let mut new_orientation = orientation.clone();
        let mut displaced_cube = (pos.0 / 50, pos.1 / 50);
        if pos.0 < 0 {
            displaced_cube.0 -= 1;
        }
        if pos.1 < 0 {
            displaced_cube.1 -= 1;
        }

        // wrap
        if is_cube {
            let pos_in_cube = (self.0 % 50, self.1 % 50);
            if cur_cube != displaced_cube {
                match (displaced_cube.0, displaced_cube.1, &new_orientation) {
                    (1, -1, _) => {
                        pos = Point2D(0, 150 + pos_in_cube.0);
                        new_orientation = Orientation::Right;
                    }
                    (2, -1, _) => {
                        pos = Point2D(pos_in_cube.0, 199);
                        new_orientation = Orientation::Up;
                    }
                    (3, 0, _) => {
                        pos = Point2D(99, 149 - pos_in_cube.1);
                        new_orientation = Orientation::Left;
                    }
                    (2, 1, Orientation::Down) => {
                        pos = Point2D(99, 50 + pos_in_cube.0);
                        new_orientation = Orientation::Left;
                    }
                    (2, 1, Orientation::Right) => {
                        pos = Point2D(100 + pos_in_cube.1, 49);
                        new_orientation = Orientation::Up;
                    }
                    (2, 2, _) => {
                        pos = Point2D(149, 49 - pos_in_cube.1);
                        new_orientation = Orientation::Left;
                    }
                    (1, 3, Orientation::Down) => {
                        pos = Point2D(49, 150 + pos_in_cube.0);
                        new_orientation = Orientation::Left;
                    }
                    (1, 3, Orientation::Right) => {
                        pos = Point2D(50 + pos_in_cube.1, 149);
                        new_orientation = Orientation::Up;
                    }
                    (0, 4, _) => {
                        pos = Point2D(100 + pos_in_cube.0, 0);
                        new_orientation = Orientation::Down;
                    }
                    (-1, 3, _) => {
                        pos = Point2D(50 + pos_in_cube.1, 0);
                        new_orientation = Orientation::Down;
                    }
                    (-1, 2, _) => {
                        pos = Point2D(50, 49 - pos_in_cube.1);
                        new_orientation = Orientation::Right;
                    }
                    (0, 1, Orientation::Up) => {
                        pos = Point2D(50, 50 + pos_in_cube.0);
                        new_orientation = Orientation::Right;
                    }
                    (0, 1, Orientation::Left) => {
                        pos = Point2D(pos_in_cube.1, 100);
                        new_orientation = Orientation::Down;
                    }
                    (0, 0, _) => {
                        pos = Point2D(0, 149 - pos_in_cube.1);
                        new_orientation = Orientation::Right;
                    }
                    _ => (),
                }
            }
            match grid[pos.1 as usize][pos.0 as usize] {
                '#' => {
                    pos = self.clone();
                    new_orientation = orientation.clone();
                }
                '.' => (),
                _ => panic!("cube invalid position"),
            }
        } else {
            let width = grid[0].len() as i32;
            let height = grid.len() as i32;
            pos.0 = ((pos.0 % width) + width) % width;
            pos.1 = ((pos.1 % height) + height) % height;

            while grid[pos.1 as usize][pos.0 as usize] == ' ' {
                pos = pos.add(&displacement);
                pos.0 = ((pos.0 % width) + width) % width;
                pos.1 = ((pos.1 % height) + height) % height;
            }
            if grid[pos.1 as usize][pos.0 as usize] == '#' {
                return (self.clone(), orientation.clone());
            }
        }
        (pos, new_orientation)
    }

    fn step_repeat(
        &self,
        orientation: Orientation,
        grid: &Vec<Vec<char>>,
        repeat: i32,
        is_cube: bool,
    ) -> (Point2D, Orientation) {
        let mut pos = self.clone();
        let mut orientation = orientation.clone();
        for _ in 0..repeat {
            (pos, orientation) = pos.step(orientation, grid, is_cube);
        }
        (pos, orientation)
    }
}
