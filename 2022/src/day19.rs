use std::fs;

pub fn part1() {
    let data = fs::read_to_string("data/day19.txt").unwrap();
    let blueprints: Vec<_> = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let values: Vec<i32> = line
                .split(" ")
                .map(|element| element.parse::<i32>())
                .filter(|e| e.is_ok())
                .map(|v| v.unwrap())
                .collect();
            Blueprint {
                ore_cost: values[0],
                clay_cost: values[1],
                obsidien_cost_ore: values[2],
                obsidien_cost_clay: values[3],
                geode_cost_ore: values[4],
                geode_cost_obsidien: values[5],
            }
        })
        .collect();

    let sum: i32 = blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            unsafe {
                UPPER_BOUND = 0;
            }
            find_best_geode_count2(
                State {
                    minutes: 24,
                    ore: Some(0),
                    ore_gen: 1,
                    clay: Some(0),
                    clay_gen: 0,
                    obsidien: Some(0),
                    obsidien_gen: 0,
                    geode: 0,
                    geode_gen: 0,
                },
                None,
                *blueprint,
                Vec::new(),
            )
            .0 * (i as i32 + 1)
        })
        .sum();
    println!("{sum}");
}

pub fn part2() {
    let data = fs::read_to_string("data/day19.txt").unwrap();
    let blueprints: Vec<_> = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let values: Vec<i32> = line
                .split(" ")
                .map(|element| element.parse::<i32>())
                .filter(|e| e.is_ok())
                .map(|v| v.unwrap())
                .collect();
            Blueprint {
                ore_cost: values[0],
                clay_cost: values[1],
                obsidien_cost_ore: values[2],
                obsidien_cost_clay: values[3],
                geode_cost_ore: values[4],
                geode_cost_obsidien: values[5],
            }
        })
        .collect();
    let blueprints = blueprints.iter().take(3);
    let counts: Vec<(i32, Vec<State>)> = blueprints
        .map(|blueprint| {
            unsafe {
                UPPER_BOUND = 0;
            }
            find_best_geode_count2(
                State {
                    minutes: 32,
                    ore: Some(0),
                    ore_gen: 1,
                    clay: Some(0),
                    clay_gen: 0,
                    obsidien: Some(0),
                    obsidien_gen: 0,
                    geode: 0,
                    geode_gen: 0,
                },
                None,
                *blueprint,
                Vec::new(),
            )
        })
        .collect();
    println!("{}", counts[0].0 * counts[1].0 * counts[2].0);
}

static mut UPPER_BOUND: i32 = 0;

fn find_best_geode_count2(
    state: State,
    next_purchase: Option<i32>,
    blueprint: Blueprint,
    mut stack: Vec<State>,
) -> (i32, Vec<State>) {
    if state.minutes <= 0 {
        return (state.geode, stack);
    }
    if state.minutes == 1 {
        unsafe {
            UPPER_BOUND = UPPER_BOUND.max(state.geode + state.geode_gen);
        }
        stack.push(state.clone());
        return (state.geode + state.geode_gen, stack);
    }

    if next_purchase == None {
        return state
            .get_finite_resources()
            .iter()
            .map(|next| find_best_geode_count2(state, Some(*next), blueprint, stack.clone()))
            .max_by_key(|(v, _)| *v)
            .unwrap();
    }

    let cur_heuristic = heuristic(state);
    unsafe {
        if cur_heuristic < UPPER_BOUND {
            return (0, stack);
        }
    }

    let mut state = state.clone();
    stack.push(state);

    let purchaseable = state.can_purchase(&blueprint);
    let mut purchased = next_purchase;
    if next_purchase == Some(0) && purchaseable.0 {
        state.ore = state.ore.and_then(|x| Some(x - blueprint.ore_cost));
    } else if next_purchase == Some(1) && purchaseable.1 {
        state.ore = state.ore.and_then(|x| Some(x - blueprint.clay_cost));
    } else if next_purchase == Some(2) && purchaseable.2 {
        state.ore = state
            .ore
            .and_then(|x| Some(x - blueprint.obsidien_cost_ore));
        state.clay = state
            .clay
            .and_then(|x| Some(x - blueprint.obsidien_cost_clay));
    } else if next_purchase == Some(3) && purchaseable.3 {
        state.ore = state.ore.and_then(|x| Some(x - blueprint.geode_cost_ore));
        state.obsidien = state
            .obsidien
            .and_then(|x| Some(x - blueprint.geode_cost_obsidien));
    } else {
        purchased = None;
    }

    state.gen_resources();

    if let Some(i) = purchased {
        match i {
            0 => {
                if state.ore != None {
                    state.ore_gen += 1;
                }
            }
            1 => {
                if state.clay != None {
                    state.clay_gen += 1;
                }
            }
            2 => {
                if state.obsidien != None {
                    state.obsidien_gen += 1;
                }
            }
            _ => state.geode_gen += 1,
        }
    } else {
        state.update_infinite_resources(&blueprint);
        return find_best_geode_count2(state, next_purchase, blueprint, stack.clone());
    }

    state.update_infinite_resources(&blueprint);
    state
        .get_finite_resources()
        .iter()
        .map(|next| find_best_geode_count2(state, Some(*next), blueprint, stack.clone()))
        .max_by_key(|(v, _)| *v)
        .unwrap()
}

