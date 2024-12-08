use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<(u64, Vec<u64>)> {
    let mut parsed_input: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input {
        let splited: Vec<&str> = line.split(": ").collect();
        let result = splited[0].parse::<u64>().unwrap();
        let nums = splited[1]
            .split(" ")
            .map(|c| c.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        parsed_input.push((result, nums));
    }
    parsed_input
}

fn check_valid_a(equation: u64, nums: &[u64], result: u64) -> bool {
    let operations = ["+", "*"];
    if nums.is_empty() || result > equation {
        return equation == result;
    }
    operations.iter().any(|op| {
        let first = nums[0];
        let mut next_result = 0;
        if op == &"+" {
            next_result = first + result;
        } else {
            next_result = first * result;
        }
        check_valid_a(equation, &nums[1..], next_result)
    })
}

fn solve_part_a(input: &Vec<String>) -> u64 {
    let equations: Vec<(u64, Vec<u64>)> = parse_input(input);
    let mut result = 0;
    for (equation, nums) in equations {
        if check_valid_a(equation, &nums, 0) {
            result += equation;
        }
    }
    result
}

fn check_valid_b(equation: u64, nums: &[u64], result: u64) -> bool {
    let operations = ["+", "*", "||"];
    if nums.is_empty() || result > equation {
        return equation == result;
    }
    operations.iter().any(|op| {
        let first = nums[0];
        let mut next_result = 0;

        if op == &"+" {
            next_result = first + result;
        } else if op == &"*" {
            next_result = first * result;
        } else {
            next_result = (result.to_string() + &first.to_string())
                .parse::<u64>()
                .unwrap();
        }
        check_valid_b(equation, &nums[1..], next_result)
    })
}

fn solve_part_b(input: &Vec<String>) -> u64 {
    let equations: Vec<(u64, Vec<u64>)> = parse_input(input);
    let mut result = 0;
    for (equation, nums) in equations {
        if check_valid_b(equation, &nums, 0) {
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
