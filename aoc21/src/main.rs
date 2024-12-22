use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    usize,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn next_buttons_keypad(keypad: &Vec<Vec<String>>, button: String) -> Vec<(String, String)> {
    let mut button_y = 0;
    let mut button_x = 0;
    for (y, row) in keypad.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if button == *value {
                button_y = y;
                button_x = x;
            }
        }
    }
    let mut next_buttons = Vec::new();
    let ops = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let remote_instructions = ["v", "^", ">", "<"];
    for (index, op) in ops.iter().enumerate() {
        let next_button_y = button_y as isize + op.0;
        let next_button_x = button_x as isize + op.1;
        if next_button_y >= 0
            && next_button_y < keypad.len() as isize
            && next_button_x >= 0
            && next_button_x < keypad[0].len() as isize
            && !(next_button_y == 3 && next_button_x == 0)
        {
            next_buttons.push((
                keypad[next_button_y as usize][next_button_x as usize].clone(),
                remote_instructions[index].to_string(),
            ));
        }
    }

    next_buttons
}

fn find_all_keypad_paths(
    keypad: &Vec<Vec<String>>,
    button_a: String,
    button_b: String,
) -> Vec<String> {
    let mut q = Vec::new();
    let mut processed: HashSet<String> = HashSet::new();
    let mut distance: HashMap<String, usize> = HashMap::new();
    distance.insert(button_a.clone(), 0);
    q.push((button_a, 0, "".to_string()));
    while !q.is_empty() {
        q.sort_by(|a, b| a.1.cmp(&b.1));
        q.reverse();
        let (a, cost, path) = q.pop().unwrap();
        if a == button_b {
            q.push((a, cost, path));
            break;
        }
        processed.insert(a.clone());
        for (next_button, remote_instruction) in next_buttons_keypad(keypad, a) {
            if !distance.contains_key(&next_button) {
                distance.insert(next_button.clone(), cost + 1);
                q.push((next_button, cost + 1, path.clone() + &remote_instruction));
            } else {
                let old_cost = distance.get_mut(&next_button).unwrap();
                if cost + 1 <= *old_cost {
                    *old_cost = cost + 1;
                    q.push((next_button, cost + 1, path.clone() + &remote_instruction));
                }
            }
        }
    }

    q.iter()
        .filter(|(button, _, _)| *button == button_b)
        .map(|(_, _, instruction)| instruction.clone() + "A")
        .collect::<Vec<String>>()
}

fn next_buttons_remote(remote: &Vec<Vec<String>>, button: String) -> Vec<(String, String)> {
    let mut button_y = 0;
    let mut button_x = 0;
    for (y, row) in remote.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if button == *value {
                button_y = y;
                button_x = x;
            }
        }
    }

    let mut next_buttons = Vec::new();
    let ops = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let remote_instructions = ["v", "^", ">", "<"];
    for (index, op) in ops.iter().enumerate() {
        let next_button_y = button_y as isize + op.0;
        let next_button_x = button_x as isize + op.1;
        if next_button_y >= 0
            && next_button_y < remote.len() as isize
            && next_button_x >= 0
            && next_button_x < remote[0].len() as isize
            && !(next_button_y == 0 && next_button_x == 0)
        {
            next_buttons.push((
                remote[next_button_y as usize][next_button_x as usize].clone(),
                remote_instructions[index].to_string(),
            ));
        }
    }
    next_buttons
}

fn find_all_remote_paths(
    remote: &Vec<Vec<String>>,
    button_a: String,
    button_b: String,
) -> Vec<String> {
    if button_a == button_b {
        return vec!["A".to_string()];
    }
    let mut q = Vec::new();
    let mut distance: HashMap<String, usize> = HashMap::new();
    distance.insert(button_a.clone(), 0);
    q.push((button_a, 0, "".to_string()));
    while !q.is_empty() {
        q.sort_by(|a, b| a.1.cmp(&b.1));
        q.reverse();
        let (a, cost, path) = q.pop().unwrap();
        if a == button_b {
            q.push((a, cost, path));
            break;
        }
        for (next_button, remote_instruction) in next_buttons_remote(remote, a) {
            if !distance.contains_key(&next_button) {
                distance.insert(next_button.clone(), cost + 1);
                q.push((next_button, cost + 1, path.clone() + &remote_instruction));
            } else {
                let old_cost = distance.get_mut(&next_button).unwrap();
                if cost + 1 <= *old_cost {
                    *old_cost = cost + 1;
                    q.push((next_button, cost + 1, path.clone() + &remote_instruction));
                }
            }
        }
    }

    q.iter()
        .filter(|(button, _, _)| *button == button_b)
        .map(|(_, _, instruction)| instruction.clone() + "A")
        .collect::<Vec<String>>()
}

