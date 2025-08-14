pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];

    let mut dividend: u64 = n;
    let mut divisor: u64 = 2;
    let mut result: u64;
    loop {
        if dividend == 1 {
            break;
        }
        if dividend % divisor == 0 {
            result = dividend / divisor;
            factors.push(divisor);
            dividend = result;
        } else {
            divisor += 1;
        }
    }

    factors
}
