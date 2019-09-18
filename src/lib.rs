//! # Entropy-rs
//!
//! Pure Rust entropy implementation(s)
//! * [Shannon Entropy](https://en.wiktionary.org/wiki/Shannon_entropy)
//!
//! ```edition2018
//! use entropy_rs::{Entropy, Shannon};
//! let data = vec![0,1,2,4,5];
//! assert_eq!(2.584962500721156, Shannon::quick(&data));
//! ```
//! ## Entropy
//! The `Entropy` trait defines the interaction with the implementations
//!
//! ## Practical Usage
//! For large files, the data should be read in blocks and fed through `input()`,
//! once all data is read, `calculate()` will compute and return the entropy.

mod shannon;
pub use shannon::Shannon;

/// Defines interaction with entropy algorithms
pub trait Entropy {
    /// add data to the calculation, this may be called as many times as necessary
    fn input<T: AsRef<[u8]>>(&mut self, data: T);
    /// compute and cache the entropy data. once this is called, no more data should be added
    fn calculate(&mut self) -> f64;
    /// readies the instance to compute new data
    fn reset(&mut self);
}
