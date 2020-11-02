//!Simple and fast counter based non-crypto random generator.
//!
//!The algorithm is based on `Middle Square Weyl Sequence RNG`.
//!See [paper](https://arxiv.org/abs/2004.06278) for details.
//!
//!**NOTE**: Not cryptographically secure.
//!
//!There are several note-worthy properties to the algorithm:
//!
//!- State is represented by counter, which is incremented to produce new value, hence making
//!it easy to predict how state would change.
//!- The code is short and simple, only taking minimum amount of operations to produce uniform output.
//!- `key` must have close to equal number of zeroes and ones for optimal output.
//!This crate provides single key for use, to have more see [gist](https://gist.githubusercontent.com/DoumanAsh/6e2b862242b7863c5119320ed5dae863/raw/2d17fd5937f158b62b8acdb4f5d590e310331b16/keys)

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

///Sample key to be used with algorithm
pub const KEY: u64 = 0x548c9decbce65297;

#[inline]
///Generates random number.
///
///- `counter` - Integer counter which acts as state. Should be increased to generate new
///number.
///- `key` - Integer which in general should be irregular bit pattern with approximately equal
///number of zeros and ones. Generally should be constant, but can be changed when new range of
///random numbers is required.
pub const fn rand(counter: u64, key: u64) -> u32 {
    let mut x = counter.wrapping_mul(key);
    let y = x;
    let z = y.wrapping_add(key);

    x = x.wrapping_mul(x).wrapping_add(y);
    x = (x >> 32) | (x << 32);

    x = x.wrapping_mul(x).wrapping_add(z);
    x = (x >> 32) | (x << 32);

    (x.wrapping_mul(x).wrapping_add(y) >> 32) as u32
}

#[derive(Debug)]
///Stateful representation of algorithm.
///
///Increments counter on each generation using the same key.
pub struct Rand {
    counter: AtomicU64,
    key: u64,
}

impl Rand {
    #[inline]
    ///Creates new instance with provided key.
    pub const fn new(key: u64) -> Self {
        Self {
            counter: AtomicU64::new(0),
            key,
        }
    }

    #[inline]
    ///Sets new counter value
    pub const fn set_counter(self, counter: u64) -> Self {
        Self {
            counter: AtomicU64::new(counter),
            key: self.key
        }
    }

    #[inline]
    ///Gets current value of counter
    pub fn counter(&self) -> u64 {
        self.counter.load(Ordering::Acquire)
    }

    #[inline]
    pub fn next_u32(&self) -> u32 {
        rand(self.counter.fetch_add(1, Ordering::AcqRel), self.key)
    }

    #[inline]
    pub fn next_u64(&self) -> u64 {
        ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
    }
}
