bufsize::SizeCounter
====================

[![Build Status](https://api.travis-ci.com/dtolnay/bufsize.svg?branch=master)](https://travis-ci.com/dtolnay/bufsize)
[![Latest Version](https://img.shields.io/crates/v/bufsize.svg)](https://crates.io/crates/bufsize)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/bufsize)

This library provides an implementation of [`bytes::BufMut`] to count the size
of a resulting buffer.

[`bytes::BufMut`]: https://docs.rs/bytes/0.4/bytes/trait.BufMut.html

```toml
[dependencies]
bufsize = "0.4"
```

*Compiler support: requires rustc 1.31+*

<br>

## Example

```rust
use bufsize::SizeCounter;
use bytes::BufMut;

pub struct DataStructure;

impl DataStructure {
    pub fn serialize<B: BufMut>(&self, buf: &mut B) {
        let name = "DataStructure";
        buf.put_u8(name.len() as u8);
        buf.put_slice(name.as_bytes());
        buf.put_u32_le(9999);
        buf.put_f32_le(1.0);
    }
}

fn main() {
    let mut sizecount = SizeCounter::new();
    DataStructure.serialize(&mut sizecount);

    let mut buffer = Vec::with_capacity(sizecount.size());
    DataStructure.serialize(&mut buffer);

    assert_eq!(sizecount.size(), buffer.len());
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
