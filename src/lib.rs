//! A Rust library for calculation of the Boys function
//!
//! \\[ F_{n}(x) = \int_{0}^{1} t^{2n} e^{-xt^{2}} \mathop{dt} \\]
//!
//! via a number of methods:
//!
//! - An "exact" algorithm originally from [Radovan Bast](https://github.com/bast/obara-saika/blob/52ff0fa4d470eeb7a41d932078b2e4d9a54d05e5/obara_saika.py#L371).
//! - A non-clean-room reimplementation of [Michael Böhme's algorithm](https://github.com/micb25/libboys).
//!
//! ## Dependencies
//!
//! Special functions are provided by the pure Rust
//! [`gsl-oxide-special`](https://crates.io/crates/gsl-oxide-special) crate; a
//! system GSL installation is not required.
//!
//! ## Literature
//!
//! [S. F. Boys, *Proc R. Soc. Am.* **1950**, *200*, 542.](https://doi.org/10.1098/rspa.1950.0036)

#![allow(clippy::many_single_char_names)]

/// An exact implementation using the gamma and incomplete gamma functions.
pub mod exact;
/// These are disabled until their correctness is confirmed.
// mod jeffhammond;
/// TODO
pub mod micb25;
