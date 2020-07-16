
extern crate num_traits;
extern crate num;

mod primes;

pub mod algo;
pub mod cfrac;
pub use primes::PCache;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
