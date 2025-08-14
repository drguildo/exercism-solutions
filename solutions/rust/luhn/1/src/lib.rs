/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let filtered: Vec<char> = code.chars().filter(|c| *c != ' ').collect();

    if filtered.len() < 2 {
        return false;
    }

    let mut sum: u32 = 0;
    for (i, c) in filtered.iter().rev().enumerate() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if i % 2 != 0 {
                if digit > 4 {
                    sum += (digit * 2) - 9;
                } else {
                    sum += digit * 2;
                }
            } else {
                sum += digit;
            }
        } else {
            return false;
        }
    }

    return (sum % 10) == 0;
}
