use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<usize> {
    input[0]
        .split(" ")
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn iterate_steps(initial_stones: &Vec<usize>, steps: usize) -> usize {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    for i in initial_stones {
        stones.insert(*i, 1);
    }
    for _ in 0..steps {
        let mut temp_stones: HashMap<usize, usize> = HashMap::new();
        for (key, value) in stones {
            let str_num = key.to_string();
            if key == 0 {
                *temp_stones.entry(1).or_insert(0) += value;
            } else if str_num.len() % 2 == 0 {
                let left_num = str_num[..str_num.len() / 2].to_string();
                let right_num = str_num[str_num.len() / 2..].to_string();
                let mut right_num = right_num.trim_start_matches('0');
                if right_num.is_empty() {
                    right_num = "0";
                }
                *temp_stones
                    .entry(left_num.parse::<usize>().unwrap())
                    .or_insert(0) += value;
                *temp_stones
                    .entry(right_num.parse::<usize>().unwrap())
                    .or_insert(0) += value;
            } else {
                *temp_stones.entry(key * 2024).or_insert(0) += value;
            }
        }
        stones = temp_stones;
    }
    stones.iter().map(|(_, counter)| counter).sum()
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let initial_stones = parse_input(input);
    iterate_steps(&initial_stones, 25)
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let initial_stones = parse_input(input);

    iterate_steps(&initial_stones, 75)
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
        assert_eq!(55312, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(65601038650482, solve_part_b(&example));
    }
}
