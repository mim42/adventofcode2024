use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<(usize, usize)> {
    let mut parsed_map = Vec::new();

    for i in input {
        let a = i
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        parsed_map.push((a[0], a[1]));
    }

    parsed_map
}

fn next_points(
    map: &Vec<(usize, usize)>,
    point: (usize, usize),
    cost: usize,
    limit: usize,
) -> Vec<(usize, usize, usize)> {
    let mut neighbours: Vec<(usize, usize, usize)> = Vec::new();
    let ops = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for op in ops {
        let next_y = point.0 as isize + op.0;
        let next_x = point.1 as isize + op.1;
        let next_cost = cost + 1;
        if next_y >= 0 && next_x >= 0 && next_y < limit as isize && next_x < limit as isize {
            if !map.contains(&(next_y as usize, next_x as usize)) {
                neighbours.push((next_y as usize, next_x as usize, next_cost))
            }
        }
    }
    neighbours
}

fn find_shortest(
    map: &Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
    limit: usize,
) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    distances.insert(start, 0);
    let mut queue = Vec::new();
    let start = start.clone();
    queue.push((start.0, start.1, 0));

    while !queue.is_empty() {
        queue.sort_by(|a, b| a.2.cmp(&b.2));
        queue.reverse();
        let (point_y, point_x, cost) = queue.pop().unwrap();

        if point_y == end.0 && point_x == end.1 {
            return cost;
        }

        if visited.contains(&(point_y, point_x)) {
            continue;
        }
        visited.insert((point_y, point_x));

        for (next_point_y, next_point_x, next_cost) in
            next_points(map, (point_y, point_x), cost, limit)
        {
            match distances.get_mut(&(next_point_y, next_point_x)) {
                Some(stored_cost) => {
                    if *stored_cost > next_cost {
                        *stored_cost = next_cost;
                        queue.push((next_point_y, next_point_x, next_cost));
                    }
                }
                None => {
                    distances.insert((next_point_y, next_point_x), next_cost);
                    queue.push((next_point_y, next_point_x, next_cost));
                }
            }
        }
    }

    0
}

fn solve_part_a(input: &Vec<String>, limit: usize, bytes: usize) -> usize {
    let map = parse_input(input);
    let cost = find_shortest(
        &map[..bytes].to_vec(),
        (0, 0),
        (limit - 1, limit - 1),
        limit,
    );

    cost
}

fn solve_part_b(input: &Vec<String>, limit: usize) -> String {
    let map = parse_input(input);

    for i in 0..map.len() - 1 {
        let cost = find_shortest(&map[..i].to_vec(), (0, 0), (limit - 1, limit - 1), limit);
        if cost == 0 {
            return map[i - 1].0.to_string() + "," + &map[i - 1].1.to_string();
        }
    }

    String::new()
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a = solve_part_a(&input, 71, 1024);
    println!("result of part a {}", result_part_a);
    let result_part_b = solve_part_b(&input, 71);
    println!("result of part b {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(22, solve_part_a(&example, 7, 12));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!("6,1", solve_part_b(&example, 7));
    }
}
