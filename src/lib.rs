#![doc(html_root_url = "https://docs.rs/bufsize/0.0.0")]
#![allow(clippy::needless_doctest_main, clippy::new_without_default)]

use bytes::{Buf, BufMut, IntoBuf};

/// Implementation of [`BufMut`] to count the size of a resulting buffer.
///
/// This effectively requires the data to be serialized twice, but in many use
/// cases inlining allows most of the effort of generating actual data to be
/// elided.
///
/// # Example
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

impl BufMut for SizeCounter {
    #[inline]
    fn remaining_mut(&self) -> usize {
        usize::max_value()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let _ = cnt;
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        unimplemented!("SizeCounter doesn't really have a buffer")
    }

    #[inline]
    fn has_remaining_mut(&self) -> bool {
        true
    }

    fn put<T: IntoBuf>(&mut self, src: T)
    where
        Self: Sized,
    {
        self.count += src.into_buf().remaining();
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
    fn put_u16_be(&mut self, n: u16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_u16_le(&mut self, n: u16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_i16_be(&mut self, n: i16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_i16_le(&mut self, n: i16) {
        let _ = n;
        self.count += 2;
    }

    #[inline]
    fn put_u32_be(&mut self, n: u32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_u32_le(&mut self, n: u32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_i32_be(&mut self, n: i32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_i32_le(&mut self, n: i32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_u64_be(&mut self, n: u64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_u64_le(&mut self, n: u64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_i64_be(&mut self, n: i64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_i64_le(&mut self, n: i64) {
        let _ = n;
        self.count += 8;
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn put_u128_be(&mut self, n: u128) {
        let _ = n;
        self.count += 16;
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn put_u128_le(&mut self, n: u128) {
        let _ = n;
        self.count += 16;
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn put_i128_be(&mut self, n: i128) {
        let _ = n;
        self.count += 16;
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn put_i128_le(&mut self, n: i128) {
        let _ = n;
        self.count += 16;
    }

    #[inline]
    fn put_uint_be(&mut self, n: u64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_uint_le(&mut self, n: u64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_int_be(&mut self, n: i64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_int_le(&mut self, n: i64, nbytes: usize) {
        let _ = n;
        self.count += nbytes;
    }

    #[inline]
    fn put_f32_be(&mut self, n: f32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_f32_le(&mut self, n: f32) {
        let _ = n;
        self.count += 4;
    }

    #[inline]
    fn put_f64_be(&mut self, n: f64) {
        let _ = n;
        self.count += 8;
    }

    #[inline]
    fn put_f64_le(&mut self, n: f64) {
        let _ = n;
        self.count += 8;
    }
}
