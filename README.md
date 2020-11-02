# squares-rnd

![Rust](https://github.com/DoumanAsh/uuid/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/squares-rnd.svg)](https://crates.io/crates/squares-rnd)
[![Documentation](https://docs.rs/squares-rnd/badge.svg)](https://docs.rs/crate/squares-rnd/)

Simple and fast counter based non-crypto random generator.

The algorithm is based on `Middle Square Weyl Sequence RNG`.
See [paper](https://arxiv.org/abs/2004.06278) for details.

**NOTE**: Not cryptographically secure.

There are several note-worthy properties to the algorithm:

- State is represented by counter, which is incremented to produce new value, hence making
it easy to predict how state would change.
- The code is short and simple, only taking minimum amount of operations to produce uniform output.
- `key` must have close to equal number of zeroes and ones for optimal output.
This crate provides single key for use, to have more see [gist](https://gist.githubusercontent.com/DoumanAsh/6e2b862242b7863c5119320ed5dae863/raw/2d17fd5937f158b62b8acdb4f5d590e310331b16/keys)
