use fxhash::FxHashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let towels = input[0]
        .split(", ")
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    let mut designs = Vec::new();

    for i in &input[2..] {
        designs.push(i.to_string());
    }
    (towels, designs)
}

fn permutations(
    towels: &Vec<String>,
    design: String,
    cache: &mut FxHashMap<String, usize>,
) -> usize {
    let mut counter = 0;
    if cache.contains_key(&design) {
        return *cache.get(&design).unwrap();
    }
    for j in towels.iter() {
        if design.len() >= j.len() && design[..j.len()] == *j {
            let sub_design = design[j.len()..].to_string();
            if design == *j {
                counter += 1;
            } else {
                counter += permutations(towels, sub_design, cache);
            }
        }
    }
    cache.insert(design, counter);
    counter
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let (towels, designs) = parse_input(input);
    let mut counter = 0;
    for i in designs {
        let mut dfs: Vec<String> = Vec::new();
        let towels_filtered = towels
            .clone()
            .into_iter()
            .filter(|c| i.contains(c))
            .collect::<Vec<String>>();
        dfs.push(i);
        while !dfs.is_empty() {
            let sub_design = dfs.pop().unwrap();
            if sub_design.is_empty() {
                counter += 1;
                break;
            }
            for j in towels_filtered.iter() {
                if sub_design.len() >= j.len() && sub_design[..j.len()] == *j {
                    dfs.push(sub_design[j.len()..].to_string())
                }
            }
        }
    }
    counter
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let (towels, designs) = parse_input(input);
    let mut counter = 0;
    for i in designs {
        let mut cache: FxHashMap<String, usize> = FxHashMap::default();
        let towels_filtered = towels
            .clone()
            .into_iter()
            .filter(|c| i.contains(c))
            .collect::<Vec<String>>();
        let permutations = permutations(&towels_filtered, i.clone(), &mut cache);
        counter += permutations;
    }
    counter
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
        assert_eq!(6, solve_part_a(&example));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(16, solve_part_b(&example));
    }
}
