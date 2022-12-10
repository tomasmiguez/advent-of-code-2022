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
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _   => panic!("Invalid playable char!"),
    }
}

enum Result {
    Lose,
    Draw,
    Win,
}

use crate::Result::*;

fn letter_to_wanted_result(letter: char) -> Result {
    match letter {
        'X' => Lose,
        'Y' => Draw,
        'Z' => Win,
        _   => panic!("Invalid result char!"),
    }
}

fn match_score(r#match: String) -> i32 {
    let oponent_play = letter_to_playable(r#match.chars().nth(0).unwrap());
    let player_play = letter_to_wanted_result(r#match.chars().nth(2).unwrap());

    match oponent_play {
        Rock => {
            match player_play {
                Lose => 0 + 3,
                Draw => 3 + 1,
                Win => 6 + 2,
            }
        },
        Paper => {
            match player_play {
                Lose => 0 + 1,
                Draw => 3 + 2,
                Win => 6 + 3,
            }
        },
        Scissors => {
            match player_play {
                Lose => 0 + 2,
                Draw => 3 + 3,
                Win => 6 + 1,
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
