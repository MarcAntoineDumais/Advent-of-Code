use std::fs;

pub fn run() {
    let data = fs::read_to_string("data/day1.txt").unwrap();
    let lines = data.lines();

    let mut top_elves = TopN::new(3);
    let mut cur_elf = 0;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            top_elves.add(cur_elf);
            cur_elf = 0;
            continue;
        }
        cur_elf += line.parse::<i32>().unwrap();
    }
    top_elves.add(cur_elf);
    println!(
        "The most calories carried by an elf is {}",
        top_elves.total()
    );
}

struct TopN<T: Ord + std::iter::Sum + Copy> {
    n: usize,
    vec: Vec<T>,
}

impl<T: Ord + std::iter::Sum + Copy> TopN<T> {
    fn new(n: usize) -> TopN<T> {
        TopN { n: n, vec: vec![] }
    }

    fn add(&mut self, val: T) {
        if self.vec.len() < self.n {
            self.vec.push(val);
        } else {
            let min_index = self
                .vec
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(index, _)| index)
                .unwrap();
            if val > self.vec[min_index] {
                self.vec[min_index] = val;
            }
        }
    }

    fn total(&self) -> T {
        self.vec.iter().copied().sum()
    }
}
