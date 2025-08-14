pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut primes = vec![];

    let mut candidate = 2_u32;
    while primes.len() < n + 1 {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }

    primes[n]
}

fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5_u32;
    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6
    }

    return true;
}
