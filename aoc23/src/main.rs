use fxhash::{FxHashMap, FxHashSet};
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> FxHashMap<String, FxHashSet<String>> {
    let mut parsed_input = FxHashMap::default();
    for i in input {
        let nodes = i.split("-").collect::<Vec<&str>>();
        let n1 = nodes[0].to_string();
        let n2 = nodes[1].to_string();
        parsed_input
            .entry(n1.clone())
            .and_modify(|c: &mut FxHashSet<String>| {
                c.insert(n2.clone());
            })
            .or_insert(FxHashSet::from_iter([n2.clone()]));
        parsed_input
            .entry(n2)
            .and_modify(|c: &mut FxHashSet<String>| {
                c.insert(n1.clone());
            })
            .or_insert(FxHashSet::from_iter([n1]));
    }
    parsed_input
}

fn bron_kerbosch(
    graph: &FxHashMap<String, FxHashSet<String>>,
    cliques: &mut Vec<FxHashSet<String>>,
    r: FxHashSet<String>,
    mut p: FxHashSet<String>,
    mut x: FxHashSet<String>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }
    if !p.is_empty() {
        p.clone().iter().for_each(|node| {
            let neighbours = graph.get(node).unwrap();
            let new_candidate = FxHashSet::from_iter([node.clone()]);
            bron_kerbosch(
                graph,
                cliques,
                r.union(&new_candidate).cloned().collect(),
                p.intersection(&neighbours).cloned().collect(),
                x.intersection(&neighbours).cloned().collect(),
            );
            p.remove(node);
            x.insert(node.clone());
        });
    }
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let graph = parse_input(input);
    let mut set: FxHashSet<Vec<String>> = FxHashSet::default();
    for (node, connected_nodes) in &graph {
        if node.starts_with('t') {
            for second_node in connected_nodes {
                for third_node in graph.get(second_node).unwrap() {
                    if connected_nodes.contains(third_node) {
                        let mut three_set =
                            vec![node.clone(), second_node.clone(), third_node.clone()];
                        three_set.sort();
                        set.insert(three_set);
                    }
                }
            }
        }
    }
    set.len()
}

fn solve_part_b(input: &Vec<String>) -> String {
    let graph = parse_input(input);
    let mut cliques: Vec<FxHashSet<String>> = Vec::new();
    bron_kerbosch(
        &graph,
        &mut cliques,
        FxHashSet::default(),
        FxHashSet::from_iter(graph.clone().into_keys()),
        FxHashSet::default(),
    );
    let max = cliques
        .into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();
    let mut max = Vec::from_iter(max.into_iter());
    max.sort();
    max.join(",")
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
