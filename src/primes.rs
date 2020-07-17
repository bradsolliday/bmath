
#[derive(Clone, PartialEq)]
enum  Num { Prime, Composite }

const BUFCAP: usize = 1_000_000;

/// An object for calculating and caching consecutive prime numbers
///
/// # Method of Calculation
///
/// Based on the Sieve of Eratosthenes, except modified to allow calculation
/// of arbitrarily large prime numbers (only space to store the generated list
/// of primes is needed).
///
/// Works by maintaining two sets of data. First, a buffer which acts a bit like
/// the number table typically used when discussing the Sieve of Eratosthenes.
/// Roughly, the number n is represented by the nth element of the buffer,
/// and buffer\[n\] is a marker indicating whether or not n has been shown to be
/// a composite number.
///
/// The second is a list of prime numbers, each paired with their smallest
/// multiple less than the max_checked number (checked to be either prime or
/// composite).
///
/// To find new primes, we loop through the list of primes, and, starting with
/// that paired multiple, start writing to all multiples of the prime in the
/// bugger an indicator that the multiple is a composite number.
///
/// In this way we can mark every composite number in the buffer as such, and
/// then loop through the buffer, knowing that every left over value not marked
/// as composite must therefore be prime, adding them to our list of primes.
///
/// What makes this technique interesting is that we can calculate arbitrarily
/// large primes wihout needing excessive space beyond that needed to store our
/// results.
///
/// The buffer doesn't actually start from 2, as in the Sieve of Eratosthenes,
/// but rather starts after the last set of checked values. The real reason we
/// needed to pair our primes with their next unchecked multiple was because
/// we need to know where we left off when we start a new buffer after having
/// filled out an old one. 
///
/// For the details you can examine the source code. For example, the primes
/// are not actually paired with their next multiple, but rather the index of
/// where that next multiple would be if it were in the buffer.
///
/// # Additional Optimizations
///
/// After having finished this, I was curious of how others have calcualted
/// prime numbers. It turns out that this is a common techique for calculating
/// primes.
///
/// However, one optimazation that other used that I didn't was to
/// observe that if p is a prime number, then no number less that p^2 is a
/// multiple of p but not a multiple of some prime number less than p (if
/// m < p, then m is divisible by some prime less than p, and hence m*p is
/// divisible by some prime less than p).
///
/// This, of course, means we don't need to start checking to see if p is a
/// divisor until we get to p^2. For must numbers, p << p^2, and so we are
/// spared from having to check if a significant of our primes are divisors.
///
/// Of course, I've since implemented this optimization.
///
/// # Examples
///
/// ```
/// use bmath::PCache;
///
/// let mut pc: PCache = PCache::new();
///
/// let cache: Vec<usize> = pc.cached_primes();
///
/// assert_eq!(cache.len(), 0);
/// 
/// assert_eq!(pc.nth_prime(1), 2);
/// assert_eq!(pc.nth_prime(2), 3);
/// assert_eq!(pc.nth_prime(3), 5);
/// assert_eq!(pc.nth_prime(156), 911);
/// assert_eq!(pc.nth_prime(307), 2027);
/// assert_eq!(pc.nth_prime(308), 2029);
///
/// let cache: Vec<usize> = pc.cached_primes();
/// 
/// assert!(cache.len() >= 308);
///
/// assert_eq!(cache[0], 2);
/// assert_eq!(cache[1], 3);
/// assert_eq!(cache[2], 5);
/// assert_eq!(cache[155], 911);
/// assert_eq!(cache[306], 2027);
/// assert_eq!(cache[307], 2029);
/// ```
pub struct PCache {
    // Invariants:
    // 1) primes[n-1].1 + max_checked + 1 is the smallest multiple of
    // primes[n-1].0 greater or equal to both max_checked + 1
    // and primes[n-1].0 * primes[n-1].0.
    // 2) primes[n-1].0 is the nth prime number (2 is the first prime number).
    // 3) If num > 1 and num <= max_checked, then num has been been proven to
    // either be composite or prime.
    //
    // buffer[primes[n-1].1] represents this first multiple of primes[n-1].0
    // meeting the above requirements. buffer "buffers" the steam of natural
    // numbers.

    primes: Vec<(usize, usize)>, // (prime, offset)
    max_checked: usize,
    buffer: Vec<Num>,
}

impl PCache {

