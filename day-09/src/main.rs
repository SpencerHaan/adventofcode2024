use std::fs;

// const INPUT_PATH: &str = "./data/day_09_test_input.txt";
const INPUT_PATH: &str = "./data/day_09_puzzle_input.txt";

const EMPTY_PLACEHOLDER: i32 = -1;

fn main() {
    let disk_map = load_disk_map();
    let compressed_disk_map = compress_disk_map(disk_map);
    let checksum = calculate_checksum(&compressed_disk_map);
    println!("checksum is: {checksum}");
}

fn load_disk_map() -> Vec<i32> {
    let mut disk_map: Vec<i32> = Vec::new();

    let mut is_file = true;
    let mut offset: usize = 0;
    let mut file_id_counter: i32 = 0;
    match fs::read_to_string(INPUT_PATH) {
        Ok(data) => {
            for d in data.chars().filter_map(|c| c.to_digit(10)) {
                if is_file {
                    disk_map.resize(offset + d as usize, file_id_counter);
                    file_id_counter += 1;
                } else {
                    disk_map.resize(offset + d as usize, -1);
                }
                offset += d as usize;
                is_file = !is_file;
            }
        },
        Err(e) => panic!("{e:?}"),
    }
    return disk_map;
}

// fn print_disk_map(disk_map: &Vec<i32>) {
//     for i in disk_map {
//         if *i == EMPTY_PLACEHOLDER {
//             print!(".");
//         } else {
//             print!("{i}");
//         }
//     }
//     println!();
// }

fn compress_disk_map(disk_map: Vec<i32>) -> Vec<i32> {
    let mut compressed_disk_map = disk_map;
    let left_index = 0;
    let right_index = compressed_disk_map.len() - 1;
    swap_left(&mut compressed_disk_map, left_index, right_index);
    return compressed_disk_map;
}

fn swap_left(disk_map: &mut Vec<i32>, left_index: usize, right_index: usize) {
    if left_index == right_index {
        return;
    }

    let left = disk_map[left_index];
    if left != EMPTY_PLACEHOLDER {
        return swap_left(disk_map, left_index + 1, right_index);
    }

    let right = disk_map[right_index];
    if right == EMPTY_PLACEHOLDER {
        return swap_left(disk_map, left_index, right_index - 1);
    }

    disk_map[left_index] = right;
    disk_map[right_index] = left;
    return swap_left(disk_map, left_index + 1, right_index - 1);
}

fn calculate_checksum(disk_map: &Vec<i32>) -> u64 {
    let mut checksum: u64 = 0;
    for (i, d) in disk_map.iter().enumerate() {
        if *d == EMPTY_PLACEHOLDER {
            continue;
        }
        checksum += *d as u64 * i as u64;
    }
    return checksum;
}
