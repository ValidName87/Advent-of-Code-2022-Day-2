use std::fs;

fn main() {
    let input_text = fs::read_to_string("input.txt");
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn point_value(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn play_against(&self, other_move: &Move) -> Outcome {
        match self {
            Move::Rock => match other_move {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Loss,
                Move::Scissors => Outcome::Win,
            },
            Move::Paper => match other_move {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Loss,
            },
            Move::Scissors => match other_move {
                Move::Rock => Outcome::Loss,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Draw,
            },
        }
    }

    fn point_value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn play_game(your_move: &Move, other_move: &Move) -> i32 {
    your_move.point_value() + your_move.play_against(other_move).point_value()
}
