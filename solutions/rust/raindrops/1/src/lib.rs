pub fn raindrops(n: u32) -> String {
    let has_factor = |factor: u32| -> bool { n % factor == 0 };

    let mut s = String::new();

    let mut no_factors = true;
    if has_factor(3) {
        s.push_str("Pling");
        no_factors = false;
    }
    if has_factor(5) {
        s.push_str("Plang");
        no_factors = false;
    }
    if has_factor(7) {
        s.push_str("Plong");
        no_factors = false;
    }

    if no_factors {
        s = format!("{}", n);
    }

    s
}
