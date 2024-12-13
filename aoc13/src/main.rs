use regex::Regex;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>, offset: usize) -> Vec<Vec<usize>> {
    let mut parsed_input = Vec::new();
    let mut counter = 0;
    loop {
        let mut temp_line: Vec<usize> = Vec::new();

        if counter >= input.len() {
            break;
        }
        let re_x_y = Regex::new(r"X\+(?<x>[\d]+), Y\+(?<y>[\d]+)").unwrap();
        let re_result = Regex::new(r"X=(?<x>[\d]+), Y=(?<y>[\d]+)").unwrap();
        let button_a = &input[counter];
        let button_b = &input[counter + 1];
        let prize = &input[counter + 2];

        for button in [button_a, button_b] {
            let capture = re_x_y.captures(&button).unwrap();
            let button_x = capture
                .name("x")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let button_y = capture
                .name("y")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            temp_line.push(button_x);
            temp_line.push(button_y);
        }
        let capture = re_result.captures(&prize).unwrap();
        let prize_x = capture
            .name("x")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let prize_y = capture
            .name("y")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        temp_line.push(offset + prize_x);
        temp_line.push(offset + prize_y);

        counter += 4;
        parsed_input.push(temp_line);
    }
    parsed_input
}

fn solve_system_equation(equations: &Vec<usize>, limit: bool) -> usize {
    let a1 = equations[0] as i64;
    let a2 = equations[1] as i64;
    let b1 = equations[2] as i64;
    let b2 = equations[3] as i64;
    let result_x = equations[4] as i64;
    let result_y = equations[5] as i64;
    let x = ((b2 * result_x) - (b1 * result_y)) / ((a1 * b2) - (a2 * b1));
    let y = ((a1 * result_y) - (a2 * result_x)) / ((a1 * b2) - (a2 * b1));

    if x >= 0 && y >= 0 && (!limit || (x <= 100 && y <= 100)) {
        if ((x * a1) + (y * b1)) == result_x && ((x * a2) + (y * b2) == result_y) {
            x as usize * 3 + y as usize
        } else {
            0
        }
    } else {
        0
    }
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let parsed_input = parse_input(input, 0);
    parsed_input
        .iter()
        .map(|machine| solve_system_equation(machine, true))
        .sum::<usize>()
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let parsed_input = parse_input(input, 10000000000000);
    parsed_input
        .iter()
        .map(|machine| solve_system_equation(machine, false))
        .sum::<usize>()
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
        assert_eq!(480, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(1206, solve_part_b(&example));
    }
}
