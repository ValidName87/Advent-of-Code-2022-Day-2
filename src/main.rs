use std::{fs, str::SplitAsciiWhitespace, thread::ScopedJoinHandle};

fn main() {
    let input_text = fs::read_to_string("input.txt").expect("Couldn't read the input!");

    let points = input_text
        .split("\n")
        .map(|s| {
            let mut chars = s.chars();
            (
                {
                    match chars.next().unwrap() {
                        'A' => Move::Rock,
                        'B' => Move::Paper,
                        'C' => Move::Scissors,
                        _ => panic!("Fist move should be A, B, or C!"),
                    }
                },
                {
                    match {
                        chars.next();
                        chars.next().unwrap()
                    } {
                        'X' => Move::Rock,
                        'Y' => Move::Paper,
                        'Z' => Move::Scissors,
                        _ => panic!("Second move should be X, Y, or Z!"),
                    }
                },
            )
        })
        .map(|moves| play_game(&moves.1, &moves.0))
        .sum::<i32>();

    println!("The total points**: {}", points);
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

    fn get_outcome(other_move: &Move, outcome: &Outcome) -> Move {
        match other_move {
            Move::Rock => match outcome {
                Outcome::Win => Move::Paper,
                Outcome::Draw => Move::Rock,
                Outcome::Loss => Move::Scissors,
            },
            Move::Paper => match outcome {
                Outcome::Win => Move::Scissors,
                Outcome::Draw => Move::Paper,
                Outcome::Loss => Move::Rock,
            },
            Move::Scissors => match outcome {
                Outcome::Win => Move::Rock,
                Outcome::Draw => Move::Scissors,
                Outcome::Loss => Move::Paper,
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
