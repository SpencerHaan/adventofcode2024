use std::collections::VecDeque;
use input;
use plane;

// const INPUT_PATH: &str = "./data/day_04_test_input.txt";
const INPUT_PATH: &str = "./data/day_04_puzzle_input.txt";

type LetterGrid = Vec<Vec<char>>;

fn main() {
    let grid = load_grid();
    for row in &grid {
        println!("{row:?}");
    }

    let count = count_xmas(&grid);
    println!("{count:?}");
}

fn load_grid() -> LetterGrid {
    let mut grid: LetterGrid = Vec::new();
    input::lines(INPUT_PATH, |line| {
        grid.push(line.chars().collect());
    });
    return grid;
}

fn count_xmas(grid: &LetterGrid) -> i32 {
    let y_max: usize = grid.len();
    let mut count = 0;
    for y in 0..y_max {
        let x_max: usize = grid[y].len();
        for x in 0..x_max {
            let char = grid[y][x];
            if char == 'A' && is_xmas(plane::Point { x, y }, grid, &plane::Point { x: x_max - 1, y: y_max - 1 }) {
                count += 1;
            }
        }
    }
    return count;
}

fn is_xmas(at: plane::Point, grid: &LetterGrid, limit: &plane::Point) -> bool {
    let mut matched_offsets: Vec<plane::Offset> = Vec::new();
    for y in [-1, 1] {
        for x in [-1, 1] {
            let offset = plane::Offset::from(x, y);
            match offset.apply_within(&at, limit) {
                None => continue,
                Some(origin) => {
                    if is_word(&origin, &offset.inverse(), grid, limit, VecDeque::from(vec!['M', 'A', 'S'])) {
                        matched_offsets.push(offset);
                    }
                }
            }
        }
    }
    if matched_offsets.len() >= 2 {
        println!("matched at {at:?} with offsets {matched_offsets:?}");
        return true;
    }
    return false;
}

fn is_word(origin: &plane::Point, offset: &plane::Offset, grid: &LetterGrid, limit: &plane::Point, mut word: VecDeque<char>) -> bool {
    match word.pop_front() {
        None => return true,
        Some(word_char) => {
            let grid_char = grid[origin.y][origin.x];
            if grid_char == word_char {
                match offset.apply_within(origin, limit) {
                    None => return word.is_empty(),
                    Some(point) => {
                        return is_word(&point, offset, grid, limit, word);
                    }
                }
            }
        }
    }
    return false;
}
