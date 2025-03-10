//! [![github]](https://github.com/dtolnay/bufsize)&ensp;[![crates-io]](https://crates.io/crates/bufsize)&ensp;[![docs-rs]](https://docs.rs/bufsize)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

#![no_std]
#![doc(html_root_url = "https://docs.rs/bufsize/1.0.10")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::let_underscore_untyped,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::new_without_default
)]

#[cfg(feature = "std")]
extern crate std;

use bytes::buf::{Buf, BufMut, UninitSlice};
#[cfg(feature = "std")]
use std::io::{self, IoSlice, Write};

/// Implementation of [`BufMut`] and [`Write`] to count the size of a resulting
/// buffer.
///
/// This effectively requires the data to be serialized twice, but in many use
/// cases inlining allows most of the effort of generating actual data to be
/// elided.
///
/// # Example: `BufMut`
///
/// ```
/// use bufsize::SizeCounter;
/// use bytes::BufMut;
///
/// pub struct DataStructure;
///
/// impl DataStructure {
///     pub fn serialize<B: BufMut>(&self, buf: &mut B) {
///         let name = "DataStructure";
///         buf.put_u8(name.len() as u8);
///         buf.put_slice(name.as_bytes());
///         buf.put_u32_le(9999);
///         buf.put_f32_le(1.0);
///     }
/// }
///
/// fn main() {
///     let mut sizecount = SizeCounter::new();
///     DataStructure.serialize(&mut sizecount);
///
///     let mut buffer = Vec::with_capacity(sizecount.size());
///     DataStructure.serialize(&mut buffer);
///
///     assert_eq!(sizecount.size(), buffer.len());
///     assert_eq!(sizecount.size(), buffer.capacity());
/// }
/// ```
///
/// # Example: `Write`
///
/// ```
/// # use anyhow::Result;
/// use bufsize::SizeCounter;
/// use serde_json::json;
///
/// fn main() -> Result<()> {
///     let value = json!({ /* ... */ });
///
///     let mut sizecount = SizeCounter::new();
///     serde_json::to_writer(&mut sizecount, &value)?;
///
///     let mut buffer = Vec::with_capacity(sizecount.size());
///     serde_json::to_writer(&mut buffer, &value)?;
///
///     assert_eq!(sizecount.size(), buffer.len());
///     assert_eq!(sizecount.size(), buffer.capacity());
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct SizeCounter {
    count: usize,
}

impl SizeCounter {
    pub fn new() -> Self {
        SizeCounter { count: 0 }
    }

    pub fn size(&self) -> usize {
        self.count
    }
}

unsafe impl BufMut for SizeCounter {
    #[inline]
    fn remaining_mut(&self) -> usize {
        usize::max_value()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let _ = cnt;
    }

    fn chunk_mut(&mut self) -> &mut UninitSlice {
        unimplemented!("SizeCounter doesn't really have a buffer")
    }

    #[inline]
    fn has_remaining_mut(&self) -> bool {
        true
    }

    fn put<T: Buf>(&mut self, src: T)
    where
        Self: Sized,
    {
        self.count += src.remaining();
    }

    #[inline]
    fn put_slice(&mut self, src: &[u8]) {
        self.count += src.len();
    }

    #[inline]
    fn put_u8(&mut self, n: u8) {
        let _ = n;
        self.count += 1;
    }

    #[inline]
    fn put_i8(&mut self, n: i8) {
        let _ = n;
        self.count += 1;
    }

    #[inline]
    fn put_u16(&mut self, n: u16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_u16_le(&mut self, n: u16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_i16(&mut self, n: i16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_i16_le(&mut self, n: i16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_u32(&mut self, n: u32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_u32_le(&mut self, n: u32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_i32(&mut self, n: i32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_i32_le(&mut self, n: i32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_u64(&mut self, n: u64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_u64_le(&mut self, n: u64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_i64(&mut self, n: i64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_i64_le(&mut self, n: i64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_u128(&mut self, n: u128) {
        let _ = n;
        self.count += 16;
    }

    #[inline]
    fn put_u128_le(&mut self, n: u128) {
        let _ = n;
        self.count += 16;
    }

    #[inline]
    fn put_i128(&mut self, n: i128) {
        let _ = n;
        self.count += 16;
    }

    #[inline]
    fn put_i128_le(&mut self, n: i128) {
        let _ = n;
        self.count += 16;
    }

    #[inline]
    fn put_uint(&mut self, n: u64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_uint_le(&mut self, n: u64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_int(&mut self, n: i64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_int_le(&mut self, n: i64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_f32(&mut self, n: f32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_f32_le(&mut self, n: f32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_f64(&mut self, n: f64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_f64_le(&mut self, n: f64) {
        let _ = n;
        self.count += 8;
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl Write for SizeCounter {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.count += buf.len();
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.count += buf.len();
        Ok(())
    }

    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        let mut sum = 0;
        for buf in bufs {
            sum += buf.len();
        }
        self.count += sum;
        Ok(sum)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
