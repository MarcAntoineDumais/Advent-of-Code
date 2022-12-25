use std::{collections::HashMap, fs};

pub fn run() {
    let data = fs::read_to_string("data/day7.txt").unwrap();
    let lines = data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().collect::<Vec<&str>>());

    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut dir: Vec<String> = Vec::new();
    for cmd in lines {
        if cmd[0] == "$" {
            if cmd[1] == "cd" {
                match cmd[2] {
                    ".." => {
                        dir.pop();
                        ()
                    }
                    "/" => {
                        dir = vec![];
                        ()
                    }
                    _ => dir.push(cmd[2].to_string()),
                }
            }
        } else if cmd[0] != "dir" {
            let size: u64 = cmd[0].parse().unwrap();
            let mut tmp_dir = dir.clone();
            loop {
                sizes
                    .entry(tmp_dir.join("/"))
                    .and_modify(|x| *x += size)
                    .or_insert(size);

                if tmp_dir.is_empty() {
                    break;
                }

                tmp_dir.pop();
            }
        }
    }

    //println!("{:?}", sizes);

    let sum = sizes
        .iter()
        .filter(|(_, v)| **v <= 100000)
        .map(|(_, v)| *v)
        .sum::<u64>();

    println!("{sum}");

    // Part 2
    let total_size: u64 = 70000000;
    let update_size: u64 = 30000000;
    let free_size = total_size - sizes[""];
    let needed_size = update_size - free_size;

    let min = sizes
        .iter()
        .filter(|(_, v)| **v >= needed_size)
        .map(|(_, v)| *v)
        .min()
        .unwrap();
    println!("{min}")
}
