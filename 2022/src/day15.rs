use std::fs;

use regex::Regex;

use crate::utils::{Intervals, Point2D};

pub fn run() {
    let reg = Regex::new(r"[xy]=(-*\d*)").unwrap();
    let data = fs::read_to_string("data/day15.txt").unwrap();
    let sensors: Vec<_> = data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut matches = reg
                .captures_iter(line)
                .map(|cap| cap.get(1).unwrap().as_str().parse::<i32>().unwrap());
            (
                Point2D(matches.next().unwrap(), matches.next().unwrap()),
                Point2D(matches.next().unwrap(), matches.next().unwrap()),
            )
        })
        .collect();

    let row = 10;
    let mut intervals = Intervals::new();
    for (sensor, beacon) in sensors.iter() {
        let manhattan_dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let row_dist = (row - sensor.1).abs();
        intervals.add(Point2D(
            sensor.0 - manhattan_dist + row_dist,
            sensor.0 + manhattan_dist - row_dist,
        ))
    }
    let invalid_positions = intervals
        .ranges
        .iter()
        .fold(0, |n, elem| n + (elem.1 - elem.0));
    println!("{invalid_positions}");

    // Part 2
    let max_coord = 4000000;
    for row in 0..=max_coord {
        let mut range = Intervals::new();
        range.add(Point2D(0, max_coord + 1));

        for (sensor, beacon) in sensors.iter() {
            let manhattan_dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            let row_dist = (row - sensor.1).abs();
            range.sub(Point2D(
                sensor.0 - manhattan_dist + row_dist,
                sensor.0 + manhattan_dist - row_dist + 1,
            ))
        }
        if !range.ranges.is_empty() {
            println!("{}", (range.ranges[0].0 as i64) * 4000000 + row as i64);
            return;
        }
    }
}
