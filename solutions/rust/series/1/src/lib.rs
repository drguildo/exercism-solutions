pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut s: Vec<String> = Vec::new();

    let mut i = 0;
    while (i + len) <= digits.len() {
        let substring: String = digits.chars().skip(i).take(len).collect();
        s.push(substring);
        i += 1;
    }

    s
}
