use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
enum Playable {
    Rock,
    Paper,
    Scissors,
}

use crate::Playable::*;

fn letter_to_playable(letter: char) -> Playable {
    match letter {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissors,
        _         => panic!("Invalid playable char!"),
    }
}

fn match_score(r#match: String) -> i32 {
    let oponent_play = letter_to_playable(r#match.chars().nth(0).unwrap());
    let player_play = letter_to_playable(r#match.chars().nth(2).unwrap());

    match oponent_play {
        Rock => {
            match player_play {
                Rock => 3 + 1,
                Paper => 6 + 2,
                Scissors => 0 + 3,
            }
        },
        Paper => {
            match player_play {
                Rock => 0 + 1,
                Paper => 3 + 2,
                Scissors => 6 + 3,
            }
        },
        Scissors => {
            match player_play {
                Rock => 6 + 1,
                Paper => 0 + 2,
                Scissors => 3 + 3,
            }
        }
    }
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(it) => it,
        Err(err) => panic!("Error opening the file: {}", err),
    };
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        match line {
            Ok(r#match) => {
                total_score += match_score(r#match);
            },
            Err(err) => panic!("Error parsing line: {}", err),
        }
    }

    println!("The total score is {}.", total_score);
}
