use std::{io::{BufReader, BufRead}, fs::File, ops::Range};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_subsections = 0;
    for line in reader.lines().map(|line| line.unwrap()) {
        let numbers: Vec<u32> = line.split(&['-', ',']).map(|s| s.parse::<u32>().unwrap()).collect();
        let first_sections = numbers[0] .. numbers[1]+1;
        let second_sections = numbers[2] .. numbers[3]+1;

        if check_if_subsections(&first_sections, &second_sections) || check_if_subsections(&second_sections, &first_sections){
            total_subsections += 1;
        }
    }

    println!("The number of tota subsections found is {}.", total_subsections);
}

fn check_if_subsections(first_sections: &Range<u32>, second_sections: &Range<u32>) -> bool {
    for section in first_sections.clone() {
        if !second_sections.contains(&section) {
            return false;
        }
    }

    true
}
