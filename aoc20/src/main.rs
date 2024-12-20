use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut racetrack: Vec<Vec<String>> = Vec::new();

    for i in input {
        let a = i.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        racetrack.push(a);
    }
    racetrack
}

fn find_start_finish(racetrack: &Vec<Vec<String>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut finish = (0, 0);
    for (y, row) in racetrack.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value == "S" {
                start = (y, x);
            } else if value == "E" {
                finish = (y, x);
            };
        }
    }
    (start, finish)
}

fn racetrack_points(racetrack: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    let mut racetrack_points = Vec::new();
    for (y, row) in racetrack.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value != "#" {
                racetrack_points.push((y, x));
            }
        }
    }
    racetrack_points
}

fn walk_racetrack(
    racetrack_points: &Vec<(usize, usize)>,
    racetrack_distances: &mut HashMap<(usize, usize), usize>,
    starting_point: (usize, usize),
    finish_point: (usize, usize),
) {
    let mut current_point = starting_point;
    let mut walk_counter = 0;
    racetrack_distances.insert(current_point, walk_counter);
    let ops = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    loop {
        for op in ops {
            let next_y = (current_point.0 as isize + op.0) as usize;
            let next_x = (current_point.1 as isize + op.1) as usize;
            if racetrack_points.contains(&(next_y, next_x)) {
                if !racetrack_distances.contains_key(&(next_y, next_x)) {
                    walk_counter += 1;
                    racetrack_distances.insert((next_y, next_x), walk_counter);
                    current_point = (next_y, next_x);
                    break;
                }
            }
        }
        if current_point == finish_point {
            break;
        }
    }
}

fn find_all_cheats(
    racetrack: &Vec<Vec<String>>,
    racetrack_points: &Vec<(usize, usize)>,
    seconds: usize,
) -> Vec<((usize, usize), (usize, usize))> {
    let mut cheats = Vec::new();
    for y in 0..racetrack.len() {
        for x in 0..racetrack[0].len() {
            if racetrack[y][x] != "#" {
                for (point_y, point_x) in racetrack_points {
                    if y.abs_diff(*point_y) + x.abs_diff(*point_x) <= seconds {
                        cheats.push(((y, x), (*point_y, *point_x)));
                    }
                }
            }
        }
    }
    cheats
}

fn count_cheats(racetrack: &Vec<Vec<String>>, cheat_time: usize) -> usize {
    let racetrack_points = racetrack_points(&racetrack);
    let (start, finish) = find_start_finish(&racetrack);
    let mut racetrack_distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut counter = 0;
    walk_racetrack(&racetrack_points, &mut racetrack_distances, start, finish);
    let total_racetrack_length = racetrack_distances.get(&finish).unwrap().clone();
    let cheats = find_all_cheats(&racetrack, &racetrack_points, cheat_time);
    for (cheat_start, cheat_finish) in cheats {
        let length = racetrack_distances.get(&cheat_start).unwrap()
            + cheat_start.0.abs_diff(cheat_finish.0)
            + cheat_start.1.abs_diff(cheat_finish.1)
            + total_racetrack_length.abs_diff(*racetrack_distances.get(&cheat_finish).unwrap());
        if total_racetrack_length >= length + 100 {
            counter += 1
        }
    }
    counter
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let racetrack = parse_input(input);
    let cheat_time = 2;
    count_cheats(&racetrack, cheat_time)
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let racetrack = parse_input(input);
    let cheat_time = 20;
    count_cheats(&racetrack, cheat_time)
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
        assert_eq!(0, solve_part_a(&example));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(0, solve_part_b(&example));
    }
}
