use std::collections::{HashMap, HashSet};

use input;
use plane::*;

// const INPUT_PATH: &str = "./data/day_08_test_input.txt";
const INPUT_PATH: &str = "./data/day_08_puzzle_input.txt";

fn main() {
    let (limit, frequencies) = load_frequencies();
    // println!("{frequencies:?}");

    let antinodes = find_antinodes(&limit, &frequencies);
    // println!("{antinodes:?}");
    println!();

    print_map(&limit, &frequencies, &antinodes);
    println!("unique antinodes: {}", antinodes.len());
}

fn load_frequencies() -> (Rect, HashMap<char, Vec<Point>>) {
    let mut width: usize = 0;
    let mut height: usize = 0;

    let mut frequencies = HashMap::new();
    input::lines_indexed(INPUT_PATH, |y, line| {
        width = line.len();
        height = y + 1;

        for (x, frequency) in line.chars().enumerate() {
            if frequency.is_alphanumeric() {
                frequencies.entry(frequency)
                    .or_insert(Vec::new())
                    .push(Point { x, y });
            }
        }
    });
    return (
        Rect { width, height },
        frequencies
    );
}

fn find_antinodes(limit: &Rect, frequencies: &HashMap<char, Vec<Point>>) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, antennas) in frequencies {
        for t in 0..antennas.len() {
            let target = antennas.get(t).unwrap();
            for s in (t + 1)..antennas.len() {
                let subject = antennas.get(s).unwrap();
                let offset = target.offset_from(subject);

                match offset.apply(target) {
                    Some(antinode) => {
                        if limit.contains(&antinode) {
                            antinodes.insert(antinode);
                        }
                    }
                    None => ()
                }
                match offset.inverse().apply(subject) {
                    Some(antinode) => {
                        if limit.contains(&antinode) {
                            antinodes.insert(antinode);
                        }
                    },
                    None => ()
                }
            }
        }
    }
    return antinodes;
}

fn print_map(limit: &Rect, frequencies: &HashMap<char, Vec<Point>>, antinodes: &HashSet<Point>) {
    let mut antenna_frequencies: HashMap<Point, char> = HashMap::new();
    for (frequency, antennas) in frequencies {
        for antenna in antennas {
            antenna_frequencies.insert(*antenna, *frequency);
        }
    }

    for y in 0..limit.height {
        for x in 0..limit.width {
            let point = Point { x, y };
            if antenna_frequencies.contains_key(&point) {
                print!("{}", antenna_frequencies[&point]);
            } else
            if antinodes.contains(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
