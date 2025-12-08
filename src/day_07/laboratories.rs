use std::fs;
use crate::common::solution::Part;
use std::str::Lines;
use std::iter::FromIterator;

const ACTUAL_INPUT_FILE: &str = "src/day_07/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_07/resources/demo_input.txt";

const DEBUG: bool = true;

fn print_char_vector(vector: &Vec<char>) {
    println!("{}", String::from_iter(vector));
}

fn print_int_vector(vector: &Vec<i64>) {
    let result: String = vector.iter().map(|&x| format!("{:x}", x)).collect();
    println!("{}", result);
}

fn solve_part1(lines: Lines) -> i64 {
    let mut resultant: Vec<char> = lines.clone().nth(1).unwrap().chars().collect();
    let mut split_counter = 0;

    for (line_idx, line) in lines.enumerate() {
        if DEBUG {
            if line_idx % 2 == 0 {
                println!("{}", line);
            } else {
                print_char_vector(&resultant);
            }
        }
        for (ch_idx, ch) in line.chars().enumerate() {
            if ch == 'S' {
                resultant[ch_idx] = '|';
            }
            if ch == '^' && resultant[ch_idx] == '|' {
                split_counter += 1;
                resultant[ch_idx-1] = '|';
                resultant[ch_idx+1] = '|';
                resultant[ch_idx] = '.';
            }
        }
    }

    return split_counter;
}

fn solve_part2(lines: Lines) -> i64 {
    let mut resultant: Vec<i64> = Vec::<i64>::new();
    
    for _ in lines.clone().nth(0).unwrap().chars() {
        resultant.push(0);
    }

    for (line_idx, line) in lines.enumerate() {
        if DEBUG {
            if line_idx % 2 == 0 {
                println!("{}", line);
            } else {
                print_int_vector(&resultant);
            }
        }
        for (ch_idx, ch) in line.chars().enumerate() {
            if ch == 'S' {
                resultant[ch_idx] += 1;
            }
            if ch == '^' && resultant[ch_idx] > 0 {
                resultant[ch_idx-1] += resultant[ch_idx];
                resultant[ch_idx+1] += resultant[ch_idx];
                resultant[ch_idx] = 0;
            }
        }
    }

    let mut accumulator = 0;

    for result in resultant {
        accumulator += result;
    }

    return accumulator;
}

pub fn solve(_part: Part, demo: bool) {
    println!("");
    println!("---------------------------");
    println!("Day 07: Laboratories");
    println!("---------------------------");
    println!("");

    let mut input_file = ACTUAL_INPUT_FILE;
    if demo {
        input_file = DEMO_INPUT_FILE;
    }

    let binding = fs::read_to_string(input_file).unwrap();
    let part1_result = solve_part1(binding.lines());
    let part2_result = solve_part2(binding.lines());

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
    println!("---------------------------");
}
