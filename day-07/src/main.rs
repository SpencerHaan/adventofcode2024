use core::fmt;
use std::collections::VecDeque;

// const INPUT_PATH: &str = "./data/day_07_test_input.txt";
const INPUT_PATH: &str = "./data/day_07_puzzle_input.txt";

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>
}

enum Operator {
    Add,
    Multiply,
    Concat
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
            Operator::Concat => write!(f, "||")
        }
    }
}

impl Operator {
    fn values() -> Vec<Operator> {
        vec![Operator::Add, Operator::Multiply, Operator::Concat]
    }

    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => format!("{a}{b}").parse::<u64>().unwrap()
        }
    }
}

fn main() {
    let equations = load_equations();

    let mut total: u64 = 0;
    for equation in equations {
        if is_true(&equation) {
            total += equation.result;
        }
    }
    println!("total calibration result: {total}");
}

fn load_equations() -> Vec<Equation> {
    let mut equations = Vec::new();
    input::lines(INPUT_PATH, |line| {
        let (result, values) = line.split_once(":").unwrap();
        let equation = Equation {
            result: result.parse::<u64>().unwrap(),
            values: values.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect()
        };
        equations.push(equation);
    });
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
