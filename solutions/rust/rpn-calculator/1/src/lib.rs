#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            CalculatorInput::Add => {
                if let Some((x, y)) = fetch_operands(stack.pop(), stack.pop()) {
                    stack.push(y + x);
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if let Some((x, y)) = fetch_operands(stack.pop(), stack.pop()) {
                    stack.push(y - x);
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if let Some((x, y)) = fetch_operands(stack.pop(), stack.pop()) {
                    stack.push(y * x);
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if let Some((x, y)) = fetch_operands(stack.pop(), stack.pop()) {
                    stack.push(y / x);
                } else {
                    return None;
                }
            }
        }
    }

    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}

fn fetch_operands(x: Option<i32>, y: Option<i32>) -> Option<(i32, i32)> {
    if let Some(x) = x {
        if let Some(y) = y {
            return Some((x, y));
        } else {
            return None;
        }
    } else {
        return None;
    }
}
