//!Simple and fast counter based non-crypto random generator.
//!
//!The algorithm is based on `Middle Square Weyl Sequence RNG`.
//!See [paper](https://arxiv.org/abs/2004.06278v7) for details.
//!
//!**NOTE**: Not cryptographically secure.
//!
//!There are several note-worthy properties to the algorithm:
//!
//!- State is represented by counter, which is incremented to produce new value, hence making
//!it easy to predict how state would change.
//!- The code is short and simple, only taking minimum amount of operations to produce uniform output.
//!- `key` must have close to equal number of zeroes and ones for optimal output.
//!This crate provides single key for use, to have more download key file [gist](https://gist.githubusercontent.com/DoumanAsh/a57bc65434702d5d7fb88343c65f3145/raw/a9b45f7155c483f689318ee501222e72be0d66ec/keys)

#![no_std]
#![warn(missing_docs)]

use core::sync::atomic::{AtomicU64, Ordering};

///Default key to be used with algorithm
pub const KEY: u64 = 0x5d8491e219f6537d;

#[inline]
///Generates random `u32`
///
///- `counter` - Integer counter which acts as state. Should be increased to generate new
///number.
///- `key` - Integer which in general should be irregular bit pattern with approximately equal
///number of zeros and ones. Generally should be constant, but can be changed when new range of
///random numbers is required.
pub const fn rand32(counter: u64, key: u64) -> u32 {
    let mut x = counter.wrapping_mul(key);
    let y = x;
    let z = y.wrapping_add(key);

    x = x.wrapping_mul(x).wrapping_add(y);
    x = (x >> 32) | (x << 32);

    x = x.wrapping_mul(x).wrapping_add(z);
    x = (x >> 32) | (x << 32);

    x = x.wrapping_mul(x).wrapping_add(y);
    x = (x >> 32) | (x << 32);

    (x.wrapping_mul(x).wrapping_add(z) >> 32) as u32
}

#[inline]
///Generates random `u64`
///
///- `counter` - Integer counter which acts as state. Should be increased to generate new
///number.
///- `key` - Integer which in general should be irregular bit pattern with approximately equal
///number of zeros and ones. Generally should be constant, but can be changed when new range of
///random numbers is required.
pub const fn rand64(counter: u64, key: u64) -> u64 {
    let mut x = counter.wrapping_mul(key);
    let y = x;
    let z = y.wrapping_add(key);

    x = x.wrapping_mul(x).wrapping_add(y);
    x = (x >> 32) | (x << 32);

    x = x.wrapping_mul(x).wrapping_add(z);
    x = (x >> 32) | (x << 32);

    x = x.wrapping_mul(x).wrapping_add(y);
    x = (x >> 32) | (x << 32);

    x = x.wrapping_mul(x).wrapping_add(z);
    let t = x;
    x = (x >> 32) | (x << 32);

    t ^ (x.wrapping_mul(x).wrapping_add(y) >> 32)
}


///Full rand result
pub struct RandRes<T> {
    ///Counter, used to generate `value`
    pub counter: u64,
    ///Value generated with `counter.
    pub value: T
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
    #[inline(always)]
    ///Creates new instance with provided key.
    pub const fn new(key: u64) -> Self {
        Self::with_counter(0, key)
    }

    #[inline]
    ///Creates new instance with provided key and initial value of counter.
    pub const fn with_counter(counter: u64, key: u64) -> Self {
        Self {
            counter: AtomicU64::new(counter),
            key,
        }
    }

    #[inline]
    ///Sets new counter value, returning old one
    pub fn set_counter(&self, counter: u64) -> u64 {
        self.counter.swap(counter, Ordering::AcqRel)
    }

    #[inline]
    ///Gets current value of counter
    pub fn counter(&self) -> u64 {
        self.counter.load(Ordering::Acquire)
    }

    #[inline]
    ///Generates new `u32` together with corresponding counter value
    pub fn next_full_u32(&self) -> RandRes<u32> {
        let counter = self.counter.fetch_add(1, Ordering::AcqRel);
        RandRes {
            counter,
            value: rand32(counter, self.key)
        }
    }

    #[inline]
    ///Generates new `u32`
    pub fn next_u32(&self) -> u32 {
        rand32(self.counter.fetch_add(1, Ordering::AcqRel), self.key)
    }

    #[inline]
    ///Generates new `u32` in range `0..to`
    pub fn next_u32_up(&self, to: u32) -> u32 {
        #[inline(always)]
        fn mul_high_u32(a: u32, b: u32) -> u32 {
            (((a as u64) * (b as u64)) >> 32) as u32
        }

        //https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut result = self.next_u32();
        let mut hi = mul_high_u32(result, to);
        let mut lo = result.wrapping_mul(to);

        if lo < to {
            while lo < (to.wrapping_neg() % to) {
                result = self.next_u32();
                hi = mul_high_u32(result, to);
                lo = result.wrapping_mul(to);
            }
        }

        hi
    }

    #[inline]
    ///Generates new `u64` together with corresponding counter value
    pub fn next_full_u64(&self) -> RandRes<u64> {
        let counter = self.counter.fetch_add(1, Ordering::AcqRel);
        RandRes {
            counter,
            value: rand64(counter, self.key)
        }
    }

    #[inline]
    ///Generates new `u64`
    pub fn next_u64(&self) -> u64 {
        rand64(self.counter.fetch_add(1, Ordering::AcqRel), self.key)
    }

    #[inline]
    ///Generates new `u64` in range `0..to`
    pub fn next_u64_up(&self, to: u64) -> u64 {
        #[inline(always)]
        fn mul_high_u64(a: u64, b: u64) -> u64 {
            (((a as u128) * (b as u128)) >> 64) as u64
        }

        //https://lemire.me/blog/2016/06/30/fast-random-shuffling/
        let mut result = self.next_u64();
        let mut hi = mul_high_u64(result, to);
        let mut lo = result.wrapping_mul(to);

        if lo < to {
            while lo < (to.wrapping_neg() % to) {
                result = self.next_u64();
                hi = mul_high_u64(result, to);
                lo = result.wrapping_mul(to);
            }
        }

        hi
    }
}

impl Default for Rand {
    #[inline(always)]
    fn default() -> Self {
        Self::new(KEY)
    }
}
