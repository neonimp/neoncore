//! This crate provides a set of shared utilities for
//! working with binary streams in memory or otherwise.

pub mod const_fn;
#[cfg(feature = "std")]
pub mod streams;
