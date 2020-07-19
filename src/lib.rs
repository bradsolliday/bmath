
//! This crate contains code written by Bradley Solliday for the purpose of
//! learning rust, demoing cool ideas, and getting better at programming more
//! generally.
//!
//! The coolest things in the crate at the moment are:<br>
//! -The implementation of the sturct PCache for efficiently calculating prime
//! numbers.<br>
//! -The method CFrac::convergents() for calculating the convergents of
//! continued fractions, which are the "best rational approximations of the
//! second kind" of a given real number. This also ties in to the fibonacci
//! numbers. Make sure to check out the source code.

extern crate num_traits;
extern crate num;

mod primes;
mod cfrac;

pub mod algo;
pub use cfrac::CFrac;
pub use primes::PCache;
