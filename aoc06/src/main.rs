use std::{collections::HashSet, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    let parsed_input = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    parsed_input
}

fn find_guard(matrix: &Vec<Vec<String>>) -> (usize, usize) {
    let mut position = (0, 0);
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == "^" {
                position.0 = i;
                position.1 = j;
            }
        }
    }
    position
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let mut matrix = parse_input(input);
    let start_position = find_guard(&matrix);
    matrix[start_position.0 as usize][start_position.1 as usize] = ".".to_string();
    let mut all_positions: HashSet<(usize, usize)> = HashSet::new();

    all_positions.insert(start_position);
    let mut direction = 0;
    let operations = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_pos = start_position;
    loop {
        let op = operations[direction % 4];
        let next_position_x = current_pos.0 as i32 + op.0;
        let next_position_y = current_pos.1 as i32 + op.1;

        if next_position_x < 0
            || next_position_x >= matrix.len() as i32
            || next_position_y < 0
            || next_position_y >= matrix.len() as i32
        {
            break;
        }
        let next_position_x = next_position_x as usize;
        let next_position_y = next_position_y as usize;
        if matrix[next_position_x][next_position_y] == "#" {
            direction += 1;
        } else {
            current_pos = (next_position_x, next_position_y);
            all_positions.insert(current_pos);
        }
    }

    all_positions.len() as i32
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let mut matrix = parse_input(input);
    let start_position = find_guard(&matrix);

    let mut counter = 0;
    matrix[start_position.0][start_position.1] = ".".to_string();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if i == start_position.0 && j == start_position.1 {
                continue;
            }
            if matrix[i][j] == "#" {
                continue;
            }
            matrix[i][j] = "#".to_string();
            let mut all_positions: HashSet<((usize, usize), usize)> = HashSet::new();
            all_positions.insert((start_position, 0));
            let mut direction = 0;
            let operations = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            let mut current_pos = start_position;
            loop {
                direction = direction % 4;
                let op = operations[direction];
                let next_position_x = current_pos.0 as i32 + op.0;
                let next_position_y = current_pos.1 as i32 + op.1;

                if next_position_x < 0
                    || next_position_x >= matrix.len() as i32
                    || next_position_y < 0
                    || next_position_y >= matrix.len() as i32
                {
                    break;
                }
                let next_position_x = next_position_x as usize;
                let next_position_y = next_position_y as usize;
                if matrix[next_position_x][next_position_y] == "#" {
                    direction += 1;
                } else {
                    current_pos = (next_position_x, next_position_y);

                    if all_positions.contains(&(current_pos, direction)) {
                        counter += 1;
                        break;
                    }
                    all_positions.insert((current_pos, direction));
                }
            }
            matrix[i][j] = ".".to_string();
        }
    }

    counter
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let result_part_a: i32 = solve_part_a(&input);
    println!("result of part a {}", result_part_a);
    let result_part_b: i32 = solve_part_b(&input);
    println!("result of part b {}", result_part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_part_a_example() {
        let example: Vec<String> = read_lines("./inputs/example-a.txt");
        assert_eq!(41, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(6, solve_part_b(&example));
    }
}
