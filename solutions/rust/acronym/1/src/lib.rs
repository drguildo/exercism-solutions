pub fn abbreviate(phrase: &str) -> String {
    let mut abbreviation = String::new();
    let words: Vec<&str> = phrase.split([' ', '-']).collect();
    for word in words {
        if word.len() > 0 {
            let uppercase: String = word.chars().filter(|c| c.is_uppercase()).collect();
            if word.len() != uppercase.len() && uppercase.len() > 0 {
                abbreviation.push_str(&uppercase.to_string());
                if word.ends_with(":") {
                    return abbreviation;
                }
            } else {
                let first_letter = word.chars().next().unwrap();
                abbreviation.push_str(&first_letter.to_uppercase().to_string());
            }
        }
    }
    abbreviation
}
