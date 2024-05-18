extern crate bmath;
extern crate console_error_panic_hook;

mod wave_grid_f32;
//mod plottable;

pub use wave_grid_f32::WaveGridF32;

//use plottable::Plottable;
use bmath::algo::gcd as gcd_impl;
use bmath::algo::gcd_factors as gcd_factors_impl;
use bmath::PCache as PCacheImpl;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PCache {
    cache: PCacheImpl,
}

#[wasm_bindgen]
pub fn gcd(m: u32, n: u32) -> u32 {
    gcd_impl(m, n)
}

#[wasm_bindgen]
pub struct GCDCoefficients {
    gcd: isize,
    a: isize,
    b: isize,
}

#[wasm_bindgen]
impl GCDCoefficients {
    pub fn gcd(&self) -> isize {
        self.gcd
    }

    pub fn a(&self) -> isize {
        self.a
    }

    pub fn b(&self) -> isize {
        self.b
    }
}

#[wasm_bindgen]
pub fn gcd_factors(m: isize, n: isize) -> GCDCoefficients {
    let (d, a, b) = gcd_factors_impl(m, n);
    GCDCoefficients { gcd: d, a, b }
}

#[wasm_bindgen]
impl PCache {
    pub fn new(bufcap: usize) -> PCache {
        console_error_panic_hook::set_once();
        PCache {
            cache: PCacheImpl::new(bufcap),
        }
    }

    pub fn nth_prime(&mut self, n: usize) -> usize {
        self.cache.nth_prime(n)
    }
}

#[wasm_bindgen]
pub struct NaivePCache {
    primes: Vec<usize>,
    max_checked: usize,
}

#[wasm_bindgen]
impl NaivePCache {
    pub fn new() -> NaivePCache {
        NaivePCache {
            primes: Vec::new(),
            max_checked: 1,
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

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
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
