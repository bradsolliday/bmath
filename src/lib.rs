pub mod algo;
mod primes;

pub use primes::PCache;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
