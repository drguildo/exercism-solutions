pub fn reply(message: &str) -> &str {
    let message: String = message.chars().filter(|c| !c.is_whitespace()).collect();
    if message.ends_with('?') {
        if is_all_caps(&message) {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    }

    if is_all_caps(&message) {
        return "Whoa, chill out!";
    }

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    return "Whatever.";
}

fn is_all_caps(message: &str) -> bool {
    let mut chars = message.chars();
    chars.any(|c| c.is_alphabetic()) && !chars.any(|c| c.is_lowercase())
}
