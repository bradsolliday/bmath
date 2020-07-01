
/// Returns the greatest common divisors of n and m
///     using euclid's algorithm.
/// Precondition: n and m are positive.
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

/// Returns (d, a, b) where d is the gcd of n and m, and
/// a and b such that a*m + b*n = d.
/// Precondition: m and n are positive. 
///
/// # Example
///
/// ```
/// use bmath::algo::gcd_factors;
/// let result = gcd_factors(25, 15);
/// assert_eq!(result, (5, 2, -3));
/// ```
pub fn gcd_factors(m: i64, n: i64) -> (i64, i64, i64) {
    panic!("Not implemented yet");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    // Returns nothing
    // Panics if d is not the gcd of m and n
    fn is_gcd(d: u64, m: u64, n: u64) {
        assert_eq!(m % d, 0);
        assert_eq!(n % d, 0);
        for i in (d + 1)..cmp::min(n, m) {
            assert!(m % i != 0 || n % i != 0);
        }
    }

    #[test]
    fn test_gcd() {
        for m in 1..200 {
            for n in 1..200 {
                let d = gcd(m, n);
                is_gcd(d, m, n);
            }
        }
    }

    #[test]
    fn test_gcd_factors() {
        for m in 1..200 {
            for n in 1..200 {
                let (d, a, b) = gcd_factors(m, n);
                assert_eq!(a*m + b*n, d);
                is_gcd(d as u64, m as u64, n as u64);
            }
        }
    }
}
