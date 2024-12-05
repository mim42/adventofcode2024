use std::{fs::read_to_string, ops::Index};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut rules_flag = true;
    for line in input {
        if line.is_empty() {
            rules_flag = false;
            continue;
        }
        if rules_flag {
            let tuple: &[i32] = &line
                .split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            rules.push((tuple[0], tuple[1]));
        } else {
            let update: Vec<i32> = line
                .split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            updates.push(update);
        }
    }

    let mut counter = 0;
    for update in updates {
        if !rules.iter().any(|(first, second)| {
            let mut first_index = -1;
            let mut second_index = -1;

            for (i, num) in update.iter().enumerate() {
                if first == num {
                    first_index = i as i32;
                } else if second == num {
                    second_index = i as i32;
                }
            }
            first_index > second_index && first_index != -1 && second_index != -1
        }) {
            counter += update[update.len() / 2];
        }
    }

    counter
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    todo!()
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
        assert_eq!(143, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(9, solve_part_b(&example));
    }
}
