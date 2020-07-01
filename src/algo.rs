
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
pub fn gcd_factors(mut m: i64, mut n: i64) -> (i64, i64, i64) {
    debug_assert!(m > 0 && n > 0, "Arguments must be positive");
    // Invariant M: m = x*m_original + y*n_original
    // Invariant N: n = a*m_original + b*n_original
    let (mut x, mut y) = (1, 0);
    let (mut a, mut b) = (0, 1);
    // M and N made true
    let mut q;
    if n > m {
        q = n / m; // Note, these two divisons are optimized to be
        n = n % m; // one instruction in assembly
        // M true
        // n == 0 => gcd = m
        if n == 0 { return (m, x, y); }
        a = a - q*x;
        b = b - q*y;
        // M still true, N made true
    }
    loop {
        // M and N true
        q = m / n;
        m = m % n; 
        // N true
        // m == 0 => gcd = n
        if m == 0 { return (n, a, b); }
        x = x - q*a;
        y = y - q*b;
        // N still true, M made true
        
        q = n / m;
        n = n % m;
        // M true
        // n == 0 => gcd = m
        if n == 0 { return (m, x, y); }
        a = a - q*x;
        b = b - q*y;
        // M still true, N made true
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
