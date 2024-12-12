use std::collections::HashSet;
use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut map: Vec<Vec<String>> = Vec::new();
    map.push(vec![".".to_string(); input[0].len() + 4]);
    map.push(vec![".".to_string(); input[0].len() + 4]);

    for line in input {
        let mut temp_line = Vec::new();
        temp_line.push(".".to_string());
        temp_line.push(".".to_string());
        for c in line.chars() {
            temp_line.push(c.to_string());
        }
        temp_line.push(".".to_string());
        temp_line.push(".".to_string());
        map.push(temp_line);
    }
    map.push(vec![".".to_string(); input[0].len() + 4]);
    map.push(vec![".".to_string(); input[0].len() + 4]);
    map
}

fn next_plants(i: usize, j: usize, plant: &String, map: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    let mut next_plants = Vec::new();
    let ops = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    for op in ops {
        let next_i = (i as i64 + op.0) as usize;
        let next_j = (j as i64 + op.1) as usize;
        if map[next_i][next_j] == *plant {
            next_plants.push((next_i, next_j));
        }
    }
    next_plants
}

fn next_different_plants(
    i: usize,
    j: usize,
    plant: &String,
    map: &Vec<Vec<String>>,
) -> Vec<(usize, usize)> {
    let mut next_plants = Vec::new();
    let ops = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    for op in ops {
        let next_i = (i as i64 + op.0) as usize;
        let next_j = (j as i64 + op.1) as usize;
        if map[next_i][next_j] != *plant {
            next_plants.push((next_i, next_j));
        }
    }
    next_plants
}

fn diagonal_different_plants(
    i: usize,
    j: usize,
    plant: &String,
    map: &Vec<Vec<String>>,
) -> Vec<(usize, usize)> {
    let mut next_plants = Vec::new();
    let ops = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for op in ops {
        let next_i = (i as i64 + op.0) as usize;
        let next_j = (j as i64 + op.1) as usize;
        if map[next_i][next_j] != *plant {
            next_plants.push((next_i, next_j));
        }
    }
    next_plants
}

fn recursive_fill_region(
    i: usize,
    j: usize,
    plant: &String,
    map: &Vec<Vec<String>>,
    region: &mut HashSet<(usize, usize)>,
) {
    if !region.contains(&(i, j)) {
        region.insert((i, j));

        for (next_i, next_j) in next_plants(i, j, plant, map) {
            recursive_fill_region(next_i, next_j, plant, map, region);
        }
    }
}

fn populate_regions(
    i: usize,
    j: usize,
    map: &Vec<Vec<String>>,
    regions: &mut Vec<HashSet<(usize, usize)>>,
) {
    let plant = &map[i][j];
    if plant != "." {
        if !regions.iter().any(|region| region.contains(&(i, j))) {
            let mut region = HashSet::new();
            recursive_fill_region(i, j, plant, map, &mut region);
            regions.push(region);
        }
    }
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let map: Vec<Vec<String>> = parse_input(input);
    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            populate_regions(i, j, &map, &mut regions);
        }
    }
    regions
        .iter()
        .map(|region| {
            region.len()
                * region
                    .iter()
                    .map(|(i, j)| 4 - next_plants(*i, *j, &map[*i][*j], &map).len())
                    .sum::<usize>()
        })
        .sum::<usize>()
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let map: Vec<Vec<String>> = parse_input(input);
    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            populate_regions(i, j, &map, &mut regions);
        }
    }
    regions
        .iter()
        .map(|region| {
            region.len()
                * (region
                    .iter()
                    .map(|(i, j)| {
                        let plant = &map[*i][*j];
                        let neighbors = next_plants(*i, *j, plant, &map);
                        let mut score = 0;
                        // Alone plant has 4 sides
                        if neighbors.len() == 0 {
                            score = 4;
                        // Plant with 1 neighbor counts are 2 vertices in the perimeter of the shape
                        } else if neighbors.len() == 1 {
                            score = 2;
                        // if plan has two neibours can have two configurations
                        // O O O     O X O
                        // X[X]X  or O[X]X
                        // O O O     O O O
                        // If its the second it counts are a single vertex
                        } else if neighbors.len() == 2 {
                            if !((neighbors[0].0 as i64 - neighbors[1].0 as i64).abs() == 2
                                || (neighbors[0].1 as i64 - neighbors[1].1 as i64).abs() == 2)
                            {
                                score += 1;
                            }
                        }

                        // For all other cases we check if a plant is a vertex
                        // X X O     X X O    O X O
                        // X[X]X  or X[X]X or X[X]X
                        // X O O     X X O    O X O
                        // The first example is a single vertex , the second counts are two vertices and the third counts as four

                        for diagonal in diagonal_different_plants(*i, *j, plant, &map) {
                            // for all diagonals of plant we check if they are themselves neighboring exactly 2 neighbors of the plant
                            let diagonal_plant = &map[diagonal.0][diagonal.1];
                            let mut counter = 0;
                            for next_diagonal in
                                next_different_plants(diagonal.0, diagonal.1, diagonal_plant, &map)
                            {
                                if neighbors.contains(&next_diagonal) {
                                    counter += 1;
                                }
                            }
                            if counter == 2 {
                                score += 1;
                            }
                        }

                        score
                    })
                    .sum::<usize>())
        })
        .sum::<usize>()
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
        assert_eq!(1930, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(1206, solve_part_b(&example));
    }
}
