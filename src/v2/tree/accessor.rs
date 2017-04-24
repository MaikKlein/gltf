
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use v2::{self, tree, Extras};

/// TODO: Add documentation.
#[derive(Clone, Debug)]
pub struct Accessor<'a, E: 'a + Extras> {
    accessor: &'a v2::accessor::Accessor<E>,
    root: &'a tree::root::Root<'a, E>,
}

/// An `Iterator` that iterates over the members of an accessor.
#[derive(Clone, Debug)]
pub struct Iter<'a, T: 'a> {
    count: usize,
    ptr: *const u8,
    stride: usize,
    _mk: std::marker::PhantomData<&'a T>,
}

impl<'a, E: 'a + Extras> Accessor<'a, E> {
    /// Reads the data pointed to by the accessor as the given type.
    /// 
    /// The data is guaranteed to be appropriately aligned for the given type.
    /// Returns `Err(())` if the given type is of incompatible size.
    pub fn iter<T>(self) -> Result<Iter<'a, T>, ()> {
        if self.accessor.component_size() != std::mem::size_of::<T>() {
            Err(())
        } else {
            let buffer_view = tree::buffer::BufferView::new(
                self.root,
                self.root.get(&self.accessor.buffer_view),
            );
            let data = buffer_view.data();
            let ptr = unsafe {
                data.as_ptr().offset(self.accessor.byte_offset as isize)
            };
            Ok(Iter {
                count: self.accessor.count as usize,
                ptr: ptr,
                stride: buffer_view.stride() as usize,
                _mk: std::marker::PhantomData,
            })
        }
    }

    pub fn kind(&self) -> v2::accessor::Kind {
        self.accessor.kind
    }

    #[doc(hidden)]
    pub fn new(
        root: &'a tree::root::Root<E>,
        accessor: &'a v2::accessor::Accessor<E>,
    ) -> Self {
        Accessor {
            accessor: accessor,
            root: root,
        }
    }

    pub fn ty(&self) -> v2::accessor::ComponentType {
        self.accessor.component_type
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
