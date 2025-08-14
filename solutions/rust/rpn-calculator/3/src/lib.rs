#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::{Add, Divide, Multiply, Subtract, Value};

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    fn handle<F>(stack: &mut Vec<i32>, func: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(func(a, b));
    }

    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        let enough = stack.len() > 1;
        match input {
            Value(value) => stack.push(*value),
            Add if enough => handle(&mut stack, |x, y| y + x),
            Subtract if enough => handle(&mut stack, |x, y| y - x),
            Multiply if enough => handle(&mut stack, |x, y| y * x),
            Divide if enough => handle(&mut stack, |x, y| y / x),
            _ => return None,
        }
    }

    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}
