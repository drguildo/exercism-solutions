use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();
    for factor in factors {
        let mut n = 1;
        while (n * factor) < limit {
            multiples.insert(n * factor);
            if *factor == 0 {
                break;
            }
            n += 1;
        }
    }
    multiples.iter().sum()
}
