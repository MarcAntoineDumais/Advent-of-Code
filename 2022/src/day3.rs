use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() {
    let data = fs::read_to_string("data/day3.txt").unwrap();
    let lines = data.lines().map(str::trim).filter(|line| !line.is_empty());

    let mut sum = 0;
    for line in lines {
        let middle = line.len() / 2;

        let mut set: HashSet<char> = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            if i < middle {
                set.insert(c);
            } else {
                if set.contains(&c) {
                    sum += priority(c);
                    break;
                }
            }
        }
    }
    println!("{sum}");
}

pub fn part2() {
    let data = fs::read_to_string("data/day3.txt").unwrap();
    let lines = data.lines().map(str::trim).filter(|line| !line.is_empty());

    let mut sum = 0;
    let mut counts: HashMap<char, i32> = HashMap::new();
    for (i, line) in lines.enumerate() {
        if i % 3 == 0 {
            counts.clear();
        }

        for c in line.chars() {
            match i % 3 {
                0 => {
                    counts.insert(c, 1);
                }
                1 => {
                    counts.entry(c).and_modify(|count| *count = 2);
                }
                _ => {
                    if counts.contains_key(&c) && counts[&c] == 2 {
                        sum += priority(c);
                        break;
                    }
                }
            }
        }
    }
    println!("{sum}");
}

fn priority(c: char) -> i32 {
    let mut priority = c as i32;
    if c.is_ascii_lowercase() {
        priority -= 'a' as i32;
        priority += 1;
    } else {
        priority -= 'A' as i32;
        priority += 27;
    }
    priority
}
