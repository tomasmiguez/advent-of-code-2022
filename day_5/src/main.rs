use std::io::{self, Write};

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    for stack in moved_stacks(INPUT) {
        print!("{}", stack.last().unwrap());
    }
    print!("\n");
    io::stdout().flush().unwrap();
}

fn moved_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = initial_stacks(input);

    for instruction in input.lines().filter(|line| line.contains("move")) {
        let instruction_values: Vec<usize> = instruction.split(' ')
                                                     .filter(|str| str.chars().all(char::is_numeric))
                                                     .filter_map(|str| str.parse().ok())
                                                     .collect();

        for _ in 0..instruction_values[0] {
            let cargo = stacks[instruction_values[1]-1].pop().unwrap();
            stacks[instruction_values[2]-1].push(cargo);
        }
    }

    stacks
}

fn initial_stacks(input: &str) -> Vec<Vec<char>> {
    let mut initial_stacks = Vec::new();
    let number_of_stacks = number_of_stacks(input).unwrap();
    for _ in 0..number_of_stacks {
        initial_stacks.push(Vec::new());
    }

    for line in input.lines() {
        if !line.contains("[") {
            break;
        }

        let line_vec: Vec<char> = line.chars().collect();
        for i in 0..number_of_stacks {
            if let Some(&cargo) = line_vec.get(1+i*4) {
                if cargo != ' ' {
                    initial_stacks[i].push(cargo);
                }
            }
        }
    }

    for i in 0..number_of_stacks {
        initial_stacks[i].reverse();
    }

    initial_stacks
}

fn number_of_stacks(input: &str) -> Option<usize> {
    for line in input.lines() {
        if !line.contains("[") {
            return Some(line.split(' ')
                            .filter(|str| !str.is_empty())
                            .count());
        }
    }

    None
}
