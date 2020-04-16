//! A Rust implementation of the MetroHash algorithm.
//!
//! MetroHash is a high quality, high performance hash algorithm
//!
//! # Example
//!
//! ```rust
//! use metrohash::MetroHashMap;
//!
//! let mut hash = MetroHashMap::default();
//! hash.insert(1000, "1000");
//! assert_eq!(hash.get(&1000), Some(&"1000"));
//! ```
#![no_std]

extern crate hashbrown;

use hashbrown::{HashMap, HashSet};
use core::hash::BuildHasherDefault;

mod metrohash64;
mod metrohash128;
mod utils;

pub use metrohash64::*;
pub use metrohash128::*;

pub type MetroHash = MetroHash64;

/// A builder for MetroHash.
pub type MetroBuildHasher = BuildHasherDefault<MetroHash>;

/// A `HashMap` using a default MetroHash.
pub type MetroHashMap<K, V> = HashMap<K, V, MetroBuildHasher>;

/// A `HashSet` using a default MetroHash.
pub type MetroHashSet<V> = HashSet<V, MetroBuildHasher>;


#[cfg(test)]
mod tests;
