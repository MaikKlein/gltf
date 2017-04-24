
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use v2::{self, tree, Extras};

/// Error encountered when pre-loading buffer data.
#[derive(Debug)]
pub enum PreloadError {
    /// Standard input / output error encountered when reading buffer data.
    Io(std::io::Error),

    /// Out of memory.
    Oom,
}

/// A fixed size 4-byte aligned byte buffer.
struct AlignedByteBuffer {
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
pub struct PreloadedBuffer<'a, E: 'a + Extras> {
    /// The internal glTF object data.
    buffer: &'a v2::buffer::Buffer<E>,

    /// The buffer data.
    buffer_data: AlignedByteBuffer,
}

#[derive(Debug)]
pub struct BufferView<'a, E: 'a + Extras> {
    /// The internal glTF object data.
    buffer_view: &'a v2::buffer::BufferView<E>,

    /// The root glTF object.
    root: &'a tree::root::Root<'a, E>,
}

/// Creates a `Buffer` wrapper and pre-loads the data it references.
#[doc(hidden)]
pub fn preload<'a, E: 'a + Extras>(
    buffer: &'a v2::buffer::Buffer<E>,
    gltf_path: &'a std::path::Path,
) -> Result<PreloadedBuffer<'a, E>, PreloadError> {
    use self::PreloadError::*;
    use std::io::Read;

    let path = gltf_path.with_file_name(&buffer.uri);
    let mut file = std::fs::File::open(path).map_err(Io)?;
    let mut dest = unsafe {
        AlignedByteBuffer::uninitialized(buffer.byte_length as usize)
    };
    file.read_exact(&mut dest).map_err(Io)?;
    
    Ok(PreloadedBuffer {
        buffer: buffer,
        buffer_data: dest,
    })
}

impl AlignedByteBuffer {
    /// Creates an uninitialized fixed sized 4-byte aligned byte buffer.
    pub unsafe fn uninitialized(len: usize) -> Self {
        let n_padding_bytes = 3;
        let mut data = Vec::with_capacity(len + n_padding_bytes);
        data.set_len(len + n_padding_bytes);
        let offset = offset_of_nearest_alignment_boundary(data.as_ptr());
        AlignedByteBuffer {
            data: data,
            len: len,
            offset: offset,
        }
    }
}

impl<'a, E: 'a + Extras> PreloadedBuffer<'a, E> {
    /// Returns the pre-loaded buffer data
    pub fn data(&'a self) -> &'a [u8] {
        &self.buffer_data
    }
}

impl<'a, E: 'a + Extras> BufferView<'a, E> {
    /// Returns the buffer this buffer view points to.
    pub fn buffer(&'a self) -> &'a PreloadedBuffer<'a, E> {
        self.root.preloaded_buffer(&self.buffer_view.buffer)
    }

    /// Returns a slice of the pre-loaded buffer data.
    ///
    /// Note that this is not the same as calling `buffer().data()`, which returns the
    /// entire contents of the parent buffer.
    pub fn data(&'a self) -> &'a [u8] {
        let begin = self.buffer_view.byte_offset as usize;
        let end = begin + self.buffer_view.byte_length as usize;
        &self.buffer().data()[begin..end]
    }

    #[doc(hidden)]
    pub fn new(
        root: &'a tree::root::Root<'a, E>,
        buffer_view: &'a v2::buffer::BufferView<E>,
    ) -> Self {
        BufferView {
            buffer_view: buffer_view,
            root: root,
        }
    }

    /// The stride in bytes between vertex attributes or other interleavable data.
    ///
    /// When zero, data is assumed to be tightly packed.
    pub fn stride(&self) -> u32 {
        self.buffer_view.byte_stride
    }
}

impl std::fmt::Debug for AlignedByteBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "AlignedByteBuffer {{ offset: {}, data: Vec<u8> }}",
            self.offset,
        )
    }
}

impl std::ops::Deref for AlignedByteBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        let begin = self.offset;
        let end = begin + self.len;
        &self.data[begin..end]
    }
}

impl std::ops::DerefMut for AlignedByteBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let begin = self.offset;
        let end = begin + self.len;
        &mut self.data[begin..end]
    }
}
