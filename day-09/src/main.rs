use std::fs;

// const INPUT_PATH: &str = "./data/day_09_test_input.txt";
const INPUT_PATH: &str = "./data/day_09_puzzle_input.txt";

const EMPTY_PLACEHOLDER: i32 = -1;

fn main() {
    let (mut disk_map, next_file_id) = load_disk_map();
    optimize_disk_map(&mut disk_map, next_file_id);
    print_disk_map(&disk_map);

    let checksum = calculate_checksum(&disk_map);
    println!("checksum is: {checksum}");
}

fn load_disk_map() -> (Vec<i32>, i32) {
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
    return (disk_map, file_id_counter);
}

fn print_disk_map(disk_map: &Vec<i32>) {
    for i in disk_map {
        if *i == EMPTY_PLACEHOLDER {
            print!(".");
        } else {
            print!("{i}");
        }
    }
    println!();
    println!();
}

fn optimize_disk_map(disk_map: &mut Vec<i32>, next_file_id: i32) {
    for file_id in (0..next_file_id).rev() {
        match find_swap_candidate(disk_map, file_id) {
            Some((file_offset, free_offset, size)) => {
                // println!("move {file_id} from {file_offset} to {free_offset}: size {size}");
                replace(disk_map, free_offset, size, file_id);
                replace(disk_map, file_offset, size, EMPTY_PLACEHOLDER);
            },
            None => continue
        }
    }
}

fn find_swap_candidate(disk_map: &Vec<i32>, file_id: i32) -> Option<(usize, usize, usize)> {
    let (file_offset, size) = find_file(disk_map, file_id)?;
    let free_offset = find_free_offset(disk_map, size)?;

    if free_offset < file_offset {
        return Some((file_offset, free_offset, size));
    }
    return None;
}

fn find_file(disk_map: &Vec<i32>, file_id: i32) -> Option<(usize, usize)> {
    let mut offset: Option<usize> = None;

    let mut blocks: usize = 0;
    for i in 0..disk_map.len() {
        let block = *disk_map.get(i).unwrap();
        if block != file_id && offset.is_some() {
            break;
        }

        if block == file_id {
            if offset.is_none() {
                offset = Some(i);
            }
            blocks += 1;
        }
    }
    return offset.map(|o| (o, blocks));
}

fn find_free_offset(disk_map: &Vec<i32>, size: usize) -> Option<usize> {
    let mut offset: Option<usize> = None;

    let mut blocks: usize = 0;
    for (i, block) in disk_map.iter().enumerate() {
        if *block != EMPTY_PLACEHOLDER {
            offset = None;
            blocks = 0;
        } else if offset.is_none() {
            offset = Some(i);
        }

        if offset.is_some() && blocks == size {
            // println!("found freespace at {offset:?} with size {blocks}");
            break;
        }
        blocks += 1;
    }
    return offset.filter(|_| blocks == size);
}

fn replace(disk_map: &mut Vec<i32>, offset: usize, size: usize, block: i32) {
    for i in offset..(offset + size) {
        disk_map[i] = block;
    }
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
