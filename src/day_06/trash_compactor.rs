use std::fs;
use crate::common::solution::Part;
use std::str::Lines;

const ACTUAL_INPUT_FILE: &str = "src/day_06/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_06/resources/demo_input.txt";

const DEBUG_LEN: usize = 0;

fn solve_part1(lines: Lines) -> i64 {
    let mut operators = Vec::<char>::new();
    let mut resultant = Vec::<i64>::new();

    let mut first_line = true;

    for line in lines.rev() {
        for (idx, item) in line.split_whitespace().enumerate() {
            if first_line {
                let op = item.chars().nth(0).unwrap();
                operators.push(op);
                resultant.push(if op == '*' {1} else {0});
            } else {
                let num: i64 = item.parse().unwrap();
                if operators[idx] == '*' {
                    resultant[idx] *= num;
                } else {
                    resultant[idx] += num;
                }
            }
        }
        first_line = false;
    }

    let mut total_result: i64 = 0;
    for result in resultant {
        total_result += result;
    }

    return total_result;
}

fn solve_part2(lines: Lines) -> i64 {
    let mut operators = Vec::<char>::new();
    let mut digit_spaces = Vec::<usize>::new();

    let mut reversed_lines = lines.clone().rev();
    let operator_line = reversed_lines.nth(0).unwrap();
    let operand_lines: Vec<_> = reversed_lines.skip(0).collect();

    let mut space_counter = 0;
    for op in operator_line.chars() {
        if op == '*' || op == '+' {
            if space_counter != 0 {
                digit_spaces.push(space_counter);
                space_counter = 0;
            }
            operators.push(op);
        } else {
            space_counter += 1;
        }
    }
    digit_spaces.push(space_counter + 1);

    let mut total_result: i64 = 0;

    let mut position = 0;
    for (idx, digit_space) in digit_spaces.into_iter().enumerate() {
        let mut result: i64 = if operators[idx] == '*' {1} else {0};
        for i in 0..digit_space {
            let mut digit = String::from("");
            for line in &operand_lines {
                digit.insert(0, line.chars().nth(position + i).unwrap());
            }
            if idx < DEBUG_LEN {
                println!("{} {}", operators[idx], digit);
            }
            let digit = digit.trim().parse::<i64>().unwrap();
            if operators[idx] == '*' {
                result *= digit;
            } else {
                result += digit;
            }
        }
        if idx < DEBUG_LEN {
            println!("-------");
            println!("{}", result);
            println!("=======");
            println!("");
        }
        position += digit_space + 1;
        total_result += result;
    }

    return total_result;
}

pub fn solve(_part: Part, demo: bool) {
    println!("");
    println!("---------------------------");
    println!("Day 06: Trash Compactor");
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
