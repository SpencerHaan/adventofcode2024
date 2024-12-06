use std::{collections::HashSet, fs};

type Floorplan = Vec<Vec<char>>;

// const INPUT_PATH: &str = "./data/test_input.txt";
const INPUT_PATH: &str = "./data/puzzle_input.txt";

const CARDINAL_GUARDS: [char; 4] = ['<', '^', '>', 'v'];

#[derive(Debug)]
enum Transform {
    None,
    Decrease(usize),
    Increase(usize)
}

impl Transform {
    fn apply(&self, value: usize) -> Option<usize> {
        return match self {
            Transform::None => Some(value),
            Transform::Decrease(amount) => {
                if value == 0 { None }
                else { Some(value - amount) }
            },
            Transform::Increase(amount) => Some(value + amount),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Offset {
    x: Transform,
    y: Transform
}

impl Offset {
    fn apply(&self, point: &Point) -> Option<Point> {
        let x = self.x.apply(point.x)?;
        let y = self.y.apply(point.y)?;
        return Some(Point { x, y });
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn offset(&self) -> Offset {
        match self {
            Direction::Up => Offset { x: Transform::None, y: Transform::Decrease(1) },
            Direction::Right => Offset { x: Transform::Increase(1), y: Transform::None },
            Direction::Down => Offset { x: Transform::None, y: Transform::Increase(1) },
            Direction::Left => Offset { x: Transform::Decrease(1), y: Transform::None },
        }
    }

    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn main() {
    match fs::read_to_string(INPUT_PATH) {
        Ok(data) => {
            let floorplan = parse_floorplan(&data);
            print_floorplan(&floorplan);

            match find_guard(&floorplan) {
                None => println!("where's the guard?"),
                Some((start, direction)) => {
                    println!("starting at {start:?} and facing {direction:?}");

                    let mut visited = HashSet::new();
                    walk(start, direction, &floorplan, |p| -> () {
                        visited.insert(p);
                    });
                    println!("distinct positions visited: {}", visited.len());
                }
            }
        },
        Err(e) => {
            println!("failed to load updates: {e:?}");
        }
    }
}

fn parse_floorplan(data: &str) -> Floorplan {
    let mut floorplan: Floorplan = Floorplan::new();
    for line in data.split_terminator("\n") {
        floorplan.push(line.chars().collect());
    }
    return floorplan;
}

fn print_floorplan(floorplan: &Floorplan) {
    let height = floorplan.len();
    let width = floorplan[0].len();
    for line in floorplan {
        for c in line {
            print!("{c}");
        }
        println!();
    }
    println!("floor plan is {width} x {height}");
    println!();
}

fn find_guard(floorplan: &Floorplan) -> Option<(Point, Direction)> {
    for (y, line) in floorplan.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if CARDINAL_GUARDS.contains(c) {
                return as_guard_direction(*c)
                    .map(|d| (Point { x, y }, d));
            }
        }
    }
    return None;
}

fn as_guard_direction(char: char) -> Option<Direction> {
    if char == '^' { Some(Direction::Up) }
    else if char == '>' { Some(Direction::Right) }
    else if char == 'v' { Some(Direction::Down) }
    else if char == '<' { Some(Direction::Left) }
    else { None }
}

fn walk<F>(from: Point, direction: Direction, floorplan: &Floorplan, mut visit: F) where F: FnMut(Point) {
    match direction.offset().apply(&from) {
        None => {
            println!("we're out at {from:?}!");
            return;
        },
        Some(to) => {
            if to.y >= floorplan.len() || to.x >= floorplan[to.y].len() {
                println!("we're out at {from:?}!");
                return;
            }

            let c: char = floorplan[to.y][to.x];
            if c == '#' {
                // turn
                return walk(from, direction.next(), floorplan, visit);
            } else {
                // continue
                visit(to.clone());
                return walk(to, direction, floorplan, visit);
            }
        }
    }
}
