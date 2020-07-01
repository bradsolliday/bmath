
/// Returns the greatest common divisors of n and m
///     using euclid's algorithm.
/// Precondition: n and m are non-zero
pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    debug_assert!(m > 0 && n > 0, "Arguments must be positive");
    if n > m {
        n = n % m;
        if n == 0 { return m; }
    }
    loop {
        m = m % n;
        if m == 0 { return n; }
        n = n % m;
        if n == 0 { return m; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    #[test]
    fn test_gcd() {
        for m in 1..200 {
            for n in 1..200 {
                let d = gcd(m, n);
                assert_eq!(m % d, 0);
                assert_eq!(n % d, 0);
                for j in (d + 1)..cmp::min(n, m) {
                    assert!(m % j != 0 || n % j != 0);
                }
            }
        }
    }
}
