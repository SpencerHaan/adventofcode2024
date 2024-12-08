use std::{collections::HashMap, ops::Div};
use input;

// const RULES_PATH: &str = "./data/day_05_test_input_rules.txt";
// const UPDATES_PATH: &str = "./data/day_05_test_input_updates.txt";

const RULES_PATH: &str = "./data/day_05_puzzle_input_rules.txt";
const UPDATES_PATH: &str = "./data/day_05_puzzle_input_updates.txt";

fn main() {
    let rules = load_rules_map();
    // println!("{rules:?}");

    let mut correct_updates: Vec<Vec<i32>> = Vec::new();
    let mut incorrect_updates: Vec<Vec<i32>> = Vec::new();
    input::lines(UPDATES_PATH, |update| {
        let mut is_correct_order = true;

        let pages: Vec<i32> = update.split(",")
            .filter_map(|p| p.parse::<i32>().ok())
            .collect();
        for (i, page) in pages.clone().into_iter().enumerate() {
            match rules.get(&page) {
                None => continue,
                Some(page_rules) => {
                    if !correct_order(&pages[..i], page_rules) {
                        is_correct_order = false;
                        break;
                    }
                }
            }
        }

        // println!("{:?}: {is_correct_order}", pages.clone());
        if is_correct_order {
            correct_updates.push(pages);
        } else {
            incorrect_updates.push(pages);
        }
    });

    let mut middle_page_sum = 0;
    for correct_update in correct_updates {
        middle_page_sum += get_middle_page(&correct_update);
        // println!("{correct_update:?}");
    }
    println!("middle page sum: {middle_page_sum}");

    let mut corrected_middle_page_sum = 0;
    for incorrect_update in incorrect_updates {
        let corrected_update = correct_update(&incorrect_update, &rules);
        corrected_middle_page_sum += get_middle_page(&corrected_update);
    }
    println!("corrected page sum: {corrected_middle_page_sum}");
}

fn load_rules_map() -> HashMap<i32, Vec<i32>> {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    input::lines(RULES_PATH, |line| {
        match line.split_once("|") {
            None => panic!("invalid rule: {line}"),
            Some((x, y)) => {
                let key = x.parse::<i32>().unwrap();
                let value = y.parse::<i32>().unwrap();
                if !rules.contains_key(&key) {
                    rules.insert(key, Vec::new());
                }
                rules.get_mut(&key).unwrap().push(value);
            }
        }
    });
    return rules;
}

fn correct_order(pages: &[i32], rules: &Vec<i32>) -> bool {
    for page in pages {
        if rules.contains(page) {
            return false;
        }
    }
    return true;
}

fn correct_update(pages: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut ordered_pages: Vec<i32> = Vec::new();
    for page in pages {
        if ordered_pages.is_empty() {
            ordered_pages.push(*page);
        } else {
            match rules.get(page) {
                None => ordered_pages.push(*page),
                Some(page_rules) => {
                    let mut insert_i: Option<usize> = None;
                    for (i, ordered_page) in ordered_pages.iter().enumerate() {
                        if page_rules.contains(ordered_page) {
                            insert_i = Some(i);
                            break;
                        }
                    }

                    match insert_i {
                        None => ordered_pages.push(*page),
                        Some(i) => ordered_pages.insert(i, *page)
                    }
                }
            }
        }
    }
    return ordered_pages;
}

fn get_middle_page(pages: &Vec<i32>) -> i32 {
    if pages.len() % 2 == 0 {
        panic!("even update, no middle page!");
    }
    let index = pages.len().div(2);
    return pages[index];
}
