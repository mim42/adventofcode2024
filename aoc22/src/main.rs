use std::fs::read_to_string;

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
        num = num ^ num << 6 & 0xFFFFFF;
        num = num ^ num >> 5 & 0xFFFFFF;
        num = num ^ num << 11 & 0xFFFFFF;
    }
    num
}

fn all_secret_number_generator(mut num: usize, iterations: usize) -> [usize; 2001] {
    let mut all_secret_numbers: [usize; 2001] = [0; 2001];
    all_secret_numbers[0] = num % 10;
    for i in 1..iterations + 1 {
        num = num ^ num << 6 & 0xFFFFFF;
        num = num ^ num >> 5 & 0xFFFFFF;
        num = num ^ num << 11 & 0xFFFFFF;
        all_secret_numbers[i] = num % 10;
    }
    all_secret_numbers
}

fn solve_part_a(secret_numbers: &Vec<usize>) -> usize {
    secret_numbers
        .iter()
        .map(|num| secret_number_generator(*num, 2000))
        .sum()
}

fn solve_part_b(secret_numbers: &Vec<usize>) -> usize {
    let mut bananas: [usize; 130321] = [0; 130321];
    let mut max = 0;
    for num in secret_numbers {
        let mut first_time: [bool; 130321] = [true; 130321];
        let all_secret_numbers = all_secret_number_generator(*num, 2000);
        for window in all_secret_numbers.windows(5) {
            let p1 = (9 + window[1] as i64 - window[0] as i64) as usize;
            let p2 = (9 + window[2] as i64 - window[1] as i64) as usize;
            let p3 = (9 + window[3] as i64 - window[2] as i64) as usize;
            let p4 = (9 + window[4] as i64 - window[3] as i64) as usize;
            let index = p1 * 6859 + p2 * 361 + p3 * 19 + p4;

            if first_time[index] {
                bananas[index] += window[4];
                if max < bananas[index] {
                    max = bananas[index];
                }
                first_time[index] = false;
            }
        }
    }
    max
}

fn main() {
    let input: Vec<String> = read_lines("./inputs/input.txt");
    let secret_numbers = parse_input(&input);
    let result_part_a = solve_part_a(&secret_numbers);
    println!("result of part a {}", result_part_a);
    let result_part_b = solve_part_b(&secret_numbers);
    println!("result of part b {}", result_part_b);
}
