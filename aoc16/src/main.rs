use std::collections::HashSet;
use std::{collections::HashMap, fs::read_to_string};
use std::{path, usize};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Point {
    y: usize,
    x: usize,
    direction: usize,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn serialise(point: &Point) -> String {
    String::from((point.y * 1000 + point.x).to_string() + ",")
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut parsed_maze = Vec::new();
    for i in input {
        parsed_maze.push(i.chars().map(|c| c.to_string()).collect::<Vec<String>>())
    }

    parsed_maze
}

fn find_start_end(map: &Vec<Vec<String>>) -> (Point, Point) {
    let mut start = Point {
        y: 0,
        x: 0,
        direction: 0,
    };
    let mut end = Point {
        y: 0,
        x: 0,
        direction: 0,
    };
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value == "S" {
                start.x = x;
                start.y = y;
            } else if value == "E" {
                end.x = x;
                end.y = y;
            };
        }
    }
    (start, end)
}

fn next_points(map: &Vec<Vec<String>>, point: &Point, cost: usize) -> Vec<(Point, usize)> {
    let mut neighbours: Vec<(Point, usize)> = Vec::new();
    let ops = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (direction, op) in ops.iter().enumerate() {
        let next_y = point.y as isize + op.0;
        let next_x = point.x as isize + op.1;
        let mut next_cost = cost + 1;
        let direction_diff = point.direction.abs_diff(direction);
        if direction_diff == 2 {
            next_cost += 2000;
        } else if direction_diff == 3 || direction_diff == 1 {
            next_cost += 1000;
        }
        if map[next_y as usize][next_x as usize] != "#" {
            neighbours.push((
                Point {
                    y: next_y as usize,
                    x: next_x as usize,
                    direction: direction,
                },
                next_cost,
            ))
        }
    }
    neighbours
}

fn find_shortest(map: &Vec<Vec<String>>, start: Point, end: Point) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut distances: HashMap<Point, usize> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value != "#" {
                for i in 0..4 {
                    distances.insert(
                        Point {
                            x: x,
                            y: y,
                            direction: i,
                        },
                        usize::MAX,
                    );
                }
            }
        }
    }

    *distances.get_mut(&start).unwrap() = 0;
    let mut queue = Vec::new();
    let start = start.clone();
    queue.push((start.clone(), 0));

    while !queue.is_empty() {
        queue.sort_by(|a, b| a.1.cmp(&b.1));
        queue.reverse();
        let (point, cost) = queue.pop().unwrap();

        if point.x == end.x && point.y == end.y {
            return cost;
        }

        if visited.contains(&point) {
            continue;
        }
        visited.insert(point.clone());

        for (next_point, next_cost) in next_points(map, &point, cost) {
            let stored_cost = distances.get_mut(&next_point).unwrap();
            if *stored_cost > next_cost {
                *stored_cost = next_cost;
                queue.push((next_point, next_cost));
            }
        }
    }

    0
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let maze = parse_input(input);
    let (start, end) = find_start_end(&maze);
    find_shortest(&maze, start, end)
}

fn find_shortest_paths(map: &Vec<Vec<String>>, start: Point, end: Point) -> String {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut distances: HashMap<Point, usize> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value != "#" {
                for i in 0..4 {
                    distances.insert(
                        Point {
                            x: x,
                            y: y,
                            direction: i,
                        },
                        usize::MAX,
                    );
                }
            }
        }
    }

    *distances.get_mut(&start).unwrap() = 0;
    let mut queue: Vec<(Point, usize, String)> = Vec::new();
    let start = start.clone();
    queue.push((start.clone(), 0, serialise(&start)));

    while !queue.is_empty() {
        queue.sort_by(|a, b| a.1.cmp(&b.1));
        queue.reverse();
        let (point, cost, mut path) = queue.pop().unwrap();
        for (i, p) in queue.clone().iter().enumerate() {
            if p.0 == point && p.1 == cost {
                let (_, _, another_path) = queue.remove(i);
                path += &another_path;
            }
        }
        if point.x == end.x && point.y == end.y {
            return path;
        }

        if visited.contains(&point) {
            continue;
        }
        visited.insert(point.clone());

        for (next_point, next_cost) in next_points(map, &point, cost) {
            let stored_cost = distances.get_mut(&next_point).unwrap();
            if *stored_cost >= next_cost {
                *stored_cost = next_cost;
                queue.push((
                    next_point.clone(),
                    next_cost,
                    path.clone() + &serialise(&next_point),
                ));
            }
        }
    }
    String::new()
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let maze = parse_input(input);
    let (start, end) = find_start_end(&maze);
    let a = find_shortest_paths(&maze, start, end);
    let mut unique: HashSet<String> = HashSet::new();
    for i in a.split(",") {
        if !i.is_empty() {
            unique.insert(i.to_string());
        }
    }

    unique.len()
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
    fn check_part_a_example_a() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(7036, solve_part_a(&example));
    }
    #[test]
    fn check_part_a_example_b() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(11048, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example_a() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(45, solve_part_b(&example));
    }
    #[test]
    fn check_part_b_example_b() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(64, solve_part_b(&example));
    }
}
