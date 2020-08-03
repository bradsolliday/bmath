extern crate bmath;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use bmath::PCache as PCacheImpl;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PCache {
    cache: PCacheImpl
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl PCache {

    pub fn new(bufcap: usize) -> PCache {
        console_error_panic_hook::set_once();
        PCache {
            cache: PCacheImpl::new(bufcap)
        }
    }

    pub fn nth_prime(&mut self, n: usize) -> usize {
        self.cache.nth_prime(n)
    }
}

#[wasm_bindgen]
pub struct NaivePCache {
    primes: Vec<usize>,
    max_checked: usize
}

#[wasm_bindgen]
impl NaivePCache {

    pub fn new() -> NaivePCache {
        NaivePCache {
            primes: Vec::new(),
            max_checked: 1
        }
    }

    pub fn nth_prime(&mut self, n: usize) -> usize {
        assert!(n > 0, "The 0th prime is not well defined");
        let mut smallest_unchecked = self.max_checked + 1;
        let mut found_divisor = false;
        while self.primes.len() < n {
            for p in &self.primes {
                if smallest_unchecked % p == 0 {
                    found_divisor = true;
                    break;
                }
            }
            if !found_divisor {
                self.primes.push(smallest_unchecked);
                self.max_checked = smallest_unchecked;
            }
            found_divisor = false;
            smallest_unchecked += 1;
        }
        self.primes[n - 1]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_primes() {
        let mut cache = PCache::new(100_000);
        assert_eq!(cache.nth_prime(7400), 75079);

        let mut naive_cache = NaivePCache::new();
        assert_eq!(naive_cache.nth_prime(300), 1987);
    }
}
