use std::fs;

pub fn run() {
    let data = fs::read_to_string("data/day2.txt").unwrap();
    let lines = data.lines();

    let mut score = 0;
    for line in lines {
        let line = line.as_bytes();
        let opp_move = parse_move(line[0]).unwrap();
        let my_move = parse_my_move(line[2], opp_move).unwrap();
        score += my_move.score(opp_move);
    }
    println!("{score}");
}

#[derive(PartialEq, Clone, Copy)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

fn parse_move(c: u8) -> Option<RPSMove> {
    let c = char::from(c);
    match c {
        'A' | 'X' => Some(RPSMove::Rock),
        'B' | 'Y' => Some(RPSMove::Paper),
        'C' | 'Z' => Some(RPSMove::Scissors),
        _ => None,
    }
}

fn parse_my_move(c: u8, opp_move: RPSMove) -> Option<RPSMove> {
    let c = char::from(c);
    match c {
        'Y' => Some(opp_move),
        'X' => match (opp_move) {
            RPSMove::Rock => Some(RPSMove::Scissors),
            RPSMove::Paper => Some(RPSMove::Rock),
            RPSMove::Scissors => Some(RPSMove::Paper),
        },
        'Z' => match (opp_move) {
            RPSMove::Rock => Some(RPSMove::Paper),
            RPSMove::Paper => Some(RPSMove::Scissors),
            RPSMove::Scissors => Some(RPSMove::Rock),
        },
        _ => None,
    }
}

impl RPSMove {
    fn score(self, opp_move: RPSMove) -> i32 {
        let mut score = self.win_score(opp_move);
        score += match self {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissors => 3,
        };
        score
    }

    fn win_score(self, opp_move: RPSMove) -> i32 {
        if self == opp_move {
            return 3;
        }
        if self == RPSMove::Rock && opp_move == RPSMove::Scissors
            || self == RPSMove::Paper && opp_move == RPSMove::Rock
            || self == RPSMove::Scissors && opp_move == RPSMove::Paper
        {
            return 6;
        }
        return 0;
    }
}
