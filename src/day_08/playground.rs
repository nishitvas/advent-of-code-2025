use std::fs;
use crate::common::solution::Part;
use std::str::Lines;
use std::collections::HashSet;

const ACTUAL_INPUT_FILE: &str = "src/day_08/resources/actual_input.txt";
const DEMO_INPUT_FILE: &str = "src/day_08/resources/demo_input.txt";

// const DEBUG: bool = true;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i64, y: i64, z: i64,
}

#[derive(Debug, Clone, Copy)]
struct PointDistance {
    a: Point,
    b: Point,
    distance: f64,
}

fn distance(a: Point, b: Point) -> f64 {
    let dist = (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2);
    return (dist as f64).sqrt();
}

#[derive(PartialEq)]
enum CheckResult {
    HasBoth,
    HasOne,
    HasNone,
}

fn junction_check(junction: &Vec<PointDistance>, point_distance: &PointDistance) -> CheckResult {
    let mut a_found = false;
    let mut b_found = false;
    for pd in junction {
        if point_distance.a == pd.a || point_distance.a == pd.b {
            a_found = true;
        }

        if point_distance.b == pd.a || point_distance.b == pd.b {
            b_found = true;
        }
    }
    if a_found && b_found {
        return CheckResult::HasBoth;
    } else if a_found || b_found {
        return CheckResult::HasOne;
    }
    return CheckResult::HasNone;
}

fn junction_combine(junction1: &Vec<PointDistance>,
                    junction2: &Vec<PointDistance>) -> Option<Vec<PointDistance>> {
    for point_distance in junction1 {
        if junction_check(junction2, point_distance) == CheckResult::HasOne {
            let mut new_junction = Vec::<PointDistance>::new();
            new_junction.extend(junction2);
            new_junction.extend(junction1);
            return Some(new_junction);
        }
    }
    return None;
}

fn compute_distances(lines: Lines) -> (Vec::<PointDistance>, usize) {
    let mut points = Vec::new();
    let mut distances = Vec::<PointDistance>::new();

    // Convert input to points
    let mut count = 0;
    for line in lines {
        let mut pts = line.splitn(3, ',');
        let x: i64 = pts.next().unwrap().parse::<i64>().unwrap();
        let y: i64 = pts.next().unwrap().parse::<i64>().unwrap();
        let z: i64 = pts.next().unwrap().parse::<i64>().unwrap();
        points.push(Point {x, y, z});
        count += 1;
    }

    // Compute distance between each points
    for i in 0..(count-1) {
        for j in (i+1)..count {
            distances.push(PointDistance {
                a: points[i],
                b: points[j],
                distance: distance(points[i],points[j]),
            })
        }
    }

    // Sort points in ascending order by distance
    distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    return (distances, count);
}

fn solve_part1(lines: Lines, iterations: usize) -> i64 {
    let (distances, _line_count) = compute_distances(lines);

    // Create junctions from shortest distances
    let mut junctions = Vec::<Vec<PointDistance>>::new();
    let mut counter = 0;
    for pd in &distances {
        // println!("{:?}", pd);
        let mut insert_new = true;
        // let mut idx = 0;
        for junc in &mut junctions {
            let check_result = junction_check(junc, pd);
            if check_result == CheckResult::HasBoth {
                insert_new = false;
                // println!("Exists in junction, skipping - {}", idx);
                break;
            }
            if check_result == CheckResult::HasOne {
                junc.push(*pd);
                insert_new = false;
                // println!("Inserted into junction - {}", idx);
                break;
            }
            // idx += 1;
        }
        if insert_new {
            // println!("Inserted into new junction");
            junctions.push(vec![*pd]);
        }

        counter += 1;
        if counter > iterations-1 {
            break;
        }
    }

    // Sort junctions into ascending by length
    junctions.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

    // Combine smaller junctions into larger junctions
    // until no further junctions can be combined
    let mut combined = true;
    while combined {
        combined = false;
        let size = junctions.len();
        'outer: for i in 0..size-1 {
            for j in i+1..size {
                match junction_combine(&junctions[i], &junctions[j]) {
                    Some(x) => {
                        // println!("Combined {} and {}", i, j);
                        junctions.remove(j);
                        junctions.remove(i);
                        junctions.push(x);
                        combined = true;
                        break 'outer;
                    },
                    None => combined = false,
                }
            }
        }
    }

    // Sort junctions into descending by length
    junctions.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());

    let circuits: Vec<i64> = junctions.into_iter()
        .map(|junction| {
            let mut set = HashSet::new();
            for j in junction {
                set.insert(j.a);
                set.insert(j.b);
            }
            return set.len() as i64;
        })
        .collect();

    let init_value: i64 = 1;
    let part1_result = circuits
        .into_iter()
        .enumerate()
        .filter(|&(idx, _junc_size)| idx < 3)
        .fold(init_value, |acc, (_idx, junction_size)| acc * junction_size as i64);
    
    return part1_result;
}

fn solve_part2(lines: Lines) -> i64 {
    let (distances, line_count) = compute_distances(lines);

    // Create junctions from shortest distances
    let mut junctions = Vec::<Vec<PointDistance>>::new();
    for pd in &distances {
        let mut insert_new = true;
        // let mut idx = 0;
        for junc in &mut junctions {
            let check_result = junction_check(junc, pd);
            if check_result == CheckResult::HasBoth {
                insert_new = false;
                // println!("Exists in junction, skipping - {}", idx);
                break;
            }
            if check_result == CheckResult::HasOne {
                junc.push(*pd);
                insert_new = false;
                // println!("Inserted into junction - {}", idx);
                break;
            }
            // idx += 1;
        }
        if insert_new {
            // println!("Inserted into new junction");
            junctions.push(vec![*pd]);
        }

        // Sort junctions into ascending by length
        junctions.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        // Combine smaller junctions into larger junctions
        // until no further junctions can be combined
        let mut combined = true;
        while combined {
            combined = false;
            let size = junctions.len();
            'outer: for i in 0..size-1 {
                for j in i+1..size {
                    match junction_combine(&junctions[i], &junctions[j]) {
                        Some(x) => {
                            // println!("Combined {} and {}", i, j);
                            junctions.remove(j);
                            junctions.remove(i);
                            junctions.push(x);
                            combined = true;
                            break 'outer;
                        },
                        None => combined = false,
                    }
                }
            }
        }
        
        // Check if all junctions have connected into one
        if junctions.len() == 1 && junctions[0].len() == line_count-1 {
            return pd.a.x * pd.b.x;
        }
    }
    return 0;
}

pub fn solve(_part: Part, demo: bool) {
    println!("");
    println!("---------------------------");
    println!("Day 08: Playground");
    println!("---------------------------");
    println!("");

    let mut input_file = ACTUAL_INPUT_FILE;
    if demo {
        input_file = DEMO_INPUT_FILE;
    }

    let iterations = if demo {10} else {1000};

    let binding = fs::read_to_string(input_file).unwrap();
    let part1_result = solve_part1(binding.lines(), iterations);
    let part2_result = solve_part2(binding.lines());

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
    println!("---------------------------");
}
