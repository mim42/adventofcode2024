use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

fn parse_input(input: &Vec<String>) -> (Vec<usize>, usize) {
    let mut counter = 0;
    let mut parsed_input = Vec::new();
    for i in input[0].chars() {
        let num = i.to_string().parse::<usize>().unwrap();
        counter += num as usize;
        parsed_input.push(num);
    }
    (parsed_input, counter)
}

fn checksum(disk: &Vec<usize>, free_spot: usize) -> usize {
    let mut checksum = 0;
    for (i, num) in disk
        .iter()
        .enumerate()
        .filter(|(_, num)| **num != free_spot)
    {
        checksum += i * num;
    }
    checksum
}

fn solve_part_a(input: &Vec<String>) -> usize {
    let (disk_map, capacity) = parse_input(input);
    let mut disk: Vec<usize> = Vec::with_capacity(capacity);
    let free_spot = capacity; //any nubmer works above capacity/2  (which is the max id)

    for (index, num) in disk_map.iter().enumerate() {
        for _ in 0..*num {
            if index % 2 == 0 {
                disk.push(index / 2);
            } else {
                disk.push(free_spot);
            }
        }
    }

    let mut start_index = 0;
    let mut end_index = disk.len() - 1;

    loop {
        if start_index == end_index {
            break;
        }
        if disk[start_index] != free_spot {
            start_index += 1;
            continue;
        }
        if disk[end_index] == free_spot {
            end_index -= 1;
            continue;
        }
        disk[start_index] = disk[end_index];
        disk[end_index] = free_spot;
    }

    checksum(&disk, free_spot)
}

fn solve_part_b(input: &Vec<String>) -> usize {
    let (disk_map, capacity) = parse_input(input);
    let mut disk: Vec<usize> = Vec::with_capacity(capacity);
    let free_spot = capacity as usize; //any nubmer works above capacity/2  (which is the max id)

    for (index, num) in disk_map.iter().enumerate() {
        for _ in 0..*num {
            if index % 2 == 0 {
                disk.push(index / 2);
            } else {
                disk.push(free_spot);
            }
        }
    }
    let lengths = disk_map
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(index, num)| (index / 2, num))
        .rev()
        .collect::<Vec<(usize, usize)>>();

    for (file_id, size) in lengths {
        //for every fild_id (right to left) we find the starting index in the disk
        let file_index = disk.iter().position(|&r| r == file_id).unwrap();
        let mut free_spot_counter = 0;
        // find free space enough for the size of file
        for i in 0..file_index {
            free_spot_counter += 1;
            if free_spot != disk[i] {
                free_spot_counter = 0;
            } else if free_spot_counter == size {
                // if enough space we fill it with the file_id
                for k in 0..free_spot_counter {
                    disk[i - k] = file_id;
                }
                // and fill older position with free space
                for k in file_index..file_index + size {
                    if disk[k] == file_id {
                        disk[k] = free_spot;
                    }
                }
                break;
            }
        }
    }

    checksum(&disk, free_spot)
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
        assert_eq!(1928, solve_part_a(&example));
    }

    #[test]
    fn check_part_b_example() {
        let example: Vec<String> = read_lines("./inputs/example-b.txt");
        assert_eq!(2858, solve_part_b(&example));
    }
}
