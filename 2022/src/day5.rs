use std::fs;

pub fn run() {
    let data = fs::read_to_string("data/day5.txt").unwrap();
    let lines = data.lines().map(str::trim).filter(|line| !line.is_empty());

    /*
                        [M]     [V]     [L]
        [G]             [V] [C] [G]     [D]
        [J]             [Q] [W] [Z] [C] [J]
        [W]         [W] [G] [V] [D] [G] [C]
        [R]     [G] [N] [B] [D] [C] [M] [W]
        [F] [M] [H] [C] [S] [T] [N] [N] [N]
        [T] [W] [N] [R] [F] [R] [B] [J] [P]
        [Z] [G] [J] [J] [W] [S] [H] [S] [G]
        1   2   3   4   5   6   7   8   9
    */
    let mut stacks = vec![
        vec!['Z', 'T', 'F', 'R', 'W', 'J', 'G'],
        vec!['G', 'W', 'M'],
        vec!['J', 'N', 'H', 'G'],
        vec!['J', 'R', 'C', 'N', 'W'],
        vec!['W', 'F', 'S', 'B', 'G', 'Q', 'V', 'M'],
        vec!['S', 'R', 'T', 'D', 'V', 'W', 'C'],
        vec!['H', 'B', 'N', 'C', 'D', 'Z', 'G', 'V'],
        vec!['S', 'J', 'N', 'M', 'G', 'C'],
        vec!['G', 'P', 'N', 'W', 'C', 'J', 'D', 'L'],
    ];

    for line in lines {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let count = line_split[1].parse::<i32>().unwrap();
        let from = line_split[3].parse::<usize>().unwrap();
        let to = line_split[5].parse::<usize>().unwrap();

        let mut tmp: Vec<char> = vec![];
        for _ in 0..count {
            tmp.push(stacks[from - 1].pop().unwrap());
        }
        //tmp.reverse();
        for _ in 0..count {
            stacks[to - 1].push(tmp.pop().unwrap());
        }
        //println!("after: {:?}", stacks);
    }
    let mut result = "".to_owned();
    for mut stack in stacks {
        result.push_str(stack.pop().unwrap().to_string().as_str());
    }
    println!("{result}");
}
