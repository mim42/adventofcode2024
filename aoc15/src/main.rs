use regex::Regex;
use std::{fs::read_to_string, thread::panicking};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input_a(input: &Vec<String>) -> (Vec<Vec<String>>, String) {
    let mut parsed_map = Vec::new();
    let mut parsed_instructions = String::new();

    for i in input {
        if i.contains("#") {
            parsed_map.push(i.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        } else if !i.is_empty() {
            parsed_instructions = parsed_instructions + i;
        }
    }

    (parsed_map, parsed_instructions)
}

fn parse_input_b(input: &Vec<String>) -> (Vec<Vec<String>>, String) {
    let mut parsed_map = Vec::new();
    let mut parsed_instructions = String::new();

    for i in input {
        if i.contains("#") {
            let temp = i
                .chars()
                .map(|c| {
                    let mut x = c.to_string();
                    if x == "#" {
                        x = "##".to_string();
                    } else if x == "O" {
                        x = "[]".to_string();
                    } else if x == "@" {
                        x = "@.".to_string();
                    } else {
                        x = "..".to_string();
                    }
                    x
                })
                .collect::<Vec<String>>()
                .join("");
            parsed_map.push(temp.chars().map(|c| c.to_string()).collect::<Vec<String>>());
        } else if !i.is_empty() {
            parsed_instructions = parsed_instructions + i;
        }
    }

    (parsed_map, parsed_instructions)
}

fn find_robot(map: &Vec<Vec<String>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value == "@" {
                return (y, x);
            }
        }
    }
    (0, 0)
}

fn move_in_map_a(map: &mut Vec<Vec<String>>, y: usize, x: usize, instruction: String) -> bool {
    let mut next_x = x;
    let mut next_y = y;
    if instruction == ">" {
        next_x += 1;
    } else if instruction == "<" {
        next_x -= 1;
    } else if instruction == "^" {
        next_y -= 1;
    } else if instruction == "v" {
        next_y += 1;
    }
    let next_value = &map[next_y][next_x];
    if next_value == "." {
        map[next_y][next_x] = map[y][x].clone();
        map[y][x] = ".".to_string();
        return true;
    } else if next_value == "O" {
        if move_in_map_a(map, next_y, next_x, instruction) {
            map[next_y][next_x] = map[y][x].clone();
            map[y][x] = ".".to_string();
            return true;
        }
    }

    false
}

fn move_in_map_b(map: &mut Vec<Vec<String>>, y: usize, x: usize, instruction: &String) -> bool {
    let temp_map = map.clone();
    let mut next_x = x;
    let mut next_y = y;
    if instruction == ">" {
        next_x += 1;
    } else if instruction == "<" {
        next_x -= 1;
    } else if instruction == "^" {
        next_y -= 1;
    } else if instruction == "v" {
        next_y += 1;
    }
    let next_value = &map[next_y][next_x];

    if next_value == "." {
        map[next_y][next_x] = map[y][x].clone();
        map[y][x] = ".".to_string();

        return true;
    } else if next_value == "[" {
        if move_in_map_b(map, next_y, next_x + 1, instruction)
            && move_in_map_b(map, next_y, next_x, instruction)
        {
            map[next_y][next_x] = map[y][x].clone();
            map[y][x] = ".".to_string();
            return true;
        }
    } else if next_value == "]" {
        if move_in_map_b(map, next_y, next_x - 1, instruction)
            && move_in_map_b(map, next_y, next_x, instruction)
        {
            map[next_y][next_x] = map[y][x].clone();
            map[y][x] = ".".to_string();
            return true;
        }
    }
    *map = temp_map;
    false
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let (mut map, instructions) = parse_input_a(input);

    for i in instructions.chars() {
        let (y, x) = find_robot(&map);
        move_in_map_a(&mut map, y, x, i.to_string());
    }
    let mut summation = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value == "O" {
                summation += (y * 100) + x;
            }
        }
    }

    summation
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let (mut map, instructions) = parse_input_b(input);

    for i in instructions.chars() {
        let (y, x) = find_robot(&map);
        move_in_map_b(&mut map, y, x, &i.to_string());
    }

    let mut summation = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value == "[" {
                summation += (y * 100) + x;
            }
        }
    }

    summation
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
        assert_eq!(10092, solve_part_a(&example));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(9021, solve_part_b(&example));
    }
}
