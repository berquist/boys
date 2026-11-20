[![Build Status](https://travis-ci.com/berquist/boys.svg?branch=master)](https://travis-ci.com/berquist/boys)
[![License](https://img.shields.io/github/license/berquist/boys.svg)](LICENSE)
[![Issues](https://img.shields.io/github/issues/berquist/boys.svg)](https://github.com/berquist/boys/issues)
[![crates.io](https://img.shields.io/crates/v/boys.svg)](https://crates.io/crates/boys)
[![Docs.rs](https://docs.rs/boys/badge.svg)](https://docs.rs/boys)

# boys

A Rust library for calculation of the Boys function

![equation](http://latex.codecogs.com/gif.latex?F_n%28x%29%3D%5Cint_0%5E1t%5E%7B2n%7De%5E%7B-xt%5E2%7Ddt)

via a number of methods:

- An "exact" algorithm originally from [Radovan Bast](https://github.com/bast/obara-saika/blob/52ff0fa4d470eeb7a41d932078b2e4d9a54d05e5/obara_saika.py#L371).
- A non-clean-room reimplementation of [Michael Böhme's algorithm](https://github.com/micb25/libboys).

## Literature

[S. F. Boys, *Proc R. Soc. Am.* **1950**, *200*, 542.](https://doi.org/10.1098/rspa.1950.0036)
