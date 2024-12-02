use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let lines: Vec<Vec<i32>> = input
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    lines
        .iter()
        .filter(|line| {
            line.windows(2)
                .all(|window| (1..4).contains(&(window[0] - window[1])))
                || line
                    .windows(2)
                    .all(|window| (1..4).contains(&(window[1] - window[0])))
        })
        .count() as i32
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let lines: Vec<Vec<i32>> = input
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    lines
        .iter()
        .filter(|line| {
            (0..line.len()).enumerate().any(|(n, _)| {
                line.iter()
                    .enumerate()
                    .filter(|(i, _)| (*i != n))
                    .map(|(_, v)| v)
                    .collect::<Vec<&i32>>()
                    .windows(2)
                    .all(|window| (1..4).contains(&(window[0] - window[1])))
                    || line
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| (*i != n))
                        .map(|(_, v)| v)
                        .collect::<Vec<&i32>>()
                        .windows(2)
                        .all(|window| (1..4).contains(&(window[1] - window[0])))
            })
        })
        .count() as i32
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
        assert_eq!(2, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(4, solve_part_b(&example));
    }
}
