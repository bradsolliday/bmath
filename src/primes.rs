
#[derive(Clone, PartialEq)]
enum  Num { Prime, Composite }

const BUFCAP: usize = 10_000_000;

pub struct PList {
    pub primes: Vec<(usize, usize)>, // (prime, offset)
    buffer: Vec<Num>,
    max_checked: usize
}

impl PList {

    pub fn new(initial_cap: usize) -> PList {
        PList {
            primes: Vec::with_capacity(initial_cap),
            buffer: vec![Num::Prime; BUFCAP],
            max_checked: 1
        }
    }

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
        
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn prime_test() {
    //    let n = 10000000;
    //    let mut plist = PList::new(n);
    //    println!("{}th prime: {}", n, plist.nth_prime(n));
    //    println!("Calculated {} primes", plist.primes.len());
    //    //println!("Cached primes: {:?}", plist.cached_primes());
    //}
}
