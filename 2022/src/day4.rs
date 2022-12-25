use std::fs;

pub fn run() {
    let data = fs::read_to_string("data/day4.txt").unwrap();
    let lines = data.lines().map(str::trim).filter(|line| !line.is_empty());

    let mut total = 0;
    for line in lines {
        let mut assignments = line.split(",");
        let assignment1 = Assignment::new(assignments.next().unwrap());
        let assignment2 = Assignment::new(assignments.next().unwrap());
        //if assignment1.contains(&assignment2) || assignment2.contains(&assignment1) {
        if assignment1.overlaps(&assignment2) {
            total += 1;
        }
    }
    println!("{total}");
}

struct Assignment {
    lower: i32, // included
    upper: i32, // included
}

impl Assignment {
    fn new(assignment: &str) -> Assignment {
        let mut bounds = assignment.split("-");
        Assignment {
            lower: bounds.next().unwrap().parse::<i32>().unwrap(),
            upper: bounds.next().unwrap().parse::<i32>().unwrap(),
        }
    }

    fn contains(&self, other: &Assignment) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (self.upper >= other.lower && self.upper <= other.upper)
            || (other.upper >= self.lower && other.upper <= self.upper)
    }
}
