use std::{collections::VecDeque, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<(u64, VecDeque<u64>)> {
    let mut parsed_input: Vec<(u64, VecDeque<u64>)> = Vec::new();
    for line in input {
        let splited: Vec<&str> = line.split(": ").collect();
        let result = splited[0].parse::<u64>().unwrap();
        let nums = splited[1]
            .split(" ")
            .map(|c| c.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        parsed_input.push((result, nums));
    }
    parsed_input
}

fn check_valid_a(equation: u64, nums: VecDeque<u64>) -> bool {
    let operations = ["+", "*"];
    if nums.len() == 1 {
        return equation == nums[0];
    }
    operations.iter().any(|op| {
        let mut temp_nums = nums.clone();
        let first = temp_nums.pop_front().unwrap();
        let second = temp_nums.pop_front().unwrap();
        let mut result = 0;

        if op == &"+" {
            result = first + second;
        } else {
            result = first * second;
        }
        temp_nums.insert(0, result);
        check_valid_a(equation, temp_nums)
    })
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let equations: Vec<(u64, VecDeque<u64>)> = parse_input(input);
    let mut result = 0;
    for (equation, nums) in equations {
        if check_valid_a(equation, nums) {
            result += equation;
        }
    }
    result
}

fn check_valid_b(equation: u64, nums: VecDeque<u64>) -> bool {
    let operations = ["+", "*", "||"];
    if nums.len() == 1 {
        return equation == nums[0];
    }
    operations.iter().any(|op| {
        let mut temp_nums = nums.clone();
        let first = temp_nums.pop_front().unwrap();
        let second = temp_nums.pop_front().unwrap();
        let mut result = 0;

        if op == &"+" {
            result = first + second;
        } else if op == &"*" {
            result = first * second;
        } else {
            result = (first.to_string() + &second.to_string())
                .parse::<u64>()
                .unwrap();
        }
        temp_nums.insert(0, result);
        check_valid_b(equation, temp_nums)
    })
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let equations: Vec<(u64, VecDeque<u64>)> = parse_input(input);
    let mut result = 0;
    for (equation, nums) in equations {
        if check_valid_b(equation, nums) {
            result += equation;
        }
    }
    result
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
        assert_eq!(3749, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(11387, solve_part_b(&example));
    }
}
