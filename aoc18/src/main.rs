use std::cmp::Reverse;
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
    limit: usize,
) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    let ops = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for op in ops {
        let next_y = point.0 as isize + op.0;
        let next_x = point.1 as isize + op.1;
        if next_y >= 0 && next_x >= 0 && next_y < limit as isize && next_x < limit as isize {
            if !map.contains(&(next_y as usize, next_x as usize)) {
                neighbours.push((next_y as usize, next_x as usize))
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
    let mut q = std::collections::BinaryHeap::new();
    let mut processed = vec![false; limit * limit];
    let mut distance = vec![usize::MAX; limit * limit];
    let start = start.0 * limit + start.1;
    let end = end.0 * limit + end.1;
    distance[start] = 0;
    q.push((Reverse(0), start));

    while !q.is_empty() {
        let a = q.pop().unwrap().1;
        if processed[a] {
            continue;
        }
        if a == end {
            return distance[end];
        }
        processed[a] = true;
        for (next_point_y, next_point_x) in next_points(map, (a / limit, a % limit), limit) {
            let next_index = next_point_y * limit + next_point_x;
            if (distance[a] + 1) < distance[next_index] {
                distance[next_index] = distance[a] + 1;
                q.push((Reverse(distance[next_index]), next_index));
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
    let mut mid = map.len() / 2;
    let mut offset = map.len() / 4;
    loop {
        let cost = find_shortest(&map[..mid].to_vec(), (0, 0), (limit - 1, limit - 1), limit);
        if cost > 0 {
            if offset == 0 {
                offset = 1;
            }
            mid = mid + offset;
            offset = offset / 2;
        } else if cost == 0 {
            if offset == 0 {
                break;
            }
            mid = mid - offset;
            offset = offset / 2;
        }
    }

    return map[mid - 1].0.to_string() + "," + &map[mid - 1].1.to_string();
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
