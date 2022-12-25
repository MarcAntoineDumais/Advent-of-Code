use std::{fmt::Display, fs};

pub fn run() {
    let data = fs::read_to_string("data/day13.txt").unwrap();
    let mut elements: Vec<Element> = data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(Element::new)
        .collect();

    let mut sum = 0;
    for i in 1..=(elements.len() / 2) {
        let index = (i - 1) * 2;
        if elements[index] <= elements[index + 1] {
            sum += i;
        }
    }
    println!("{sum}");

    // Part 2
    let divider_1 = Element::List(vec![Element::Int(2)]);
    let divider_2 = Element::List(vec![Element::Int(6)]);
    elements.push(divider_1.clone());
    elements.push(divider_2.clone());
    elements.sort();
    println!(
        "{}",
        (elements.binary_search(&divider_1).unwrap() + 1)
            * (elements.binary_search(&divider_2).unwrap() + 1)
    );
}

#[derive(Clone, Debug)]
enum Element {
    Int(i32),
    List(Vec<Element>),
}

impl Element {
    fn new(s: &str) -> Element {
        if s.len() == 2 && &s[0..2] == "[]" {
            return Element::List(Vec::new());
        }
        if &s[0..1] != "[" {
            return Element::Int(s.parse().unwrap());
        }
        let elements = split_shallow(&s[1..(s.len() - 1)])
            .iter()
            .map(|s| Element::new(*s))
            .collect();
        Element::List(elements)
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl Eq for Element {}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let (Self::Int(l0), Self::Int(r0)) = (self, other) {
            return l0.cmp(r0);
        }

        let left = match self {
            Element::Int(_) => vec![self.clone()],
            Element::List(v) => v.clone(),
        };
        let right = match other {
            Element::Int(_) => vec![other.clone()],
            Element::List(v) => v.clone(),
        };

        let mut left_iter = left.iter();
        let mut right_iter = right.iter();

        loop {
            let left_opt = left_iter.next();
            let right_opt = right_iter.next();
            match left_opt {
                Some(left_val) => match right_opt {
                    Some(right_val) => {
                        return match left_val.partial_cmp(right_val).unwrap() {
                            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                            std::cmp::Ordering::Equal => continue,
                            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                        }
                    }
                    None => return std::cmp::Ordering::Greater,
                },
                None => match right_opt {
                    Some(_) => return std::cmp::Ordering::Less,
                    None => return std::cmp::Ordering::Equal,
                },
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Int(n) => f.write_str(n.to_string().as_str()),
            Element::List(elements) => {
                f.write_str("[")?;
                let elements_str: Vec<String> =
                    elements.iter().map(|elem| format!("{elem}")).collect();
                f.write_str(elements_str.join(",").as_str())?;
                f.write_str("]")
            }
        }
    }
}

fn split_shallow(s: &str) -> Vec<&str> {
    let mut depth = 0;
    let mut start_index = 0;
    let mut elements = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if depth > 0 {
            match c {
                ']' => depth -= 1,
                '[' => depth += 1,
                _ => (),
            }
            continue;
        }

        match c {
            '[' => depth += 1,
            ',' => {
                elements.push(&s[start_index..i]);
                start_index = i + 1
            }
            _ => (),
        }
    }
    elements.push(&s[start_index..s.chars().count()]);
    elements
}
