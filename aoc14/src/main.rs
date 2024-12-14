use regex::Regex;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<isize>> {
    let mut parsed_input = Vec::new();
    let re = Regex::new(r"p=(?<x>[\d]+),(?<y>[\d]+) v=(?<v_x>-?[\d]+),(?<v_y>-?[\d]+)").unwrap();
    for line in input {
        let capture = re.captures(line).unwrap();
        let x = capture
            .name("x")
            .unwrap()
            .as_str()
            .parse::<isize>()
            .unwrap();
        let y = capture
            .name("y")
            .unwrap()
            .as_str()
            .parse::<isize>()
            .unwrap();
        let v_x = capture
            .name("v_x")
            .unwrap()
            .as_str()
            .parse::<isize>()
            .unwrap();
        let v_y = capture
            .name("v_y")
            .unwrap()
            .as_str()
            .parse::<isize>()
            .unwrap();
        parsed_input.push(vec![x, y, v_x, v_y]);
    }

    parsed_input
}

fn wrap_around(x: isize, y: isize, x_limit: isize, y_limit: isize) -> (isize, isize) {
    let mut wrapped_x = x;
    let mut wrapped_y = y;
    if x >= x_limit {
        wrapped_x = x % x_limit;
    } else if x < 0 {
        wrapped_x = x_limit - x.abs();
    }
    if y >= y_limit {
        wrapped_y = y % y_limit;
    } else if y < 0 {
        wrapped_y = y_limit - y.abs();
    }

    (wrapped_x, wrapped_y)
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let mut parsed_input = parse_input(input);
    let x_limit: isize = 101;
    let y_limit: isize = 103;
    for _ in 0..100 {
        for robot in &mut parsed_input {
            let coords = wrap_around(robot[0] + robot[2], robot[1] + robot[3], x_limit, y_limit);
            robot[0] = coords.0;
            robot[1] = coords.1;
        }
    }
    let mut counter_q1 = 0;
    let mut counter_q2 = 0;
    let mut counter_q3 = 0;
    let mut counter_q4 = 0;
    for i in &parsed_input {
        let x = i[0];
        let y = i[1];
        if x < x_limit / 2 {
            if y < y_limit / 2 {
                counter_q1 += 1;
            } else if y > y_limit / 2 {
                counter_q3 += 1;
            }
        } else if x > x_limit / 2 {
            if y < y_limit / 2 {
                counter_q2 += 1;
            } else if y > y_limit / 2 {
                counter_q4 += 1;
            }
        }
    }
    counter_q1 * counter_q2 * counter_q3 * counter_q4
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let mut parsed_input = parse_input(input);
    let x_limit: isize = 101;
    let y_limit: isize = 103;

    for i in 1..10403 {
        for robot in &mut parsed_input {
            let coords = wrap_around(robot[0] + robot[2], robot[1] + robot[3], x_limit, y_limit);
            robot[0] = coords.0;
            robot[1] = coords.1;
        }

        let mut flag = false;
        for i in 0..x_limit {
            let mut counter = 0;
            for j in 0..y_limit {
                if parsed_input.iter().any(|c| c[0] == i && c[1] == j) {
                    counter += 1;
                }
            }
            if counter > 35 {
                flag = true;
            }
        }
        if flag {
            return i;
        }
    }

    0
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
        assert_eq!(12, solve_part_a(&example));
    }
}
