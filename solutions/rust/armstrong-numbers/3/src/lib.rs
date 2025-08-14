pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as u32))
        .collect();
    let num_digits = digits.len() as u32;
    let total: u32 = digits.iter().map(|n| n.pow(num_digits)).sum();

    total == num
}
