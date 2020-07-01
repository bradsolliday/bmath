
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
/// # Precondition
/// m and n are positive. 
///
/// # Example
///
/// ```
/// use bmath::algo::gcd_factors;
/// use std::cmp::min;
/// let (m, n) = (25, 15);
/// let (d, a, b) = gcd_factors(m, n);
/// assert_eq!(a*m + b*n, d);
/// assert_eq!(m % d, 0);
/// assert_eq!(n % d, 0);
/// for k in (1 + d)..min(m, n) {
///     assert!(m % k != 0 || n % k != 0);
/// }
/// ```
pub fn gcd_factors(m: i64, n: i64) -> (i64, i64, i64) {
    debug_assert!(m > 0 && n > 0, "Arguments must be positive");
    let (mut c, mut d) = (m, n);
    let (mut x, mut y) = (1, 0);
    let (mut a, mut b) = (0, 1);
    let (mut q, mut r, mut t);
    loop {
        q = c / d; // Note, these two divisons are optimized to be
        r = c % d; // one instruction in assembly
        if r == 0 { return (d, a, b); }
        c = d;
        d = r;
        t = a;
        a = x - q*a;
        x = t;
        t = b;
        b = y - q*b;
        y = t;
    }
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
