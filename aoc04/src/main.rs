use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn solve_part_a(input: &Vec<String>) -> i32 {
    let xmas = input
        .iter()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut counter = 0;
    for (i, line) in xmas.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            let possible_xmas = generate_a(i, j, &xmas);
            possible_xmas.iter().for_each(|word| {
                if word.join("") == "XMAS" {
                    {
                        counter += 1;
                    }
                }
            });
        }
    }
    counter
}
fn generate_a<'a>(i: usize, j: usize, xmas: &Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let operations = [
        (1, 1),
        (1, 0),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut generated_lines: Vec<Vec<&'a str>> = Vec::new();
    for (x, y) in operations {
        let mut temp_x = i;
        let mut temp_y = j;
        let mut line: Vec<&'a str> = Vec::new();
        for _ in 0..4 {
            match xmas.get(temp_x) {
                Some(row) => match row.get(temp_y) {
                    Some(letter) => {
                        line.push(*letter);
                    }
                    None => (),
                },
                None => (),
            }

            temp_x = (temp_x as i32 + x) as usize;
            temp_y = (temp_y as i32 + y) as usize;
        }
        generated_lines.push(line);
    }
    generated_lines
}

fn solve_part_b(input: &Vec<String>) -> i32 {
    let xmas = input
        .iter()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut counter = 0;
    for (i, line) in xmas.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if xmas[i][j] == "A" {
                let ops = [(1, 1, -1, -1), (-1, 1, 1, -1)];
                if ops
                    .iter()
                    .map(|op| {
                        let prev = match xmas.get((i as i32 + op.0) as usize) {
                            Some(row) => match row.get((j as i32 + op.1) as usize) {
                                Some(letter) => letter,
                                None => "",
                            },
                            None => "",
                        };
                        let next = match xmas.get((i as i32 + op.2) as usize) {
                            Some(row) => match row.get((j as i32 + op.3) as usize) {
                                Some(letter) => letter,
                                None => "",
                            },
                            None => "",
                        };
                        prev.to_owned() + "A" + next
                    })
                    .all(|mas| mas == "MAS" || mas == "SAM")
                {
                    counter += 1;
                }
            }
        }
    }
    counter
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
        assert_eq!(18, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(9, solve_part_b(&example));
    }
}
