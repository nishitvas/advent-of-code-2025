use std::fs;
use crate::common::solution::Part;

const ACTUAL_INPUT_FILE: &str = "src/day_04/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_04/resources/demo_input.txt";


struct Point(i32, i32);

const DIRECTIONS: [Point; 8] = [
    Point(-1, -1),
    Point(-1, 0),
    Point(-1, 1),
    Point(0, -1),
    Point(0, 1),
    Point(1, -1),
    Point(1, 0),
    Point(1, 1),
];

fn solve_for_movable_rolls(floor_plan: &mut Vec::<Vec<char>>, max_rolls: usize) -> u32 {
    let mut movable_rolls = Vec::<Point>::new();
    let mut i = 0;
    while i < floor_plan.len() {
        let mut j = 0;
        while j < floor_plan[i].len() {
            if floor_plan[i][j] != '@' {
                j += 1;
                continue;
            }
            let mut surrounding_rolls = 0;
            for dir in DIRECTIONS.iter() {
                let Point(_x, _y) = &dir;
                let x = (i as i32) + _x;
                let y = (j as i32) + _y;

                // Check out of bounds
                if x < 0 || x >= (floor_plan.len() as i32) {
                    continue;
                }

                if y < 0 || y >= (floor_plan[i].len() as i32) {
                    continue;
                }

                if floor_plan[x as usize][y as usize] == '@' {
                    surrounding_rolls += 1;
                }
            }
            if surrounding_rolls < max_rolls {
                // println!("[{},{}]", i, j);
                movable_rolls.push(Point(i as i32, j as i32))
            }
            j += 1;
        }
        i += 1;
    }
    
    for point in &movable_rolls {
        let Point(x, y) = &point;
        floor_plan[*x as usize][*y as usize] = 'x';
    }

    return movable_rolls.len() as u32;
}

pub fn solve(_part: Part, demo: bool) {
    println!("");
    println!("---------------------------");
    println!("Day 04: Printing Department");
    println!("---------------------------");
    println!("");

    let mut input_file = ACTUAL_INPUT_FILE;
    if demo {
        input_file = DEMO_INPUT_FILE;
    }

    let binding = fs::read_to_string(input_file).unwrap();

    // let mut part1_result = 0;
    // let mut part2_result = 0;

    let mut floor_plan = Vec::<Vec<char>>::new();
    let max_allowed_surrounding_rolls = 4;

    for line in binding.lines() {
        floor_plan.push(line.chars().collect());
    }

    let mut part1_result = 0;
    let mut part2_result = 0;

    let mut result = solve_for_movable_rolls(&mut floor_plan, max_allowed_surrounding_rolls);
    part1_result += result;
    part2_result += result;

    while result > 0 {
        result = solve_for_movable_rolls(&mut floor_plan, max_allowed_surrounding_rolls);
        part2_result += result;
    }

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
    println!("---------------------------");
}
