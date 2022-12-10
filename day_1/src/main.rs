use core::num;
use std::{
    io::BufReader,
    io::BufRead,
    fs::File,
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut current_elf_calories = 0;
    // We could instead store the current top 3 in each iteration,
    // which would be O(n) instead of O(n*logn)
    let mut elves_calories = Vec::new();
    for line in reader.lines() {
        match line.unwrap().parse::<i32>() {
            Ok(calories) => {
                current_elf_calories += calories;
            },
            Err(ref e) if e.kind() == &num::IntErrorKind::Empty => {
                elves_calories.push(current_elf_calories);
                current_elf_calories = 0;
            },
            Err(e) => panic!("Unexpected error {}.", e),
         }
    }

    elves_calories.sort();
    let top_three_elves_calories: i32 = (&elves_calories[elves_calories.len()-3 ..]).iter().sum();

    println!("Top three elves carried {} calories.", top_three_elves_calories);

    Ok(())
}
