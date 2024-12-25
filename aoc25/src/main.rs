use fxhash::{FxHashMap, FxHashSet};
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut pins: Vec<Vec<i32>> = Vec::new();
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let input = input
        .iter()
        .map(|c| {
            c.chars()
                .map(|s| if s == '#' { 1 } else { 0 })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut index = 0;
    loop {
        if index >= input.len() {
            break;
        }

        let mut a = vec![0, 0, 0, 0, 0];
        let temp_input = &input[index..index + 7];
        if temp_input[0].iter().all(|f| *f == 1) {
            for i in &temp_input[1..] {
                a[0] += i[0];
                a[1] += i[1];
                a[2] += i[2];
                a[3] += i[3];
                a[4] += i[4];
            }
            keys.push(a);
        } else {
            for i in &temp_input[..temp_input.len() - 1] {
                a[0] += i[0];
                a[1] += i[1];
                a[2] += i[2];
                a[3] += i[3];
                a[4] += i[4];
            }
            pins.push(a);
        }

        index += 8;
    }

    (pins, keys)
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let (pins, keys) = parse_input(input);
    let mut counter = 0;
    for key in keys {
        for mut pin in pins.clone() {
            if key.iter().zip(pin.iter_mut()).all(|(a, b)| *a + *b < 6) {
                counter += 1;
            }
        }
    }

    counter
}

fn solve_part_b(input: &Vec<String>) -> String {
    todo!()
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
        assert_eq!(7, solve_part_a(&example));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!("co,de,ka,ta", solve_part_b(&example));
    }
}
