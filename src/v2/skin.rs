
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::slice::Iter as SliceIter;
use v2::{accessor, raw, scene, Extras, Root};
use self::accessor::Accessor;
use self::raw::root::Index;
use self::scene::Node;

#[derive(Clone, Debug)]
pub struct IterJoints<'a, X: 'a + Extras> {
    /// The internal iterator.
    iter: SliceIter<'a, Index<raw::scene::Node<X>>>,

    /// The root glTF object.
    root: &'a Root<X>,
}

#[derive(Clone, Debug)]
pub struct Skin<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::skin::Skin<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

impl<'a, X: 'a + Extras> Skin<'a, X> {
    /// Constructor for a `Skin`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::skin::Skin<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    /// Returns the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`,each matrix is assumed to be the 4x4 identity matrix
    /// which implies that the inverse-bind matrices were pre-applied.
    pub fn inverse_bind_matrices(&self) -> Option<Accessor<'a, X>> {
        self.raw.inverse_bind_matrices.as_ref().map(|index| {
            self.root.iter_accessors().nth(index.value() as usize).unwrap()
        })
    }

    /// Returns an `Iterator` that visits the joints of the skin.
    pub fn iter_joints(&self) -> IterJoints<'a, X> {
        IterJoints {
            iter: self.raw.joints.iter(),
            root: self.root,
        }
    }

    /// Returns the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    pub fn skeleton(&self) -> Option<Node<'a, X>> {
        self.raw.skeleton.as_ref().map(|index| {
            self.root.iter_nodes().nth(index.value() as usize).unwrap()
        })
    }
}

impl<'a, X: 'a + Extras> Iterator for IterJoints<'a, X> {
    type Item = Node<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|index| {
            self.root.iter_nodes().nth(index.value() as usize).unwrap()
        })
    }
}

