use std::{collections::HashMap, fs};

pub fn part1() {
    run(true);
}

pub fn part2() {
    run(false);
}

fn run(is_part1: bool) {
    let data = fs::read_to_string("data/day21.txt").unwrap();
    let mut known_monkeys = HashMap::new();
    let mut to_calculate = HashMap::new();
    data.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let split: Vec<&str> = line.split(": ").collect();
            if !is_part1 && split[0] == "humn" {
                return;
            }
            if let Ok(x) = split[1].parse::<i64>() {
                known_monkeys.insert(split[0].to_string(), x);
            } else {
                let name = match is_part1 {
                    true => "",
                    false => split[0],
                };
                to_calculate.insert(split[0].to_string(), Op::new(split[1], name));
            }
        });

    let mut added_node_count = 0;
    loop {
        if is_part1 {
            if let Some(x) = known_monkeys.get("root") {
                println!("{}", *x);
                return;
            }
        } else {
            if let Some(x) = known_monkeys.get("humn") {
                println!("{}", *x);
                return;
            }
        }

        to_calculate.clone().iter().for_each(|(monkey, op)| {
            let mut updated_op = op.clone();
            if !is_part1 {
                match (op.op_left.clone(), op.op_right.clone()) {
                    (Operand::Number(x), Operand::Unknown(s)) => {
                        if s == "humn" {
                            if let OpType::Equality = op.op_type {
                                known_monkeys.insert(s, x);
                                return;
                            }
                            to_calculate.insert(s, op.invert(Operand::Unknown(monkey.clone())).0);
                        }
                    }
                    (Operand::Unknown(s), Operand::Number(y)) => {
                        if s == "humn" {
                            if let OpType::Equality = op.op_type {
                                known_monkeys.insert(s, y);
                                return;
                            }
                            to_calculate.insert(s, op.invert(Operand::Unknown(monkey.clone())).0);
                        }
                    }
                    (Operand::Number(_), Operand::Number(_)) => (),
                    (Operand::Unknown(_), Operand::Unknown(_)) => (),
                }
            }

            match (
                op.op_left.clone(),
                op.op_right.clone(),
                known_monkeys.get(monkey),
            ) {
                (_, _, None) => (),
                (Operand::Number(_), Operand::Number(_), Some(_)) => {
                    to_calculate.remove(&monkey.clone());
                }
                (_, _, Some(z)) => {
                    to_calculate.remove(&monkey.clone());
                    let inverted = op.invert(Operand::Number(*z));
                    if to_calculate.contains_key(&inverted.1) {
                        to_calculate.insert(added_node_count.to_string(), inverted.0);
                        added_node_count += 1;
                        to_calculate.insert(
                            added_node_count.to_string(),
                            Op {
                                op_left: Operand::Unknown(inverted.1),
                                op_right: Operand::Unknown((added_node_count - 1).to_string()),
                                op_type: OpType::Equality,
                            },
                        );
                        added_node_count += 1;
                    } else {
                        to_calculate.insert(inverted.1, inverted.0);
                    }
                }
            }

            if let Operand::Unknown(left) = &op.op_left {
                if let Some(x) = known_monkeys.get(left) {
                    updated_op.op_left = Operand::Number(*x);
                }
            }
            if let Operand::Unknown(right) = &op.op_right {
                if let Some(x) = known_monkeys.get(right) {
                    updated_op.op_right = Operand::Number(*x);
                }
            }

            if let Some(x) = op.calculate() {
                known_monkeys.insert(monkey.clone(), x);
                to_calculate.remove(monkey);
                return;
            }
            to_calculate
                .entry(monkey.clone())
                .and_modify(|x| *x = updated_op.clone());

            match (updated_op.op_left, updated_op.op_right, updated_op.op_type) {
                (Operand::Unknown(left), Operand::Number(x), OpType::Equality) => {
                    known_monkeys.insert(left.clone(), x);
                    to_calculate.remove(&monkey.clone());
                }
                (Operand::Number(x), Operand::Unknown(right), OpType::Equality) => {
                    known_monkeys.insert(right.clone(), x);
                    to_calculate.remove(&monkey.clone());
                }
                _ => (),
            }
        });
    }
}

