
#[derive(Clone, PartialEq)]
enum  Num { Prime, Composite }

const BUFCAP: usize = 1_000_000;

/// An object for calculating and caching consecutive prime numbers
///
/// # Examples
///
/// ```
/// //examples go here
/// ```
pub struct PCache {
    primes: Vec<(usize, usize)>, // (prime, offset)
    buffer: Vec<Num>,
    max_checked: usize
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
        // Consider passing in arguments to initialize primes with
        // capaciity and to allow the user to set the BUFCAP.
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

    fn check_next(&mut self, length: usize) {

        // max_checked + 1 is the smallest next prime we might find. If
        // max_checked + 1 < k, then it has a multiple within the buffer
        debug_assert!(self.max_checked + 1 >= length, "buffer may contain \
        multiples of it's own primes. Choose a smaller buffer size");
        debug_assert!(length <= self.buffer.capacity(), "length of segment \
        to check larger than buffer capacity");

        // Remove composite numbers
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
                // new_prime >= max_checked + 1 >= length
                self.primes.push((new_prime, offset + new_prime - length));
            } else {
                self.buffer[offset] = Num::Prime;
            }
        }
        self.max_checked += length;
    }

}
