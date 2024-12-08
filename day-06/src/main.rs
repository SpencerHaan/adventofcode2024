use std::{collections::HashSet, fs, hash::Hash};

// const INPUT_PATH: &str = "./data/test_input.txt";
const INPUT_PATH: &str = "./data/puzzle_input.txt";

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Guard {
    position: Point,
    direction: Direction
}

impl Guard {
    fn next(&self) -> Option<Point> {
        self.direction.offset().apply(&self.position)
    }

    fn step(&mut self, to: Point) {
        self.position = to
    }

    fn turn(&mut self) {
        self.direction = self.direction.next()
    }
}

struct Level {
    limit: Point,
    obstacles: HashSet<Point>
}

impl Level {
    fn is_obstacle(&self, point: &Point) -> bool {
        self.obstacles.contains(point)
    }

    fn with_obstacle(&self, obstacle: Point) -> Level {
        let mut obstacles = self.obstacles.clone();
        obstacles.insert(obstacle);

        Level {
            limit: self.limit,
            obstacles
        }
    }
}

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
            match parse_level(&data) {
                Ok((level, guard)) =>  {
                    print_level(&level, &guard);
                    println!("{guard:?}");

                    find_loop_obstacles(&level, &guard);
                },
                Err(m) => println!("failed to parse level: {m}")
            }
        },
        Err(e) => {
            println!("failed to load level: {e:?}");
        }
    }
}

fn parse_level(data: &str) -> Result<(Level, Guard), &str> {
    let mut guard = Err("guard not found");
    let mut obstacles: HashSet<Point> = HashSet::new();

    let mut x_limit: usize = 0;
    let mut y_limit: usize = 0;

    for (y, r) in data.split_terminator("\n").enumerate() {
        y_limit += 1;
        x_limit = r.len();

        for (x, c) in r.chars().enumerate() {
            let point = Point { x, y };
            if c == '#' {
                obstacles.insert(point);
                continue;
            } else if c == '^' {
                guard = Ok(Guard {
                    position: point,
                    direction: Direction::Up
                })
            }
        }
    }

    return Ok((
        Level {
            limit: Point { x: x_limit, y: y_limit },
            obstacles
        },
        guard?,
    ));
}

fn print_level(level: &Level, guard: &Guard) {
    for y in 0..level.limit.y {
        for x in 0..level.limit.x {
            let point = Point { x, y };
            if level.obstacles.contains(&point) {
                print!("#");
            } else if guard.position == point {
                print!("^");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn debug_print_level(level: &Level, guard: &Guard, visited: &HashSet<Guard>, test_obstacle: &Point) {
    let visited_points: HashSet<Point> = HashSet::from_iter(visited.iter().map(|g| g.position));
    for y in 0..level.limit.y {
        for x in 0..level.limit.x {
            let point = Point { x, y };
            if point == *test_obstacle {
                print!("0");
            } else if visited_points.contains(&point) {
                print!("X")
            } else if level.obstacles.contains(&point) {
                print!("#");
            } else if guard.position == point {
                print!("^");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn find_loop_obstacles(level: &Level, guard: &Guard) {
    let mut loop_obstacles = HashSet::new();
    walk(*guard, level, |ghost| {
        let to = ghost.next().unwrap();
        let new_level = level.with_obstacle(to);

        let mut visited = HashSet::new();
        walk(*guard, &new_level, |g: Guard| {
            if visited.contains(&g) {
                loop_obstacles.insert(to);
                return false;
            } else {
                visited.insert(g);
                return true;
            }
        });
        return true;
    });
    println!("loop obstacle count: {}", loop_obstacles.len());
}

fn walk<F>(mut guard: Guard, level: &Level, mut cont: F) where F: FnMut(Guard) -> bool {
    match guard.next() {
        None => (),
        Some(to) => {
            if to.x >= level.limit.x || to.y >= level.limit.y {
                return;
            }

            if level.is_obstacle(&to) {
                guard.turn();
            } else {
                if !cont(guard) {
                    return;
                }
                guard.step(to);
            }
            return walk(guard, level, cont);
        }
    }
}
