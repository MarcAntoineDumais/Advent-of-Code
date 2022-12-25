use crate::utils::{Orientation, Point2D};
use pathfinding::prelude::astar;
use std::{collections::HashMap, fs};

pub fn part1() {
    run(false);
}

pub fn part2() {
    run(true);
}

fn run(part_2: bool) {
    let data = fs::read_to_string("data/day24.txt").unwrap();
    let height = data.lines().count();
    let width = data.lines().next().unwrap().chars().count();

    let mut blizzards = Vec::new();
    data.lines().enumerate().for_each(|(row, line)| {
        for (col, c) in line.chars().enumerate() {
            if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
                continue;
            }

            let orientation = match c {
                '^' => Some(Orientation::Up),
                'v' => Some(Orientation::Down),
                '<' => Some(Orientation::Left),
                '>' => Some(Orientation::Right),
                _ => None,
            };
            if let Some(orientation) = orientation {
                blizzards.push(Blizzard {
                    pos: Point2D(col as i32, row as i32),
                    orientation,
                })
            }
        }
    });

    let start = Node {
        pos: Point2D(1, 0),
        blizzards,
        step: 0,
    };
    let goal = Point2D(width as i32 - 2, height as i32 - 1);
    let path = astar(
        &start,
        |node| {
            let mut occupied = HashMap::new();
            let moved_blizzards: Vec<Blizzard> = node
                .blizzards
                .iter()
                .map(|bliz| {
                    let moved_bliz = bliz.step(width as i32, height as i32);
                    occupied.insert(moved_bliz.pos.clone(), true);
                    moved_bliz
                })
                .collect();

            let mut next_step = node.step;
            if part_2 {
                if node.step == 0 && node.pos == goal {
                    next_step = 1;
                } else if node.step == 1 && node.pos == start.pos {
                    next_step = 2;
                }
            }
            let mut next_nodes = Vec::new();
            if !occupied.contains_key(&node.pos) {
                next_nodes.push((
                    Node {
                        pos: node.pos.clone(),
                        blizzards: moved_blizzards.clone(),
                        step: next_step,
                    },
                    1,
                ));
            }
            let move_up = node.pos.add(&Point2D(0, -1));
            if (move_up.1 > 0 || (move_up.0 == 1 && move_up.1 == 0))
                && !occupied.contains_key(&move_up)
            {
                next_nodes.push((
                    Node {
                        pos: move_up,
                        blizzards: moved_blizzards.clone(),
                        step: next_step,
                    },
                    1,
                ));
            }
            let move_down = node.pos.add(&Point2D(0, 1));
            if (move_down.1 < height as i32 - 1
                || (move_down.0 == (width as i32 - 2) && move_down.1 == (height as i32 - 1)))
                && !occupied.contains_key(&move_down)
            {
                next_nodes.push((
                    Node {
                        pos: move_down,
                        blizzards: moved_blizzards.clone(),
                        step: next_step,
                    },
                    1,
                ));
            }
            let move_right = node.pos.add(&Point2D(1, 0));
            if move_right.0 < width as i32 - 1
                && move_right.1 > 0
                && move_right.1 < height as i32 - 1
                && !occupied.contains_key(&move_right)
            {
                next_nodes.push((
                    Node {
                        pos: move_right,
                        blizzards: moved_blizzards.clone(),
                        step: next_step,
                    },
                    1,
                ));
            }
            let move_left = node.pos.add(&Point2D(-1, 0));
            if move_left.0 > 0
                && move_left.1 > 0
                && move_left.1 < height as i32 - 1
                && !occupied.contains_key(&move_left)
            {
                next_nodes.push((
                    Node {
                        pos: move_left,
                        blizzards: moved_blizzards.clone(),
                        step: next_step,
                    },
                    1,
                ));
            }
            next_nodes
        },
        |node| {
            if part_2 {
                let mut distance = 0;
                if node.step == 0 {
                    distance += (goal.0 - node.pos.0).abs() + (goal.1 - node.pos.1).abs();
                    distance += 2 * ((goal.0 - start.pos.0).abs() + (goal.1 - start.pos.1).abs());
                } else if node.step == 1 {
                    distance += (start.pos.0 - node.pos.0).abs() + (start.pos.1 - node.pos.1).abs();
                    distance += (goal.0 - start.pos.0).abs() + (goal.1 - start.pos.1).abs();
                } else {
                    distance += (goal.0 - node.pos.0).abs() + (goal.1 - node.pos.1).abs();
                }
                return distance;
            } else {
                return (goal.0 - node.pos.0).abs() + (goal.1 - node.pos.1).abs();
            }
        },
        |node| node.pos == goal && (!part_2 || node.step == 2),
    )
    .unwrap();

    // for (i, node) in path.0.iter().enumerate() {
    //     println!("Minute {i}");
    //     node.draw(width as i32, height as i32);
    //     println!();
    // }
    println!("{}", path.1);
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Point2D,
    blizzards: Vec<Blizzard>,
    step: i32,
}

impl Node {
    fn draw(&self, width: i32, height: i32) {
        let bliz_map: HashMap<Point2D, Orientation> = self
            .blizzards
            .iter()
            .map(|bliz| (bliz.pos, bliz.orientation.clone()))
            .collect();
        for row in 0..height {
            let mut row_str = String::new();
            for col in 0..width {
                if self.pos == Point2D(col, row) {
                    row_str.push_str("E");
                } else if let Some(orientation) = bliz_map.get(&Point2D(col, row)) {
                    row_str.push_str(match orientation {
                        Orientation::Right => ">",
                        Orientation::Up => "^",
                        Orientation::Left => "<",
                        Orientation::Down => "v",
                    });
                } else if col == 0
                    || col == width - 1
                    || (row == 0 && col != 1)
                    || (row == height - 1 && col != width - 2)
                {
                    row_str.push_str("#");
                } else {
                    row_str.push_str(".");
                }
            }
            println!("{row_str}");
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    pos: Point2D,
    orientation: Orientation,
}

impl Blizzard {
    fn step(&self, width: i32, height: i32) -> Blizzard {
        let mut pos = self.pos.clone();
        match self.orientation {
            Orientation::Right => {
                if pos.0 == width - 2 {
                    pos.0 = 1;
                } else {
                    pos.0 += 1;
                }
            }
            Orientation::Up => {
                if pos.1 == 1 {
                    pos.1 = height - 2;
                } else {
                    pos.1 -= 1;
                }
            }
            Orientation::Left => {
                if pos.0 == 1 {
                    pos.0 = width - 2;
                } else {
                    pos.0 -= 1;
                }
            }
            Orientation::Down => {
                if pos.1 == height - 2 {
                    pos.1 = 1;
                } else {
                    pos.1 += 1;
                }
            }
        }
        Blizzard {
            pos,
            orientation: self.orientation.clone(),
        }
    }
}
