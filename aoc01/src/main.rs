use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let mut left_list = vec![];
    let mut right_list = vec![];

    input.iter().for_each(|f| {
        let temp: Vec<&str> = f.split(" ").collect();
        left_list.push(temp[0].parse::<i32>().unwrap());
        right_list.push(temp[3].parse::<i32>().unwrap());
    });
    left_list.sort();
    right_list.sort();

    let mut difference = 0;

    left_list.iter().enumerate().for_each(|(num, item)| {
        difference += (item - right_list.get(num).unwrap()).abs();
    });
    difference
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let mut left_list = vec![];
    let mut right_list = vec![];

    input.iter().for_each(|f| {
        let temp: Vec<&str> = f.split(" ").collect();
        left_list.push(temp[0].parse::<i32>().unwrap());
        right_list.push(temp[3].parse::<i32>().unwrap());
    });

    let mut similarity = 0;

    left_list.iter().for_each(|item| {
        similarity += item * right_list.iter().filter(|num| *num == item).count() as i32
    });
    similarity as i32
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
        assert_eq!(11, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(31, solve_part_b(&example));
    }
}
