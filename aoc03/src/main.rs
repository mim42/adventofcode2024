use regex::Regex;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let re = Regex::new(r"mul\((?<x>[\d]+),(?<y>[\d]+)\)").unwrap();
    let line = input.join("");
    re.captures_iter(&line)
        .map(|mul| {
            let x = mul.name("x").unwrap().as_str().parse::<i32>().unwrap();
            let y = mul.name("y").unwrap().as_str().parse::<i32>().unwrap();
            x * y
        })
        .sum::<i32>()
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let re = Regex::new(r"mul\((?<x>[\d]+),(?<y>[\d]+)\)").unwrap();
    let line = input.join("");

    let mut dos: Vec<&str> = line.split("do()").collect::<Vec<&str>>();
    dos.iter_mut().for_each(|s| match s.find("don't()") {
        Some(pos) => *s = &s[..pos],
        None => (),
    });
    dos.iter()
        .map(|seg| {
            re.captures_iter(seg)
                .map(|mul| {
                    let x = mul.name("x").unwrap().as_str().parse::<i32>().unwrap();
                    let y = mul.name("y").unwrap().as_str().parse::<i32>().unwrap();
                    x * y
                })
                .sum::<i32>()
        })
        .sum::<i32>()
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
        assert_eq!(161, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(48, solve_part_b(&example));
    }
}