fn remote_on_keypad(keypad: &Vec<Vec<String>>, code: String) -> Vec<String> {
    let mut remote_commands = vec!["".to_string()];
    let mut pointin_button = "A".to_string();
    for button in code.chars() {
        let mut temp = Vec::new();
        for path in find_all_keypad_paths(keypad, pointin_button.to_string(), button.to_string()) {
            for command in &remote_commands {
                temp.push(command.clone() + &path);
            }
        }
        remote_commands = temp;
        pointin_button = button.to_string();
    }
    remote_commands
}

// fn heuristic(a: &String) -> usize {
//     let mut counter = 0;
//     let mut prev_char = '_';
//     for i in a.chars() {
//         if i == prev_char {
//             counter += 1;
//         } else {
//             prev_char = i;
//         }
//     }
//     counter + 100 - a.len() * 3 as usize
// }

fn remote_on_remote(
    remote: &Vec<Vec<String>>,
    cache: &mut HashMap<(String, usize), usize>,
    memory: &mut HashMap<String, Vec<String>>,
    remote_counter: HashMap<String, usize>,
    depth: usize,
) -> usize {
    if depth == 0 {
        return remote_counter
            .iter()
            .map(|(code, counter)| code.len() * counter)
            .sum();
    }
    let mut true_length = 0;
    for (code, counter) in remote_counter {
        let mut possible_codes = Vec::new();
        let mut min_length = usize::MAX;
        if !cache.contains_key(&(code.clone(), depth)) {
            let mut remote_commands = vec!["".to_string()];
            let mut pointin_button = "A".to_string();

            for button in code.chars() {
                let mut temp = Vec::new();
                for path in
                    find_all_remote_paths(remote, pointin_button.to_string(), button.to_string())
                {
                    for command in &remote_commands {
                        temp.push(command.clone() + &path);
                    }
                }
                remote_commands = temp;
                pointin_button = button.to_string();
            }

            possible_codes = remote_commands.clone();

            for possible_code in possible_codes {
                let mut commands_counter: HashMap<String, usize> = HashMap::new();
                let splited = possible_code
                    .split_inclusive("A")
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>();
                for sub_codes in splited {
                    commands_counter
                        .entry(sub_codes)
                        .and_modify(|c| *c += counter)
                        .or_insert(counter);
                }
                let length =
                    remote_on_remote(remote, cache, memory, commands_counter.clone(), depth - 1);
                if length <= min_length {
                    min_length = length;
                }
            }
            cache.insert((code.clone(), depth), min_length / counter);
        } else {
            min_length = *cache.get(&(code, depth)).unwrap() * counter;
            println!("works")
        }

        true_length += min_length
    }

    true_length
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let keypad = vec![
        vec!["7", "8", "9"],
        vec!["4", "5", "6"],
        vec!["1", "2", "3"],
        vec!["", "0", "A"],
    ]
    .iter()
    .map(|row| row.iter().map(|c| c.to_string()).collect::<Vec<String>>())
    .collect::<Vec<Vec<String>>>();

    let remote = vec![vec!["_", "^", "A"], vec!["<", "v", ">"]]
        .iter()
        .map(|row| row.iter().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut answer = 0;
    for code in input {
        let first_codes = remote_on_keypad(&keypad, code.clone());

        let final_code = first_codes
            .iter()
            .map(|code| {
                let mut cache: HashMap<(String, usize), usize> = HashMap::new();
                let mut commands_counter: HashMap<String, usize> = HashMap::new();
                let mut memory: HashMap<String, Vec<String>> = HashMap::new();
                let splited = code
                    .split_inclusive("A")
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>();

                for sub_codes in splited {
                    commands_counter
                        .entry(sub_codes)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
                remote_on_remote(&remote, &mut cache, &mut memory, commands_counter, 25)
            })
            .min_by(|a, b| a.cmp(&b))
            .unwrap();

        answer += code
            .trim_start_matches('0')
            .trim_end_matches('A')
            .parse::<usize>()
            .unwrap()
            * final_code;
    }
    answer
}

fn solve_part_b(input: &Vec<String>) -> usize {
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
        assert_eq!(126384, solve_part_a(&example));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(0, solve_part_b(&example));
    }
}
