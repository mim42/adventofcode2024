use std::fs::read_to_string;
extern crate fxhash;
use fxhash::{FxHashMap, FxHashSet};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    input
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn possible_coords_a(
    coords: Vec<(usize, usize)>,
    x_bound: usize,
    y_bound: usize,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (i, first_coord) in coords.iter().enumerate() {
        for (j, second_coord) in coords.iter().enumerate() {
            if i != j {
                let first_x_diff = (second_coord.0 as i64 - first_coord.0 as i64) * 2;
                let first_y_diff = (second_coord.1 as i64 - first_coord.1 as i64) * 2;

                let a = (
                    first_coord.0 as i64 + first_x_diff,
                    first_coord.1 as i64 + first_y_diff,
                );
                if (0..x_bound as i64).contains(&a.0) && (0..y_bound as i64).contains(&a.1) {
                    result.push((a.0 as usize, a.1 as usize));
                }
            }
        }
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let input = parse_input(input);
    let mut coords_map: FxHashMap<String, Vec<(usize, usize)>> = FxHashMap::default();
    let mut unique_coords: FxHashSet<(usize, usize)> = FxHashSet::default();

    for (i, line) in input.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            if letter != "." {
                let coords = coords_map.entry(letter.clone()).or_insert(vec![]);
                coords.push((i, j));
            }
        }
    }

    for (_, coords) in coords_map {
        for i in possible_coords_a(coords, input.len(), input[0].len()) {
            unique_coords.insert(i);
        }
    }

    unique_coords.len() as u64
}

fn possible_coords_b(
    coords: Vec<(usize, usize)>,
    x_bound: usize,
    y_bound: usize,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    if coords.len() > 1 {
        result = coords.to_vec();
    }
    for (i, first_coord) in coords.iter().enumerate() {
        for (j, second_coord) in coords.iter().enumerate() {
            if i != j {
                let mut multi = 2;
                loop {
                    let first_x_diff = (second_coord.0 as i64 - first_coord.0 as i64) * multi;
                    let first_y_diff = (second_coord.1 as i64 - first_coord.1 as i64) * multi;

                    let a = (
                        first_coord.0 as i64 + first_x_diff,
                        first_coord.1 as i64 + first_y_diff,
                    );
                    if (0..x_bound as i64).contains(&a.0) && (0..y_bound as i64).contains(&a.1) {
                        result.push((a.0 as usize, a.1 as usize));
                        multi += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    result
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let input = parse_input(input);
    let mut coords_map: FxHashMap<String, Vec<(usize, usize)>> = FxHashMap::default();
    let mut unique_coords: FxHashSet<(usize, usize)> = FxHashSet::default();
    for (i, line) in input.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            if letter != "." {
                let coords = coords_map.entry(letter.clone()).or_insert(vec![]);
                coords.push((i, j));
            }
        }
    }

    for (_, coords) in coords_map {
        for i in possible_coords_b(coords, input.len(), input[0].len()) {
            unique_coords.insert(i);
        }
    }

    unique_coords.len() as u64
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a = solve_part_a(&input);
    println!("result of part a {}", result_part_a);
    let result_part_b = solve_part_b(&input);
    println!("result of part b {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(14, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(34, solve_part_b(&example));
    }
}
