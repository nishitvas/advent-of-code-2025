use std::fs;

const STARTING_POS: i16 = 50;
const DIAL_MAX: i16 = 100;

const ACTUAL_INPUT_FILE: &str = "src/day_01/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_01/resources/demo_input.txt";


fn extract_clicks(instruction: &str) -> i16 {
    let (direction, _clicks) = instruction.split_at(1);
    let mut clicks: i16 = _clicks.parse().expect("Expected clicks to be number");
    if direction == "L" {
        clicks *= -1;
    }
    return clicks;
}

struct DialResult {
    position: i16,
    loops: i16,
}

fn turn_dial(start_position: i16, instruction: &str) -> DialResult {
    let clicks = extract_clicks(instruction);
    let end_position = start_position + clicks;
    let mut loops = end_position / DIAL_MAX;
    
    if end_position == 0 {
        // Exactly at 0
        loops = 1;
    } else if end_position < 0 {
        // Negative loops crossed 0
        loops *= -1;
        if start_position != 0 {
            loops += 1
        }
    }

    // println!("{instruction}, {start_position} -> {loops}, {end_position}");
    // https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
    // let new_position = end_position % DIAL_MAX;
    return DialResult {
        position: end_position.rem_euclid(DIAL_MAX),
        loops: loops
    }
}

pub fn solve(_: u8, demo: bool) {
    println!("");
    println!("-----------------------");
    println!("Day 01: Secret Entrance");
    println!("-----------------------");
    println!("");

    let mut position = STARTING_POS;
    let mut part1_result = 0;
    let mut part2_result = 0;

    let mut input_file = ACTUAL_INPUT_FILE;
    if demo {
        input_file = DEMO_INPUT_FILE;
    }

    let binding = fs::read_to_string(input_file).unwrap();
    for instruction in binding.lines() {
        let dial_result = turn_dial(position, instruction);
        position = dial_result.position;
        if position == 0 {
            part1_result += 1
        }
        part2_result += dial_result.loops;
    }
    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
    println!("-----------------------");
}