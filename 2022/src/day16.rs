use std::{collections::HashMap, fs};

use regex::Regex;

pub fn part1() {
    let (valves, valve_indices, next, non_zero_valves) = process_input();

    let minutes = 30;
    let start = valve_indices["AA"];

    let (max_flow, max_plan, _) =
        search_solution(start, minutes, false, &valves, next, non_zero_valves);
    println!(
        "Max {max_flow}  Plan {:?}",
        max_plan
            .iter()
            .map(|i| valves[*i].0.as_str())
            .collect::<Vec<&str>>()
    );
}

pub fn part2() {
    let (valves, valve_indices, next, non_zero_valves) = process_input();

    let minutes = 26;
    let start = valve_indices["AA"];

    let (max_flow, max_plan, max_elephant) =
        search_solution(start, minutes, true, &valves, next, non_zero_valves);
    println!(
        "Max {max_flow}  Plan {:?}  Elephant {max_elephant}",
        max_plan
            .iter()
            .map(|i| valves[*i].0.as_str())
            .collect::<Vec<&str>>()
    );
}

fn process_input() -> (
    Vec<(String, i32, Vec<String>)>,
    HashMap<String, usize>,
    Vec<Vec<Option<usize>>>,
    Vec<usize>,
) {
    let reg =
        Regex::new(r"Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)").unwrap();
    let data = fs::read_to_string("data/day16.txt").unwrap();

    let mut valve_indices = HashMap::new();
    let mut edges = Vec::new();
    let valves: Vec<_> = data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(i, line)| {
            let cap = reg.captures(line).unwrap();
            let origin = cap.get(1).unwrap().as_str();
            let flow = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let dests: Vec<_> = cap
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(str::to_string)
                .collect();
            for dest in dests.iter() {
                edges.push((origin.to_string(), dest.to_string()));
            }
            valve_indices.insert(origin.to_string(), i);
            (origin.to_string(), flow, dests)
        })
        .collect();
    let edges: Vec<(usize, usize)> = edges
        .iter()
        .map(|(orig, dest)| {
            (
                *valve_indices.get(orig).unwrap(),
                *valve_indices.get(dest).unwrap(),
            )
        })
        .collect();

    let mut distances = vec![vec![i32::MAX / 2 - 1000; valves.len()]; valves.len()];
    let mut next = vec![vec![None; valves.len()]; valves.len()];
    for (orig, dest) in edges {
        distances[orig][dest] = 1;
        next[orig][dest] = Some(dest);
    }
    for i in 0..valves.len() {
        distances[i][i] = 0;
        next[i][i] = Some(i);
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if distances[i][j] > distances[i][k] + distances[k][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                    next[i][j] = next[i][k];
                }
            }
        }
    }

    let non_zero_valves: Vec<_> = valves
        .iter()
        .filter(|(_, n, _)| *n > 0)
        .map(|(s, _, _)| valve_indices[s])
        .collect();

    (valves, valve_indices, next, non_zero_valves)
}

fn search_solution(
    start: usize,
    minutes: i32,
    with_elephant: bool,
    valves: &Vec<(String, i32, Vec<String>)>,
    next: Vec<Vec<Option<usize>>>,
    non_zero_valves: Vec<usize>,
) -> (i32, Vec<usize>, usize) {
    let mut max_plan: Vec<usize> = non_zero_valves.clone();
    let mut max_elephant = 0;
    let mut max_flow = calc_flow(&max_plan, max_elephant, start, &valves, &next, minutes);
    for _ in 0..1000 {
        let mut cur_elephant = match with_elephant {
            true => fastrand::usize(..) % non_zero_valves.len(),
            false => 0,
        };
        let mut cur_plan: Vec<usize> = non_zero_valves.clone();
        fastrand::shuffle(&mut cur_plan);
        let mut cur_flow = calc_flow(&cur_plan, cur_elephant, start, &valves, &next, minutes);
        if cur_flow > max_flow {
            max_flow = cur_flow;
            max_plan = cur_plan.clone();
            max_elephant = cur_elephant;
        }

        loop {
            let mut swaps: Vec<_> = (0..non_zero_valves.len())
                .collect::<Vec<_>>()
                .iter()
                .map(|i| ((i + 1)..non_zero_valves.len()).map(|j| (i.clone(), j, None)))
                .flatten()
                .collect();
            if with_elephant {
                (0..non_zero_valves.len())
                    .filter(|e| *e != cur_elephant)
                    .for_each(|e| {
                        swaps.push((0, 0, Some(e)));
                    });
            }
            let best = swaps
                .iter()
                .map(|(i, j, e)| match e {
                    Some(e) => {
                        let val = calc_flow(&cur_plan, *e, start, &valves, &next, minutes);
                        (i, j, Some(*e), val)
                    }
                    None => {
                        cur_plan.swap(*i, *j);
                        let val =
                            calc_flow(&cur_plan, cur_elephant, start, &valves, &next, minutes);
                        cur_plan.swap(*i, *j);
                        (i, j, *e, val)
                    }
                })
                .filter(|(_, _, _, val)| *val > cur_flow)
                .max_by_key(|(_, _, _, val)| *val);
            match best {
                None => {
                    break;
                }
                Some((i, j, e, flow)) => {
                    match e {
                        Some(e) => {
                            cur_elephant = e;
                        }
                        None => {
                            cur_plan.swap(*i, *j);
                        }
                    }
                    cur_flow = flow;
                    if cur_flow > max_flow {
                        max_flow = cur_flow;
                        max_plan = cur_plan.clone();
                        max_elephant = cur_elephant;
                    }
                }
            }
        }
    }
    (max_flow, max_plan, max_elephant)
}

fn calc_flow(
    plan: &Vec<usize>,
    elephant: usize,
    start: usize,
    valves: &Vec<(String, i32, Vec<String>)>,
    next: &Vec<Vec<Option<usize>>>,
    minutes: i32,
) -> i32 {
    let mut flow = 0;
    let mut plan_it = plan[..elephant].iter();
    let mut plan_elephant_it = plan[elephant..].iter();
    let mut cur = start;
    let mut cur_elephant = start;
    let mut dest = plan_it.next();
    let mut dest_elephant = plan_elephant_it.next();
    for time_left in (1..=minutes).rev() {
        match dest {
            Some(d) => {
                if cur == *d {
                    flow += valves[*d].1 * (time_left - 1);
                    dest = plan_it.next();
                } else {
                    cur = next[cur][*d].unwrap();
                }
            }
            None => {
                if dest_elephant == None {
                    break;
                }
            }
        }
        match dest_elephant {
            Some(d) => {
                if cur_elephant == *d {
                    flow += valves[*d].1 * (time_left - 1);
                    dest_elephant = plan_elephant_it.next();
                } else {
                    cur_elephant = next[cur_elephant][*d].unwrap();
                }
            }
            None => {
                if dest == None {
                    break;
                }
            }
        }
    }
    flow
}
