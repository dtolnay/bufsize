bufsize::SizeCounter
====================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/bufsize-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/bufsize)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bufsize.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bufsize)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bufsize-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/bufsize)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/bufsize/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/bufsize/actions?query=branch%3Amaster)

This library provides an implementation of [`bytes::BufMut`] to count the size
of a resulting buffer.

[`bytes::BufMut`]: https://docs.rs/bytes/0.4/bytes/trait.BufMut.html

```toml
[dependencies]
bufsize = "1.0"
```

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
