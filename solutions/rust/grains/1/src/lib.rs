pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    (1..s).fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    (1..=64).map(|n| square(n)).sum()
}
