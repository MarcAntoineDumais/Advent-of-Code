use std::collections::VecDeque;

pub fn run() {
    let mut monkeys = vec![
        Monkey {
            items: VecDeque::from(vec![54, 89, 94]),
            op: |x| x * 7,
            test: |x| match x % 17 {
                0 => 5,
                _ => 3,
            },
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![66, 71]),
            op: |x| x + 4,
            test: |x| match x % 3 {
                0 => 0,
                _ => 3,
            },
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![76, 55, 80, 55, 55, 96, 78]),
            op: |x| x + 2,
            test: |x| match x % 5 {
                0 => 7,
                _ => 4,
            },
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![93, 69, 76, 66, 89, 54, 59, 94]),
            op: |x| x + 7,
            test: |x| match x % 7 {
                0 => 5,
                _ => 2,
            },
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![80, 54, 58, 75, 99]),
            op: |x| x * 17,
            test: |x| match x % 11 {
                0 => 1,
                _ => 6,
            },
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![69, 70, 85, 83]),
            op: |x| x + 8,
            test: |x| match x % 19 {
                0 => 2,
                _ => 7,
            },
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![89]),
            op: |x| x + 6,
            test: |x| match x % 2 {
                0 => 0,
                _ => 1,
            },
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![62, 80, 58, 57, 93, 56]),
            op: |x| x * x,
            test: |x| match x % 13 {
                0 => 6,
                _ => 4,
            },
            count: 0,
        },
    ];

    // let mut monkeys = vec![
    //     Monkey {
    //         items: VecDeque::from(vec![79, 98]),
    //         op: |x| x * 19,
    //         test: |x| match x % 23 {
    //             0 => 2,
    //             _ => 3,
    //         },
    //         count: 0,
    //     },
    //     Monkey {
    //         items: VecDeque::from(vec![54, 65, 75, 74]),
    //         op: |x| x + 6,
    //         test: |x| match x % 19 {
    //             0 => 2,
    //             _ => 0,
    //         },
    //         count: 0,
    //     },
    //     Monkey {
    //         items: VecDeque::from(vec![79, 60, 97]),
    //         op: |x| x * x,
    //         test: |x| match x % 13 {
    //             0 => 1,
    //             _ => 3,
    //         },
    //         count: 0,
    //     },
    //     Monkey {
    //         items: VecDeque::from(vec![74]),
    //         op: |x| x + 3,
    //         test: |x| match x % 17 {
    //             0 => 0,
    //             _ => 1,
    //         },
    //         count: 0,
    //     },
    // ];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            loop {
                let (finished, item, index) = monkeys[i].inspect();
                if finished {
                    break;
                }

                monkeys[index].items.push_back(item);
            }
        }
    }

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i}: {}", monkey.count);
    }
}

struct Monkey {
    items: VecDeque<u128>,
    op: fn(u128) -> u128,
    test: fn(u128) -> usize,
    count: u128,
}

impl Monkey {
    fn inspect(&mut self) -> (bool, u128, usize) {
        match self.items.pop_front().map(self.op).map(|item| {
            self.count += 1;
            //let item = item / 3;
            let item = item % 9699690;
            (false, item, (self.test)(item))
        }) {
            Some(x) => x,
            None => (true, 0, 0),
        }
    }
}
