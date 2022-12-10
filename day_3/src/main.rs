use std::{
    io::{BufReader, BufRead},
    fs::File,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_priority = 0;
    for line in reader.lines() {
        let backpack = line.unwrap();
        total_priority += get_repeated_item(&backpack).and_then(item_to_priority).unwrap();
    }

    println!("The sum of priorities of the items is {}.", total_priority);
}

fn get_repeated_item(backpack: &str) -> Option<char> {
    let first_compartment = &backpack[.. backpack.len()/2];
    let second_compartment = &backpack[backpack.len()/2 ..];

    // This can be done in O(n) with bucket sort or a hash map, instead of O(n^2)
    // like here.
    for item in first_compartment.chars() {
        if second_compartment.chars().any(|possible_item| item == possible_item) {
            return Some(item);
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
