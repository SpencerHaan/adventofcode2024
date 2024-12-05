use std::{collections::VecDeque, fs, process::exit};

// const INPUT_PATH: &str = "./data/test_input.txt";
const INPUT_PATH: &str = "./data/puzzle_input.txt";

type LetterGrid = Vec<Vec<char>>;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn add_offset(&self, offset: &Offset) -> Offset {
        let x = self.x as i32 + offset.x;
        let y = self.y as i32 + offset.y;
        return Offset { x, y };
    }
}

#[derive(Debug)]
struct Offset {
    x: i32,
    y: i32
}

impl Offset {
    fn to_point(&self, max: &Point) -> Option<Point> {
        return if self.x.is_negative() || self.y.is_negative() || self.x as usize > max.x || self.y as usize > max.y {
            None
        } else {
            Some(Point {
                x: (self.x as usize).clamp(0, max.x),
                y: (self.y as usize).clamp(0, max.y)
            })
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH);
    match input {
        Ok(i) => {
            let grid = to_grid(&i);
            for row in &grid {
                println!("{row:?}");
            }

            let count = count_xmas(&grid);
            println!("{count:?}");
        },
        Err(e) => {
            println!("{e:?}");
            exit(-1);
        }
    }
}

fn to_grid(str: &str) -> LetterGrid {
    return str.split_terminator("\n")
        .map(|l| l.chars().collect())
        .collect();
}

fn count_xmas(grid: &LetterGrid) -> i32 {
    let y_max: usize = grid.len();
    let mut count = 0;
    for y in 0..y_max {
        let x_max: usize = grid[y].len();
        for x in 0..x_max {
            let char = grid[y][x];
            if char == 'X' {
                count += count_mas(Point { x, y }, grid, &Point { x: x_max - 1, y: y_max - 1 });
            }
        }
    }
    return count;
}

fn count_mas(at: Point, grid: &LetterGrid, limit: &Point) -> i32 {
    let mut count = 0;
    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }

            let offset = Offset { x, y };
            if is_word(&at, &offset, grid, limit, VecDeque::from(vec!['M', 'A', 'S'])) {
                println!("matched at {at:?} with offset {offset:?}");
                count += 1;
            }
        }
    }
    return count;
}

fn is_word(origin: &Point, offset: &Offset, grid: &LetterGrid, limit: &Point, mut word: VecDeque<char>) -> bool {
    match word.pop_front() {
        None => return true,
        Some(word_char) => {
            match origin.add_offset(offset).to_point(limit) {
                None => return false,
                Some(point) => {
                    let grid_char = grid[point.y][point.x];
                    if grid_char == word_char {
                        return is_word(&point, offset, grid, limit, word);
                    }
                }
            }
        }
    }
    return false;
}
