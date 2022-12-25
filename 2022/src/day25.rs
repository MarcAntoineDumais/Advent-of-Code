use std::{collections::VecDeque, fs};

pub fn part1() {
    let data = fs::read_to_string("data/day25.txt").unwrap();
    let sum: i128 = data.lines().map(parse_snafu).sum();

    println!("{sum}");
    println!("{}", to_snafu(sum));
}

fn parse_snafu(line: &str) -> i128 {
    let mut value = 0;

    for (i, c) in line.chars().rev().enumerate() {
        value += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            x => panic!("invalid snafu {x}"),
        } * 5i128.pow(i as u32);
    }

    value
}

fn to_snafu(value: i128) -> String {
    let mut result: VecDeque<String> = VecDeque::new();
    let mut position = 0;
    let mut remaining = value;
    loop {
        if remaining == 0 {
            break;
        }

        let pow = 5i128.pow(position);
        let val = (remaining / pow) % 5;
        remaining -= val * pow;

        if val == 3 {
            remaining += pow * 5;
            result.push_front(String::from("="));
        } else if val == 4 {
            remaining += pow * 5;
            result.push_front(String::from("-"));
        } else {
            result.push_front(val.to_string());
        }
        position += 1;
    }

    let str_vec: Vec<String> = result.into();
    str_vec.join("")
}
