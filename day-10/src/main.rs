use std::collections::{HashMap, HashSet};

use plane::*;

// const INPUT_PATH: &str = "./data/day_10_test_input.txt";
const INPUT_PATH: &str = "./data/day_10_puzzle_input.txt";

const MIN_ELEVATION: u32 = 0;
const MAX_ELEVATION: u32 = 9;

fn main() {
    let (map, bounds, trailhead_candidates) = load_map();
    print_map(&map, &bounds);
    println!("possible trailhead candidates: {}", trailhead_candidates.len());

    let mut total_score = 0;
    let mut total_rating = 0;
    for trailhead in trailhead_candidates {
        let mut peaks_reached: HashSet<Point> = HashSet::new();
        let rating = find_trails(&map, bounds, trailhead, Direction::Up, MIN_ELEVATION, &mut peaks_reached);
        println!("{trailhead}: score {}, rating: {rating}", peaks_reached.len());

        total_score += peaks_reached.len();
        total_rating += rating;
    }
    println!("trailhead totals: score {total_score}, rating {total_rating}");
}

fn load_map() -> (HashMap<Point, u32>, Rect, Vec<Point>) {
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut map: HashMap<Point, u32> = HashMap::new();
    let mut trailhead_candidates: Vec<Point> = Vec::new();

    input::lines_indexed(INPUT_PATH, |y, r| {
        width = r.len();
        height = y + 1;
        for (x, c) in r.chars().enumerate() {
            let point = Point { x, y };
            let elevation = c.to_digit(10).unwrap();
            map.insert(point, elevation);
            if elevation == MIN_ELEVATION {
                trailhead_candidates.push(point);
            }
        }
    });

    return (map, Rect { width, height }, trailhead_candidates);
}

fn print_map(map: &HashMap<Point, u32>, bounds: &Rect) {
    for (point, overflow) in bounds.into_iter() {
        if overflow {
            println!();
        }

        match map.get(&point) {
            Some(height) => print!("{height}"),
            None => continue
        }
    }
    println!();
    println!();
}

fn find_trails(map: &HashMap<Point, u32>, bounds: Rect, from: Point, heading: Direction, current_elevation: u32, peaks_reached: &mut HashSet<Point>) -> u32 {
    if current_elevation == MAX_ELEVATION {
        peaks_reached.insert(from);
        return 1;
    }

    let next_elevation = current_elevation + 1;

    let mut rating = 0;
    for direction in heading.into_iter() {
        match direction.offset().apply(&from) {
            Some(to) => {
                if bounds.contains(&to) {
                    match map.get(&to) {
                        Some(elevation) => {
                            if *elevation == next_elevation {
                                rating += find_trails(map, bounds, to, direction, next_elevation, peaks_reached);
                            }
                        },
                        None => panic!("point not within map"),
                    }
                }
            },
            None => ()
        }
    }
    return rating;
}
