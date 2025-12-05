extern crate itertools;
use crate::common::solution::Part;

use std::fs;
use itertools::Itertools;

const ACTUAL_INPUT_FILE: &str = "src/day_02/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_02/resources/demo_input.txt";

fn all_elements_same(arr: &Vec<String>) -> bool {
    let mut same = true;
    for x in arr.iter() {
        if *x != arr[0] {
            same = false;
            break;
        }
    }
    return same;
    // arr.iter().all(|x| *x == arr[0])
}

fn test_invalid_id(id_str: &str, chunk_size: usize) -> bool {
    if chunk_size <= 0 {
        return false;
    }
    let chunks: Vec<String> = id_str.chars()
        .chunks(chunk_size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>();
    
    all_elements_same(&chunks)
}

fn get_invalid_id_part1(id: &u64) -> u64 {
    let id_str = id.to_string();
    let id_len = id_str.len();
    if id_len % 2 == 0 && test_invalid_id(&id_str, id_len / 2) {
        return *id;
    }
    return 0;
}

fn get_invalid_id_part2(id: &u64) -> u64 {
    let id_str = id.to_string();
    let id_len: usize = id_str.len();

    if id_len < 2 {
        return 0;
    }

    let chunk_range = 1..=(id_len / 2);
    let invalid_id = chunk_range.rev()
        .filter(|n| id_len % n == 0)
        .any(|n| test_invalid_id(&id_str, n));

    if invalid_id {
        return *id;
    }
    // for chunk_size in 1..=(id_len / 2) {
    //     if id_len % chunk_size == 0 && test_invalid_id(&id_str, chunk_size) {
    //         return *id;
    //     }
    // }

    return 0;
}


pub fn solve(_part: Part, demo: bool) {
    println!("");
    println!("-----------------------");
    println!("Day 02: Gift Shop");
    println!("-----------------------");
    println!("");

    let mut input_file = ACTUAL_INPUT_FILE;
    if demo {
        input_file = DEMO_INPUT_FILE;
    }

    let input = fs::read_to_string(input_file).unwrap();

    let mut part1_result = 0;
    let mut part2_result = 0;
    
    for range in input.split(',') {

        let mut range_entries = range.split('-');
        let start: u64 = range_entries.next().unwrap().parse().unwrap();
        let end: u64 = range_entries.next().unwrap().parse().unwrap();

        for id in start..=end {
            part1_result += get_invalid_id_part1(&id);
            part2_result += get_invalid_id_part2(&id);
        }
    }

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
    println!("-----------------------");
}
