use std::collections::HashMap;
use input;

// const INPUT_PATH: &str = "./data/day_01_test_input.txt";
const INPUT_PATH: &str = "./data/day_01_puzzle_input.txt";

fn main() {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    input::lines(INPUT_PATH, |pair: &str| {
        let split: Vec<&str> = pair.split_whitespace().collect();
        if split.is_empty() {
            return;
        }

        left_list.push(split[0].parse::<i32>().unwrap());
        right_list.push(split[1].parse::<i32>().unwrap());
    });

    let total_distance = distance(left_list.clone(), right_list.clone());
    println!("total distance: {}", total_distance);

    let total_similarity = similarity(left_list, right_list);
    println!("total similarity: {}", total_similarity);
}

fn distance(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    for (i, left) in left_list.iter().enumerate() {
        let right = right_list.get(i).unwrap();
        total_distance += (left - right).abs();
    }
    return total_distance;
}

fn similarity(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for (_, right) in right_list.iter().enumerate() {
        let count = right_counts.entry(*right).or_insert(0);
        *count += 1;
    }

    let mut total_similarity = 0;
    for left in left_list.iter() {
        total_similarity += left * right_counts.get(left).unwrap_or(&0);
    }
    return total_similarity;
}
