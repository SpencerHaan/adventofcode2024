use std::{collections::HashMap, fs, ops::Div};

// const RULES_PATH: &str = "./data/test_input_rules.txt";
// const UPDATES_PATH: &str = "./data/test_input_updates.txt";

const RULES_PATH: &str = "./data/puzzle_input_rules.txt";
const UPDATES_PATH: &str = "./data/puzzle_input_updates.txt";

fn main() {
    let rules = load_rules_map();
    // println!("{rules:?}");
    match fs::read_to_string(UPDATES_PATH) {
        Ok(updates) => {
            let mut correct_updates: Vec<Vec<i32>> = Vec::new();
            for update in updates.split_terminator("\n") {
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

                if is_correct_order {
                    correct_updates.push(pages);
                }
            }

            let mut middle_page_sum = 0;
            for correct_update in correct_updates {
                middle_page_sum += get_middle_page(&correct_update);
                // println!("{correct_update:?}");
            }
            println!("middle page sum: {middle_page_sum}");
        },
        Err(e) => {
            println!("failed to load updates: {e:?}");
        }
    }
}

fn load_rules_map() -> HashMap<i32, Vec<i32>> {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    match fs::read_to_string(RULES_PATH) {
        Ok(data) => {
            for line in data.split_terminator("\n").into_iter() {
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
            }
            return rules;
        },
        Err(e) => {
            println!("failed to load rules: {e:?}");
        },
    }
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

fn get_middle_page(pages: &Vec<i32>) -> i32 {
    if pages.len() % 2 == 0 {
        panic!("even update, no middle page!");
    }
    let index = pages.len().div(2);
    return pages[index];
}
