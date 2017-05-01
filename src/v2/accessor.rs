
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::marker::PhantomData;
use v2::{buffer, raw, Extras, Root};

/// TODO: Add documentation.
#[derive(Clone, Debug)]
pub struct Accessor<'a, X: 'a + Extras> {
    raw: &'a raw::accessor::Accessor<X>,
    root: &'a Root<X>,
}

/// An `Iterator` that iterates over the members of an accessor.
#[derive(Clone, Debug)]
pub struct Iter<'a, T: 'a> {
    count: usize,
    ptr: *const u8,
    stride: usize,
    _mk: PhantomData<&'a T>,
}

impl<'a, X: 'a + Extras> Accessor<'a, X> {
    /// Interprets the data pointed to by the accessor as the given type.
    /// 
    /// The data referenced by the accessor is guaranteed to be appropriately
    /// aligned.
    ///
    /// # Panics
    ///
    /// If size_of::<T>() != component_size.
    pub unsafe fn iter<T>(self) -> Iter<'a, T> {
        assert!(self.raw.component_size() == std::mem::size_of::<T>());
        let buffer_view = buffer::BufferView::from_raw(
            self.root,
            self.root.get(&self.raw.buffer_view),
        );
        let data = buffer_view.data();
        let ptr = data.as_ptr().offset(self.raw.byte_offset as isize);
        Iter {
            count: self.raw.count as usize,
            ptr: ptr,
            stride: buffer_view.stride() as usize,
            _mk: PhantomData,
        }
    }

    pub fn kind(&self) -> raw::accessor::Kind {
        self.raw.kind
    }

    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::accessor::Accessor<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    pub fn ty(&self) -> raw::accessor::ComponentType {
        self.raw.component_type
    }
}

impl<'a, T: 'a> ExactSizeIterator for Iter<'a, T> {}
impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        use std::mem::{size_of, transmute_copy};
        if self.count > 0 {
            let value: T = unsafe { transmute_copy(&*self.ptr) };
            self.count -= 1;
            unsafe {
                if self.stride > 0 {
                    self.ptr = self.ptr.offset(self.stride as isize);
                } else {
                    self.ptr = self.ptr.offset(size_of::<T>() as isize);
                }
                Some(value)
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}
