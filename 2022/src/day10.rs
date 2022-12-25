use std::{collections::HashSet, fs};

pub fn run() {
    let data = fs::read_to_string("data/day10.txt").unwrap();
    let mut lines = data.lines().map(str::trim).filter(|line| !line.is_empty());

    let cycles_vec: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let cycles: HashSet<i32> = cycles_vec.into_iter().collect();

    let mut x = 1;
    let mut cpu = CpuState::Idle;
    let mut signal = 0;
    let mut crt = String::new();
    for cycle in 1..=240 {
        if cycles.contains(&cycle) {
            signal += cycle * x;
        }

        let cycle_x = (cycle - 1) % 40;
        if cycle_x == 0 && cycle > 1 {
            crt.push_str("\n");
        }
        if x >= cycle_x - 1 && x <= cycle_x + 1 {
            crt.push_str("#");
        } else {
            crt.push_str(".");
        }

        cpu = match cpu {
            CpuState::Idle => match lines.next() {
                Some(line) => match line {
                    "noop" => CpuState::Idle,
                    addx_cmd => {
                        let addx_cmd_split: Vec<&str> = addx_cmd.split_whitespace().collect();
                        CpuState::Addx(addx_cmd_split[1].parse::<i32>().unwrap())
                    }
                },
                None => CpuState::Idle,
            },
            CpuState::Addx(n) => {
                x += n;
                CpuState::Idle
            }
        };
    }
    println!("{signal}");
    println!("{crt}");
}

enum CpuState {
    Idle,
    Addx(i32),
}
