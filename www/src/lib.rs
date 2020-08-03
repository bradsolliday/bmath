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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_primes() {
        let mut cache = PCache::new(100_000);
        assert_eq!(cache.nth_prime(7400), 75079);
    }
}
