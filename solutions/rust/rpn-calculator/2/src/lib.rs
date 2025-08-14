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
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(y + x);
            }
            CalculatorInput::Subtract => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(y - x);
            }
            CalculatorInput::Multiply => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(y * x);
            }
            CalculatorInput::Divide => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(y / x);
            }
        }
    }

    if stack.len() > 1 {
        None
    } else {
        stack.pop()
    }
}