fn heuristic(state: State) -> i32 {
    let mut state = state.clone();
    while state.minutes > 0 {
        state.geode += state.geode_gen;
        state.minutes -= 1;
        state.geode_gen += 1;
    }
    state.geode
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Blueprint {
    ore_cost: i32,
    clay_cost: i32,
    obsidien_cost_ore: i32,
    obsidien_cost_clay: i32,
    geode_cost_ore: i32,
    geode_cost_obsidien: i32,
}

impl Blueprint {
    fn max_ore_cost(&self) -> i32 {
        self.ore_cost
            .max(self.clay_cost)
            .max(self.obsidien_cost_ore)
            .max(self.geode_cost_ore)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    minutes: i32,
    ore: Option<i32>,
    ore_gen: i32,
    clay: Option<i32>,
    clay_gen: i32,
    obsidien: Option<i32>,
    obsidien_gen: i32,
    geode: i32,
    geode_gen: i32,
}

impl State {
    fn gen_resources(&mut self) {
        if let Some(x) = self.ore {
            self.ore = Some(x + self.ore_gen);
        }
        if let Some(x) = self.clay {
            self.clay = Some(x + self.clay_gen);
        }
        if let Some(x) = self.obsidien {
            self.obsidien = Some(x + self.obsidien_gen);
        }
        self.geode += self.geode_gen;
        self.minutes -= 1;
    }

    fn update_infinite_resources(&mut self, blueprint: &Blueprint) {
        if self.ore != None
            && self.ore_gen >= blueprint.max_ore_cost()
            && self.ore.unwrap() >= blueprint.max_ore_cost()
        {
            self.ore = None;
            self.ore_gen = 0;
        }
        if self.clay != None
            && self.clay_gen >= blueprint.obsidien_cost_clay
            && self.clay.unwrap() >= blueprint.obsidien_cost_clay
        {
            self.clay = None;
            self.clay_gen = 0;
        }
        if self.obsidien != None
            && self.obsidien_gen >= blueprint.geode_cost_obsidien
            && self.obsidien.unwrap() >= blueprint.geode_cost_obsidien
        {
            self.obsidien = None;
            self.obsidien_gen = 0;
        }
    }

    fn get_finite_resources(&self) -> Vec<i32> {
        let mut finite = vec![3];
        if self.ore != None {
            finite.push(0);
        }
        if self.clay != None {
            finite.push(1);
        }
        if self.obsidien != None {
            finite.push(2);
        }
        finite
    }

    fn can_purchase(&self, blueprint: &Blueprint) -> (bool, bool, bool, bool) {
        (
            self.ore != None && self.ore.unwrap() >= blueprint.ore_cost,
            self.clay != None && (self.ore == None || self.ore.unwrap() >= blueprint.clay_cost),
            self.obsidien != None
                && (self.ore == None || self.ore.unwrap() >= blueprint.obsidien_cost_ore)
                && (self.clay == None || self.clay.unwrap() >= blueprint.obsidien_cost_clay),
            (self.ore == None || self.ore.unwrap() >= blueprint.geode_cost_ore)
                && (self.obsidien == None
                    || self.obsidien.unwrap() >= blueprint.geode_cost_obsidien),
        )
    }
}
