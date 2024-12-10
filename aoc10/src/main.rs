use std::{collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<usize>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn find_trailheads(map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            if *num == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

fn find_trail_a(
    map: &Vec<Vec<usize>>,
    current_point: (usize, usize),
    peaks: &mut HashSet<(usize, usize)>,
) {
    let num = map[current_point.0][current_point.1];
    if num == 9 {
        peaks.insert(current_point);
    }
    let ops = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    for op in ops {
        let next_point_x = current_point.0 as i64 + op.0;
        let next_point_y = current_point.1 as i64 + op.1;
        if next_point_x >= 0
            && next_point_x < map.len() as i64
            && next_point_y >= 0
            && next_point_y < map[0].len() as i64
        {
            if num + 1 == map[next_point_x as usize][next_point_y as usize] {
                find_trail_a(map, (next_point_x as usize, next_point_y as usize), peaks);
            }
        }
    }
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let map = parse_input(input);

    let trailheads = find_trailheads(&map);
    let mut sum = 0;
    for point in trailheads {
        let mut peaks: HashSet<(usize, usize)> = HashSet::new();
        find_trail_a(&map, point, &mut peaks);
        sum += peaks.len();
    }
    sum
}

fn find_trail_b(
    map: &Vec<Vec<usize>>,
    current_point: (usize, usize),
    path: Vec<(usize, usize)>,
    peaks: &mut HashSet<Vec<(usize, usize)>>,
) {
    let num = map[current_point.0][current_point.1];
    if num == 9 {
        peaks.insert(path.clone());
    }
    let ops = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    for op in ops {
        let next_point_x = current_point.0 as i64 + op.0;
        let next_point_y = current_point.1 as i64 + op.1;
        if next_point_x >= 0
            && next_point_x < map.len() as i64
            && next_point_y >= 0
            && next_point_y < map[0].len() as i64
        {
            if num + 1 == map[next_point_x as usize][next_point_y as usize] {
                let mut next_path = path.clone();
                next_path.push((next_point_x as usize, next_point_y as usize));
                find_trail_b(
                    map,
                    (next_point_x as usize, next_point_y as usize),
                    next_path,
                    peaks,
                );
            }
        }
    }
}
fn solve_part_b(input: &Vec<String>) -> usize {
    let map = parse_input(input);

    let trailheads = find_trailheads(&map);
    let mut sum = 0;
    for point in trailheads {
        let mut peaks: HashSet<Vec<(usize, usize)>> = HashSet::new();
        find_trail_b(&map, point, vec![], &mut peaks);
        sum += peaks.len();
    }
    sum
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
        assert_eq!(36, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(81, solve_part_b(&example));
    }
}