#[derive(Clone)]
struct Op {
    op_left: Operand,
    op_right: Operand,
    op_type: OpType,
}

impl Op {
    fn new(s: &str, name: &str) -> Op {
        Op {
            op_left: Operand::Unknown(s[0..4].to_string().clone()),
            op_right: Operand::Unknown(s[7..].to_string().clone()),
            op_type: OpType::new(&s[5..=5], name),
        }
    }

    fn calculate(&self) -> Option<i64> {
        match (
            self.op_left.clone(),
            self.op_right.clone(),
            self.op_type.clone(),
        ) {
            (Operand::Number(x), Operand::Number(y), OpType::Addition) => Some(x + y),
            (Operand::Number(x), Operand::Number(y), OpType::Subtraction) => Some(x - y),
            (Operand::Number(x), Operand::Number(y), OpType::Multiplication) => Some(x * y),
            (Operand::Number(x), Operand::Number(y), OpType::Division) => Some(x / y),
            _ => None,
        }
    }

    fn invert(&self, unknown: Operand) -> (Op, String) {
        match (
            self.op_left.clone(),
            self.op_right.clone(),
            self.op_type.clone(),
        ) {
            (Operand::Number(_), Operand::Number(_), _) => panic!("wrong invert"),
            (_, _, OpType::Equality) => {
                panic!("wrong type for invert")
            }
            (Operand::Unknown(s), _, OpType::Addition) => (
                Op {
                    op_left: unknown.clone(),
                    op_right: self.op_right.clone(),
                    op_type: OpType::Subtraction,
                },
                s.clone(),
            ),
            (_, Operand::Unknown(s), OpType::Addition) => (
                Op {
                    op_left: unknown.clone(),
                    op_right: self.op_left.clone(),
                    op_type: OpType::Subtraction,
                },
                s.clone(),
            ),
            (Operand::Unknown(s), _, OpType::Subtraction) => (
                Op {
                    op_left: unknown.clone(),
                    op_right: self.op_right.clone(),
                    op_type: OpType::Addition,
                },
                s.clone(),
            ),
            (_, Operand::Unknown(s), OpType::Subtraction) => (
                Op {
                    op_left: self.op_left.clone(),
                    op_right: unknown.clone(),
                    op_type: OpType::Subtraction,
                },
                s.clone(),
            ),
            (Operand::Unknown(s), _, OpType::Multiplication) => (
                Op {
                    op_left: unknown.clone(),
                    op_right: self.op_right.clone(),
                    op_type: OpType::Division,
                },
                s.clone(),
            ),
            (_, Operand::Unknown(s), OpType::Multiplication) => (
                Op {
                    op_left: unknown.clone(),
                    op_right: self.op_left.clone(),
                    op_type: OpType::Division,
                },
                s.clone(),
            ),
            (Operand::Unknown(s), _, OpType::Division) => (
                Op {
                    op_left: unknown.clone(),
                    op_right: self.op_right.clone(),
                    op_type: OpType::Multiplication,
                },
                s.clone(),
            ),
            (_, Operand::Unknown(s), OpType::Division) => (
                Op {
                    op_left: self.op_left.clone(),
                    op_right: unknown.clone(),
                    op_type: OpType::Division,
                },
                s.clone(),
            ),
        }
    }
}

#[derive(Clone)]
enum Operand {
    Number(i64),
    Unknown(String),
}

#[derive(Clone)]
enum OpType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equality,
}

impl OpType {
    fn new(s: &str, name: &str) -> OpType {
        match (s, name) {
            (_, "root") => OpType::Equality,
            ("+", _) => OpType::Addition,
            ("-", _) => OpType::Subtraction,
            ("*", _) => OpType::Multiplication,
            ("/", _) => OpType::Division,
            _ => panic!("wrong op"),
        }
    }
}
