pub fn egg_count(display_value: u32) -> usize {
    let mut n = display_value;
    let mut number_of_eggs = 0;
    while n > 0 {
        number_of_eggs += (n & 1) as usize;
        n >>= 1;
    }
    number_of_eggs
}
