# squares-rnd

![Rust](https://github.com/DoumanAsh/squares-rnd/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/squares-rnd.svg)](https://crates.io/crates/squares-rnd)
[![Documentation](https://docs.rs/squares-rnd/badge.svg)](https://docs.rs/crate/squares-rnd/)

Simple and fast counter based non-crypto random generator.

The algorithm is based on `Middle Square Weyl Sequence RNG`.
See [paper](https://arxiv.org/abs/2004.06278v7) for details.

**NOTE**: Not cryptographically secure.

There are several note-worthy properties to the algorithm:

- State is represented by counter, which is incremented to produce new value, hence making
it easy to predict how state would change.
- The code is short and simple, only taking minimum amount of operations to produce uniform output.
- `key` must have close to equal number of zeroes and ones for optimal output.
This crate provides single key for use, to have more download key file [gist](https://gist.githubusercontent.com/DoumanAsh/a57bc65434702d5d7fb88343c65f3145/raw/a9b45f7155c483f689318ee501222e72be0d66ec/keys)
