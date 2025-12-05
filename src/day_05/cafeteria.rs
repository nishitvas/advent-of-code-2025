use std::fs;
use crate::common::solution::Part;
use std::cmp;

const ACTUAL_INPUT_FILE: &str = "src/day_05/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_05/resources/demo_input.txt";

#[derive(Clone)]
#[derive(Debug)]
#[derive(Ord)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(PartialEq)]
struct InclusiveRange {
    start: i64,
    end: i64,
}

impl InclusiveRange {
    fn contains(&self, num: i64) -> bool {
        return num >= self.start && num <= self.end;
    }
}

fn solve_part2(fresh_ranges: &mut Vec::<InclusiveRange>) -> i64 {
    let mut merged_ranges = Vec::<InclusiveRange>::new();
    fresh_ranges.sort();
    for range in fresh_ranges {
        let mut unique_range = true;
        let mut modified = false;
        let mut min_start = i64::MAX;
        let mut max_end = 0;
        let mut removal_indices = Vec::<usize>::new();
        for (idx, merged_range) in merged_ranges.clone().into_iter().enumerate() {
            if merged_range.contains(range.start) ^ merged_range.contains(range.end) {
                if merged_range.contains(range.start) {
                    min_start = cmp::min(min_start, merged_range.start);
                    max_end = cmp::max(max_end, range.end);
                }

                if merged_range.contains(range.end) {
                    min_start = cmp::min(min_start, range.start);
                    max_end = cmp::max(max_end, merged_range.end);
                }
                
                removal_indices.push(idx);
                modified = true;
            }

            if merged_range.contains(range.start) || merged_range.contains(range.end) {
                unique_range = false;
            }

            if range.start < merged_range.start && range.end > merged_range.end {
                min_start = range.start;
                max_end = range.end;
                removal_indices.push(idx);
                unique_range = false;
                modified = true;
            }
        }
        for idx in removal_indices.into_iter().rev() {
            // println!("{:?} -> {}", merged_ranges, idx);
            merged_ranges.remove(idx);
        }

        if modified {
            let new_range = InclusiveRange{start: min_start, end: max_end};
            // println!("New range: {:?}", new_range);
            merged_ranges.push(new_range);
        }

        if unique_range {
            merged_ranges.push(range.clone());
        }
    }

    let mut counter = 0;
    for merged_range in merged_ranges {
        println!("Range: {:?}", merged_range);
        counter += merged_range.end - merged_range.start + 1;
    }

    return counter;
}

pub fn solve(_part: Part, demo: bool) {
    println!("");
    println!("---------------------------");
    println!("Day 05: Cafeteria");
    println!("---------------------------");
    println!("");

    let mut input_file = ACTUAL_INPUT_FILE;
    if demo {
        input_file = DEMO_INPUT_FILE;
    }

    let binding = fs::read_to_string(input_file).unwrap();

    let mut parsing_range = true;

    let mut fresh_ranges = Vec::<InclusiveRange>::new();
    let mut fresh_id_counter = 0;

    for line in binding.lines() {
        if line == "" {
            parsing_range = false;
            continue;
        }

        if parsing_range {
            let mut range_entries = line.split('-');
            let start: i64 = range_entries.next().unwrap().parse().unwrap();
            let end: i64 = range_entries.next().unwrap().parse().unwrap();
            fresh_ranges.push(InclusiveRange{start, end});
        } else {
            let ingredient_id = line.parse().unwrap();

            for range in &fresh_ranges {
                if range.contains(ingredient_id) {
                    fresh_id_counter += 1;
                    break;
                }
            }
        }
    }

    let part2_result = solve_part2(&mut fresh_ranges);
    

    println!("Part 1: {fresh_id_counter}");
    println!("Part 2: {part2_result}");
    println!("---------------------------");
}
