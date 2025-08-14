pub fn collatz(mut n: u64) -> Option<u64> {
    let mut steps = 0;

    if n == 0 {
        return None;
    }

    while n != 1 {
        steps += 1;

        if n % 2 == 0 {
            n = n / 2;
        } else {
            if let Some(result) = n.checked_mul(3).and_then(|x| x.checked_add(1)) {
                n = result;
            } else {
                return None;
            }
        }
    }

    return Some(steps);
}
