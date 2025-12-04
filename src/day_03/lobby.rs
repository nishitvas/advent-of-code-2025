use std::fs;

const ACTUAL_INPUT_FILE: &str = "src/day_03/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_03/resources/demo_input.txt";

#[derive(Debug)]
struct Battery {
    joltage: u64,
    index: usize,
}

fn find_max_battery(bank_part: &str) -> Battery {
    let mut max_battery: Battery = Battery {
        joltage: 0,
        index: usize::MAX,
    };

    for (index, char) in bank_part.chars().enumerate() {
        let joltage = char.to_digit(10).unwrap() as u64;
        if max_battery.joltage < joltage {
            max_battery.joltage = joltage;
            max_battery.index = index;
        }
    }

    return max_battery;
}

fn find_max_joltage(bank: &str, num_of_batteries: usize) -> u64 {
    if num_of_batteries == 0 {
        return 0;
    }

    let max_index = bank.len() - num_of_batteries;
    let search_space = &bank[0..=max_index];
    println!("search_space: {}", search_space);
    let max_battery = find_max_battery(&search_space);
    println!("max: {:?}", max_battery);
    let next_index = max_battery.index + 1;

    let next_joltage = find_max_joltage(&bank[next_index..bank.len()], num_of_batteries-1);
    let multiplier = u64::pow(10, (num_of_batteries-1) as u32);
    return (multiplier * max_battery.joltage) + next_joltage;
}

pub fn solve(_part: u8, demo: bool) {
    println!("");
    println!("-----------------------");
    println!("Day 03: Lobby");
    println!("-----------------------");
    println!("");

    let mut input_file = ACTUAL_INPUT_FILE;
    if demo {
        input_file = DEMO_INPUT_FILE;
    }

    let binding = fs::read_to_string(input_file).unwrap();

    let mut part1_result = 0;
    let mut part2_result = 0;

    for line in binding.lines() {
        let max_joltage_2 = find_max_joltage(&line, 2);
        let max_joltage_12 = find_max_joltage(&line, 12);
        part1_result += max_joltage_2;
        part2_result += max_joltage_12;
    }

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
    println!("-----------------------");
}
