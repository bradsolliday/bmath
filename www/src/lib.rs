extern crate bmath;

use wasm_bindgen::prelude::*;
use bmath::PCache;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn nth_prime(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut pc = PCache::new();
    pc.nth_prime(n)
}

//// This is like the `main` function, except for JavaScript.
//#[wasm_bindgen(start)]
//pub fn main_js() -> Result<(), JsValue> {
//    // This provides better error messages in debug mode.
//    // It's disabled in release mode so it doesn't bloat up the file size.
//    #[cfg(debug_assertions)]
//    console_error_panic_hook::set_once();
//
//
//    // Your code goes here!
//    let mut pc = PCache::new();
//    let n = 300;
//    let pn = pc.nth_prime(n);
//    let s = &format!("The {}th prime is {}", n, pn);
//    console::log_1(&JsValue::from_str(s));
//
//    Ok(())
//}
