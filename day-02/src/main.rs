use std::ops::RangeInclusive;
use input;

// const INPUT_PATH: &str = "./data/day_02_test_input.txt";
const INPUT_PATH: &str = "./data/day_02_puzzle_input.txt";

const SAFE_RANGE: RangeInclusive<i32> = 1..=3;

fn main() {
    let mut safe_count = 0;
    input::lines(INPUT_PATH, |report| {
        if report.is_empty() {
            return;
        }

        let levels: Vec<i32> = report.split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect();

        let mut safe = true;
        let bad_level_indexes = get_bad_level_indexes(&levels);
        if !bad_level_indexes.is_empty() {
            safe = false;

            for bad_level_index in bad_level_indexes {
                let mut new_levels = levels.clone();
                new_levels.remove(bad_level_index);

                if get_bad_level_indexes(&new_levels).is_empty() {
                    safe = true;
                    break;
                }
            }
        }

        if safe {
            safe_count += 1;
        }

        if !safe {
            println!("{}: {:?}", if safe { "SAFE" } else { "UNSAFE" }, levels);
        }
    });
    println!("safe count: {}", safe_count);
}

fn get_bad_level_indexes(levels: &Vec<i32>) -> Vec<usize> {
    let mut bad_levels: Vec<usize> = Vec::new();
    let mut increase: Option<bool> = None;
    for current in 0..levels.len() - 1 {
        let next = current + 1;

        let delta = levels[next] - levels[current];
        if !SAFE_RANGE.contains(&delta.abs()) {
            bad_levels = (current..=next).collect();
            break;
        }

        if increase.is_none() {
            increase = Some(delta.is_positive())
        } else if increase.unwrap() != delta.is_positive() {
            let previous = current - 1;
            bad_levels = (previous..=next).collect();
            break;
        }
    }
    return bad_levels;
}
