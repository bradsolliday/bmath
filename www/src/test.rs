
fn main() {
    let ones: u32 = -1_i32 as u32;
    println!("ones (-1):         {:b}", ones);
    let x: u32 = -8_i32 as u32;
    println!("-8 (as 1s and 0s): {:b}", x);
    println!("-8 inverted:       {:032b}", x ^ ones);
    println!("-8 inverted + 1:   {:032b} (this is just {} in binary)", (x ^ ones) + 1, (x ^ ones) + 1);
}
