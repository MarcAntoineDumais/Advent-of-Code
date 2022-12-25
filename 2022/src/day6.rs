use std::{collections::HashMap, fs};

pub fn run() {
    let criteria = 14;

    let data = fs::read_to_string("data/day6.txt").unwrap();
    let chars: Vec<char> = data.chars().collect();

    let mut counts: HashMap<char, u32> = HashMap::new();
    for (i, c) in chars.iter().enumerate() {
        counts.entry(*c).and_modify(|x| *x += 1).or_insert(1);
        if counts.len() == criteria {
            println!("{}", i + 1);
            return;
        }
        if i >= criteria - 1 {
            let oldest = chars[i - (criteria - 1)];
            counts.entry(oldest).and_modify(|v| *v -= 1);
            counts.retain(|_, v| *v > 0);
        }
    }
}
