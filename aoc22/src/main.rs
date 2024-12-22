use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<usize> {
    input
        .iter()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn secret_number_generator(mut num: usize, iterations: usize) -> usize {
    for _ in 0..iterations {
        num = (num ^ (num * 64)) % 16777216;
        num = (num ^ (num / 32)) % 16777216;
        num = (num ^ (num * 2048)) % 16777216;
    }
    num
}

fn all_secret_number_generator(mut num: usize, iterations: usize) -> Vec<usize> {
    let mut all_secret_numbers = vec![num % 10];
    for _ in 0..iterations {
        num = (num ^ (num * 64)) % 16777216;
        num = (num ^ (num / 32)) % 16777216;
        num = (num ^ (num * 2048)) % 16777216;
        all_secret_numbers.push(num % 10);
    }
    all_secret_numbers
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let secret_numbers = parse_input(input);
    secret_numbers
        .iter()
        .map(|num| secret_number_generator(*num, 2000))
        .sum()
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let mut bananas: HashMap<[i64; 4], usize> = HashMap::new();
    for num in parse_input(input) {
        let mut first_time: HashSet<[i64; 4]> = HashSet::new();
        let all_secret_numbers = all_secret_number_generator(num, 2000);
        for window in all_secret_numbers.windows(5) {
            let p1 = window[1] as i64 - window[0] as i64;
            let p2 = window[2] as i64 - window[1] as i64;
            let p3 = window[3] as i64 - window[2] as i64;
            let p4 = window[4] as i64 - window[3] as i64;
            if !first_time.contains(&[p1, p2, p3, p4]) {
                bananas
                    .entry([p1, p2, p3, p4])
                    .and_modify(|counter| *counter += window[4])
                    .or_insert(window[4]);
                first_time.insert([p1, p2, p3, p4]);
            }
        }
    }
    let (_, max_bananas) = bananas.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    max_bananas
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
        assert_eq!(37327623, solve_part_a(&example));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(23, solve_part_b(&example));
    }
}
