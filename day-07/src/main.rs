use core::fmt;
use std::{collections::VecDeque, fs};

// const INPUT_PATH: &str = "./data/test_input.txt";
const INPUT_PATH: &str = "./data/puzzle_input.txt";

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>
}

enum Operator {
    Add,
    Multiply,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
        }
    }
}

impl Operator {
    fn values() -> Vec<Operator> {
        vec![Operator::Add, Operator::Multiply]
    }

    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

fn main() {
    match fs::read_to_string(INPUT_PATH) {
        Ok(data) => {
            let equations = parse_equations(&data);

            let mut total: u64 = 0;
            for equation in equations {
                if is_true(&equation) {
                    total += equation.result;
                }
            }
            println!("total calibration result: {total}");
        },
        Err(e) => {
            println!("failed to load input: {e:?}");
        }
    }
}

fn parse_equations(data: &str) -> Vec<Equation> {
    let mut equations = Vec::new();
    for line in data.split_terminator("\n") {
        let (result, values) = line.split_once(":").unwrap();
        let equation = Equation {
            result: result.parse::<u64>().unwrap(),
            values: values.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect()
        };
        equations.push(equation);
    }
    return equations;
}

fn is_true(equation: &Equation) -> bool {
    match equation.values.split_first() {
        None => false,
        Some((first, remaining_values)) => {
            match evaluate(equation.result, *first, remaining_values) {
                None => false,
                Some(mut operators) => {
                    for value in equation.values.iter() {
                        print!("{value}");
                        match operators.pop_front() {
                            None => (),
                            Some(o) => print!(" {o} ")
                        }
                    }
                    println!(" = {}", equation.result);
                    return true;
                }
            }
        }
    }
}

fn evaluate(target_result: u64, running_result: u64, remaining_values: &[u64]) -> Option<VecDeque<Operator>> {
    match remaining_values.first() {
        None => {
            if running_result == target_result {
                Some(VecDeque::new())
            } else {
                None
            }
        },
        Some(next_value) => {
            for operator in Operator::values() {
                match evaluate(target_result, operator.evaluate(running_result, *next_value), &remaining_values[1..]) {
                    None => continue,
                    Some(mut operators) => {
                        operators.push_front(operator);
                        return Some(operators);
                    }
                }
            }
            return None;
        }
    }
}
