//! `funtools` contain a collection of fun iterators to tinker with.
//! For examples, see the [tests directory](https://git.sr.ht/~knarkzel/funtools/tree/master/item/tests).

mod prime;
pub use prime::Prime;

mod fibonacci;
pub use fibonacci::Fibonacci;

mod tribonacci;
pub use tribonacci::Tribonacci;
