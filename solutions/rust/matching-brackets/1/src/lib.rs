pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        if c == '[' || c == '{' || c == '(' {
            stack.push(c);
        }

        if c == ']' || c == '}' || c == ')' {
            if let Some(matching) = stack.pop() {
                if c == ']' && matching != '[' {
                    return false;
                }
                if c == '}' && matching != '{' {
                    return false;
                }
                if c == ')' && matching != '(' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
