use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(
    input: &Vec<String>,
) -> (
    HashMap<String, bool>,
    HashMap<String, (String, String, String)>,
) {
    let mut parsed_input = HashMap::default();
    let mut initial_values = HashMap::default();
    let mut flag = true;

    for i in input {
        if i.is_empty() {
            flag = false;
            continue;
        }
        if flag {
            let split = i
                .split(": ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
            let mut value = false;
            if split[1] == "1" {
                value = true;
            }
            initial_values.insert(split[0].clone(), value);
        } else {
            let split = i
                .split(" -> ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
            let node = split[1].to_string();
            let gates = split[0]
                .split(" ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>();

            parsed_input.insert(
                node.clone(),
                (gates[0].clone(), gates[1].clone(), gates[2].clone()),
            );
            parsed_input.insert(node, (gates[2].clone(), gates[1].clone(), gates[0].clone()));
        }
    }
    (initial_values, parsed_input)
}

fn parse_input_b(input: &Vec<String>) -> HashMap<(String, String, String), String> {
    let mut parsed_input = HashMap::default();

    let mut flag = true;

    for i in input {
        if i.is_empty() {
            flag = false;
            continue;
        }
        if !flag {
            let split = i
                .split(" -> ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
            let node = split[1].to_string();
            let gates = split[0]
                .split(" ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>();

            parsed_input.insert(
                (gates[0].clone(), gates[1].clone(), gates[2].clone()),
                node.clone(),
            );
            parsed_input.insert((gates[2].clone(), gates[1].clone(), gates[0].clone()), node);
        }
    }
    parsed_input
}

fn find_value(
    graph: &HashMap<String, (String, String, String)>,
    initial_nodes: &mut HashMap<String, bool>,
    node: &String,
) -> bool {
    match initial_nodes.get(node) {
        Some(v) => *v,
        None => {
            let (a, op, b) = graph.get(node).unwrap();
            let value;
            if op == "OR" {
                value = find_value(graph, initial_nodes, a) || find_value(graph, initial_nodes, b);
            } else if op == "AND" {
                value = find_value(graph, initial_nodes, a) && find_value(graph, initial_nodes, b);
            } else {
                value = find_value(graph, initial_nodes, a) ^ find_value(graph, initial_nodes, b);
            }
            // initial_nodes.insert(node.clone(), value);
            value
        }
    }
}

fn is_full_adder(
    graph: &HashMap<(String, String, String), String>,
    node_a: &String,
    node_b: &String,
    c_in: &String,
    sum: &String,
) -> (String, Vec<String>) {
    let mut problematic_nodes = Vec::new();
    let mut graph_iter = graph.iter().filter(|((next_a, op, next_b), xor_node)| {
        next_a == node_a && next_b == node_b && op == "XOR"
    });
    let mut c_in_and_xor = String::new();
    let mut c_out: String = String::new();
    match graph_iter.next() {
        Some(((_, op, _), xor_node)) => {
            let mut graph_iter = graph.iter().filter(|((next_a, op, next_b), _)| {
                next_a == c_in && next_b == xor_node && op == "XOR"
            });
            match graph_iter.next() {
                Some(((_, op, _), result)) => {
                    if result != sum {
                        println!("problem at sum {} with c_in {}", sum, c_in);
                        problematic_nodes.push(sum.clone());
                    }
                }
                None => {
                    println!("problem at result xor {} {}", c_in, xor_node);
                    problematic_nodes.push(c_in.clone());
                    problematic_nodes.push(xor_node.clone());
                    return (c_out, problematic_nodes);
                }
            }

            let mut graph_iter = graph.iter().filter(|((next_a, op, next_b), _)| {
                next_a == c_in && next_b == xor_node && op == "AND"
            });
            match graph_iter.next() {
                Some(((_, op, _), c_in_and_xor_node)) => {
                    c_in_and_xor = c_in_and_xor_node.clone();
                }
                None => {
                    println!("problem at result c_in_and_xor_node {} {}", c_in, xor_node);
                    problematic_nodes.push(c_in.clone());
                    problematic_nodes.push(xor_node.clone());
                    return (c_out, problematic_nodes);
                }
            }
        }
        None => (),
    }

    let mut graph_iter = graph
        .iter()
        .filter(|((next_a, op, next_b), _)| next_a == node_a && next_b == node_b && op == "AND");

    match graph_iter.next() {
        Some(((_, op, _), a_and_b_node)) => {
            let mut graph_iter = graph.iter().filter(|((next_a, op, next_b), _)| {
                next_a == a_and_b_node && next_b == &c_in_and_xor && op == "OR"
            });
            match graph_iter.next() {
                Some(((_, op, _), c_in_and_node)) => {
                    c_out = c_in_and_node.clone();
                }
                None => {
                    println!("problem at result and_or_node {}", a_and_b_node);
                    problematic_nodes.push(c_in_and_xor.clone());
                    problematic_nodes.push(a_and_b_node.clone());
                    return (c_out, problematic_nodes);
                }
            }
        }
        None => (),
    }

    (c_out, problematic_nodes)
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let (mut initial_values, graph) = parse_input(input);
    let mut z = Vec::from_iter(graph.iter().filter(|(k, _)| k.starts_with("z")));
    z.sort();
    let answer = z
        .iter()
        .map(|(node, _)| find_value(&graph, &mut initial_values, node))
        .collect::<Vec<bool>>();
    let mut final_answer = 0;
    for (index, value) in answer.iter().enumerate() {
        final_answer += (*value as usize) << index;
    }
    final_answer
}

fn solve_part_b(input: &Vec<String>) -> String {
    let (initial_values, graph) = parse_input(input);
    let temp_graph = graph.clone();
    let mut z = Vec::from_iter(temp_graph.iter().filter(|(k, _)| k.starts_with("z")));
    let mut x = Vec::from_iter(initial_values.iter().filter(|(k, _)| k.starts_with("x")));
    let mut y = Vec::from_iter(initial_values.iter().filter(|(k, _)| k.starts_with("y")));
    z.sort();
    x.sort();
    y.sort();

    let mut graph = parse_input_b(input);
    let mut current_c_in = "tss".to_string();
    let mut index = 1;

    loop {
        if index > 44 {
            break;
        }
        let xx = x[index].0;
        let yy = y[index].0;
        let zz = z[index].0;
        println!("{}", index);
        let (next_c_in, problematic_nodes) = is_full_adder(&mut graph, xx, yy, &current_c_in, zz);
        if problematic_nodes.len() == 0 {
            current_c_in = next_c_in;
            index += 1;
        } else {
            break;
        }
    }

    String::new()
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
        assert_eq!(2024, solve_part_a(&example));
    }
    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!("co,de,ka,ta", solve_part_b(&example));
    }
}
