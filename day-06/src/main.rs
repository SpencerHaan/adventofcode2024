use std::{collections::HashSet, hash::Hash};
use input;
use plane;

// const INPUT_PATH: &str = "./data/day_06_test_input.txt";
const INPUT_PATH: &str = "./data/day_06_puzzle_input.txt";

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Guard {
    position: plane::Point,
    direction: plane::Direction
}

impl Guard {
    fn next(&self) -> Option<plane::Point> {
        self.direction.offset().apply(&self.position)
    }

    fn step(&mut self, to: plane::Point) {
        self.position = to
    }

    fn turn(&mut self) {
        self.direction = self.direction.next()
    }
}

struct Level {
    limit: plane::Point,
    obstacles: HashSet<plane::Point>
}

impl Level {
    fn is_obstacle(&self, point: &plane::Point) -> bool {
        self.obstacles.contains(point)
    }

    fn with_obstacle(&self, obstacle: plane::Point) -> Level {
        let mut obstacles = self.obstacles.clone();
        obstacles.insert(obstacle);

        Level {
            limit: self.limit,
            obstacles
        }
    }
}

fn main() {
    match load_level() {
        Ok((level, guard)) =>  {
            print_level(&level, &guard);
            println!("{guard:?}");

            find_loop_obstacles(&level, &guard);
        },
        Err(m) => println!("failed to parse level: {m}")
    }
}

fn load_level() -> Result<(Level, Guard), String> {
    let mut guard = Err("guard not found");
    let mut obstacles: HashSet<plane::Point> = HashSet::new();

    let mut x_limit: usize = 0;
    let mut y_limit: usize = 0;

    input::lines_indexed(INPUT_PATH, |y, r| {
        y_limit += 1;
        x_limit = r.len();

        for (x, c) in r.chars().enumerate() {
            let point = plane::Point { x, y };
            if c == '#' {
                obstacles.insert(point);
                continue;
            } else if c == '^' {
                guard = Ok(Guard {
                    position: point,
                    direction: plane::Direction::Up
                })
            }
        }
    });

    return Ok((
        Level {
            limit: plane::Point { x: x_limit, y: y_limit },
            obstacles
        },
        guard?,
    ));
}

fn print_level(level: &Level, guard: &Guard) {
    for y in 0..level.limit.y {
        for x in 0..level.limit.x {
            let point = plane::Point { x, y };
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

// fn debug_print_level(level: &Level, guard: &Guard, visited: &HashSet<Guard>, test_obstacle: &plane::Point) {
//     let visited_points: HashSet<plane::Point> = HashSet::from_iter(visited.iter().map(|g| g.position));
//     for y in 0..level.limit.y {
//         for x in 0..level.limit.x {
//             let point = plane::Point { x, y };
//             if point == *test_obstacle {
//                 print!("0");
//             } else if visited_points.contains(&point) {
//                 print!("X")
//             } else if level.obstacles.contains(&point) {
//                 print!("#");
//             } else if guard.position == point {
//                 print!("^");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
//     println!();
// }

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
