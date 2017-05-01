
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use v2::{raw, Extras, Root};

/// A fixed size 4-byte aligned byte buffer.
#[derive(Debug)]
pub struct AlignedBufferData {
    /// Owned data.
    data: Vec<u8>,

    /// Length of the buffer in bytes
    len: usize,

    /// Byte offset where the data starts.
    offset: usize,
}

/// Returns the number of bytes between a pointer address and the address of the
/// nearest 4-byte alignment boundary ahead.
fn offset_of_nearest_alignment_boundary(address: *const u8) -> usize {
    [0, 3, 2, 1][address as usize % 4]
}

#[derive(Debug)]
pub struct Buffer<'a, X: 'a + Extras> {
    /// Contains the pre-loaded buffer data this `Buffer` describes.
    data: &'a AlignedBufferData,
    
    /// The internal glTF object data.
    raw: &'a raw::buffer::Buffer<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

#[derive(Debug)]
pub struct BufferView<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::buffer::BufferView<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

impl<'a, X: 'a + Extras> Buffer<'a, X> {
    /// Constructor for a `Buffer`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::buffer::Buffer<X>,
        data: &'a AlignedBufferData,
    ) -> Self {
        Self {
            data: data,
            raw: raw,
            root: root,
        }
    }

    /// Returns the entire contents of the pre-loaded buffer data this `Buffer`
    /// describes.
    pub fn data(&mut self) -> &[u8] {
        &self.data[..]
    }
}

impl AlignedBufferData {
    /// Creates an uninitialized fixed sized 4-byte aligned byte buffer.
    pub unsafe fn uninitialized(len: usize) -> Self {
        let n_padding_bytes = 3;
        let mut data = Vec::with_capacity(len + n_padding_bytes);
        data.set_len(len + n_padding_bytes);
        let offset = offset_of_nearest_alignment_boundary(data.as_ptr());
        AlignedBufferData {
            data: data,
            len: len,
            offset: offset,
        }
    }
}

impl<'a, X: 'a + Extras> BufferView<'a, X> {
    /// Returns the buffer this buffer view points to.
    pub fn buffer(&'a self) -> Buffer<'a, X> {
        Buffer::from_raw(
            &self.root,
            self.root.get(&self.raw.buffer),
            self.root.buffer_data(&self.raw.buffer),
        )
    }

    /// Returns a slice of the pre-loaded buffer data.
    ///
    /// Note that this is not the same as calling `buffer().data()`,
    /// which would instead return the entire contents of the parent buffer.
    pub fn data(&'a self) -> &'a [u8] {
        let begin = self.raw.byte_offset as usize;
        let end = begin + self.raw.byte_length as usize;
        let buffer_data = self.root.buffer_data(&self.raw.buffer);
        &buffer_data[begin..end]
    }

    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::buffer::BufferView<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    pub fn stride(&self) -> u32 {
        self.raw.byte_stride
    }
}

impl std::ops::Deref for AlignedBufferData {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        let begin = self.offset;
        let end = begin + self.len;
        &self.data[begin..end]
    }
}

impl std::ops::DerefMut for AlignedBufferData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let begin = self.offset;
        let end = begin + self.len;
        &mut self.data[begin..end]
    }
}