    /// Returns a new empty PCache.
    /// No primes will be calculated until a prime is requested from it.
    ///
    /// # Examples
    ///
    /// ```
    /// use bmath::PCache;
    ///
    /// let mut cache: PCache = PCache::new();
    /// ```
    pub fn new() -> PCache {
        // buffer is initialized to Num::Prime as numbers are prime if
        // they haven't been found to be composite.

        // Consider allowing the user to pass in arguments to set the size
        // of the buffer (perhaps reset it too). Big affect on performance
        // and the user should have control.
        // Initializing primes with capcity doesn't seem to have a noticeable
        // affect
        PCache {

            primes: Vec::new(),
            buffer: vec![Num::Prime; BUFCAP],
            max_checked: 1 // 2 is the first unchecked value
        }
    }

    /// Returns the nth prime number. First checks if nth prime has been
    /// cached and only calculates new primes if it hasn't. Caches all
    /// newly computed primes.
    ///
    /// Precondition: n > 0
    ///
    /// # Examples
    ///
    /// ```
    /// use bmath::PCache;
    ///
    /// let mut primes = PCache::new();
    ///
    /// assert_eq!(2,  primes.nth_prime(1));
    /// assert_eq!(3,  primes.nth_prime(2));
    /// assert_eq!(5,  primes.nth_prime(3));
    /// assert_eq!(11, primes.nth_prime(5));
    /// ```
    pub fn nth_prime(&mut self, n: usize) -> usize {
        assert!( n > 0, "0th prime not well defined");
        while n > self.primes.len() {
            self.check_next(
                if self.max_checked < BUFCAP {
                    self.max_checked + 1
                } else {
                    BUFCAP
                });
        }
        self.primes[n - 1].0
    }

    /// Returns a new Vec<usize> of all primes cached so far. The outputs
    /// nth element (index 0 is the first element) is the nth prime
    ///
    /// # Examples
    ///
    /// ```
    /// use bmath::PCache;
    ///
    /// let mut primes = PCache::new();
    /// let cached_vals = primes.cached_primes();
    ///
    /// assert_eq!(cached_vals.len(), 0);
    ///
    /// let p50 = primes.nth_prime(50);
    /// let cached_vals = primes.cached_primes();
    ///
    /// assert!(cached_vals.len() >= 50);
    ///
    /// assert_eq!(cached_vals[49], p50);
    /// assert_eq!(cached_vals[0], 2);
    /// assert_eq!(cached_vals[4], 11);
    /// ```
    pub fn cached_primes(&self) -> Vec<usize> {
        self.primes.iter().map(|(p,_)| *p).collect()
    }

    // Checks the next length numbers after self.max_checked and adds any
    // found prime numbers to self.primes.
    fn check_next(&mut self, length: usize) {

        // max_checked + 1 is the smallest next prime we might find. If
        // max_checked + 1 < k, then it has a multiple within the buffer
        debug_assert!(self.max_checked + 1 >= length, "buffer may contain \
        multiples of it's own primes. Choose a smaller buffer size");
        debug_assert!(length <= self.buffer.capacity(), "length of segment \
        to check larger than buffer capacity");

        // Remove composite numbers from self.buffer
        for i in 0..self.primes.len() {
            let (p, mut offset) = self.primes[i];
            while offset < length {
                self.buffer[offset] = Num::Composite;
                offset += p;
            }
            self.primes[i].1 = offset - length;
        }

        // Add new-found primes to list
        for offset in 0..length {
            if self.buffer[offset] == Num::Prime {
                let new_prime = self.max_checked + 1 + offset;
                let new_offset =
                    offset + new_prime * new_prime - new_prime - length;
                self.primes.push((new_prime, new_offset));
            } else {
                self.buffer[offset] = Num::Prime;
            }
        }
        self.max_checked += length;
    }

}


#[cfg(test)]
mod tests {
    use super::PCache;

    const PRIMES: [usize; 32] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109,
    113, 127, 131];

    const P1000: usize = 7919; // The 1000th prime according to the internet

    #[test]
    fn test_nth_prime() {
        let mut pc = PCache::new();
        assert_eq!(pc.nth_prime(2) , PRIMES[1]);
        assert_eq!(pc.nth_prime(15), PRIMES[14]);
        assert_eq!(pc.nth_prime(30), PRIMES[29]);
        assert_eq!(pc.nth_prime(11), PRIMES[10]);
        assert_eq!(pc.nth_prime(25), PRIMES[24]);
        assert_eq!(pc.nth_prime(1000), P1000);
    }

    #[test]
    fn test_cached_primes() {
        let mut pc = PCache::new();
        pc.nth_prime(1000);
        let cache = pc.cached_primes();
        for (n, &p) in cache.iter().enumerate() {
            assert_eq!(pc.nth_prime(n + 1), p)
        }
    }
}
