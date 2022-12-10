#![feature(iter_array_chunks)]

use std::{
    io::{BufReader, BufRead},
    fs::File, char,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_priority = 0;
    for elves_group_res in reader.lines().array_chunks::<3>() {
        let elves_group = elves_group_res.map(|backpack_res| backpack_res.unwrap());
        total_priority += get_group_badge(elves_group).and_then(item_to_priority).unwrap();
    }

    println!("The sum of priorities of the items is {}.", total_priority);
}

fn get_group_badge(elves_group: [String; 3]) -> Option<char> {
    // This can be done in O(n) instead of O(n^3) using counting.
    for first_elve_item in elves_group[0].chars() {
        for second_elve_item in elves_group[1].chars() {
            for third_elve_item in elves_group[2].chars() {
                if first_elve_item == second_elve_item && second_elve_item == third_elve_item {
                    return Some(first_elve_item);
                }
            }
        }
    }

    None
}

fn item_to_priority(item: char) -> Option<i32> {
    let item_number = item as i32;
    if 97 <= item_number && item_number <= 122 {
        Some(item_number - 97 + 1)
    }
    else if 65 <= item_number && item_number <= 90 {
        Some(item_number - 65 + 27)
    }
    else {
        None
    }
}
