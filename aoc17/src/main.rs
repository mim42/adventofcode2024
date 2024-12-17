use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> (usize, usize, usize, Vec<usize>) {
    let register_a: usize = input[0].split(": ").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    let register_b: usize = input[1].split(": ").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    let register_c: usize = input[2].split(": ").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    let program: Vec<usize> = input[4].split(": ").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (register_a, register_b, register_c, program)
}

fn run_program(
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: &Vec<usize>,
) -> Vec<usize> {
    let mut a = register_a;
    let mut b = register_b;
    let mut c = register_c;
    let mut output: Vec<usize> = Vec::new();
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        let combo_operand = [0, 1, 2, 3, a, b, c, 7][operand];
        match opcode {
            0 => {
                a = a / 2_usize.pow(combo_operand as u32) as usize;
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                b = combo_operand % 8;
            }
            3 => {
                if a != 0 {
                    instruction_pointer = operand;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                output.push(combo_operand % 8);
            }
            6 => {
                b = a / 2_usize.pow(combo_operand as u32) as usize;
            }
            7 => {
                c = a / 2_usize.pow(combo_operand as u32) as usize;
            }
            _ => (),
        }
        instruction_pointer += 2;
    }
    output
}

fn solve_part_a(input: &Vec<String>) -> String {
    let (a, b, c, program) = parse_input(input);
    let output = run_program(a, b, c, &program);

    output
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let (_, b, c, program) = parse_input(input);
    let mut potential = Vec::new();
    potential.push(0);
    let mut answer = 0;

    while !potential.is_empty() {
        let guess = potential.pop().unwrap();
        for i in 0..8 {
            let new_guess = guess + i;
            let output = run_program(new_guess, b, c, &program);
            if program[program.len() - output.len()..] == output {
                if output.len() == program.len() {
                    answer = new_guess;
                    break;
                }
                potential.push((new_guess) * 8);
            }
        }
    }

    let mut index = 0;
    loop {
        let output = run_program(answer - index, b, c, &program);
        if output != program {
            return answer - index + 1;
        }
        index += 1;
    }
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
        assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(117440, solve_part_b(&example));
    }
}
