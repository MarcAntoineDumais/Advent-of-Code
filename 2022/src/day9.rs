use std::{collections::HashSet, fs};

use crate::utils::Point2D;

pub fn run() {
    let data = fs::read_to_string("data/day9.txt").unwrap();
    let lines = data.lines().map(str::trim).filter(|line| !line.is_empty());

    let mut tail_positions: HashSet<Point2D> = HashSet::new();
    //let mut head = Point2D(0, 0);
    //let mut tail = Point2D(0, 0);
    let mut rope = vec![Point2D(0, 0); 10];
    tail_positions.insert(rope[rope.len() - 1].clone());
    for line in lines {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        for _ in 0..cmd[1].parse::<u32>().unwrap() {
            match cmd[0] {
                "U" => {
                    rope[0].1 += 1;
                }
                "D" => {
                    rope[0].1 -= 1;
                }
                "L" => {
                    rope[0].0 -= 1;
                }
                "R" => {
                    rope[0].0 += 1;
                }
                _ => {}
            }

            for i in 1..rope.len() {
                let diff = rope[i - 1].sub(&rope[i]);
                if diff.magnitude_squared() > 2 {
                    rope[i] = rope[i].add(&diff.clamp(-1, 1));
                    if i == (rope.len() - 1) {
                        tail_positions.insert(rope[i].clone());
                    }
                }
            }
        }
    }

    let count = tail_positions.len();
    println!("{count}");
}
