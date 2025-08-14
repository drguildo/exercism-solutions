pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let first_char = student.chars().next().unwrap().to_ascii_uppercase();
    assert!(first_char.is_ascii_alphabetic() && first_char >= 'A' && first_char <= 'L');

    let index = (first_char as u8 - b'A') as usize;

    let rows: Vec<&str> = diagram.split_ascii_whitespace().collect();
    assert!(rows.len() == 2 && rows[0].len() == rows[1].len());

    let offset = index * 2;
    assert!(offset + 1 < rows[0].len());

    let mut flowers = Vec::with_capacity(4);
    for row in &rows {
        flowers.push(flower_code_to_name(row.chars().nth(offset).unwrap()));
        flowers.push(flower_code_to_name(row.chars().nth(offset + 1).unwrap()));
    }

    flowers
}

fn flower_code_to_name(code: char) -> &'static str {
    match code.to_ascii_uppercase() {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => panic!("Invalid flower code: {}", code),
    }
}

