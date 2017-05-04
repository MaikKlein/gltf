
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{raw, Extras, Root};

#[derive(Debug)]
pub struct Animation<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::animation::Animation<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

impl<'a, X: 'a + Extras> Animation<'a, X> {
    /// Constructor for an `Animation`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::animation::Animation<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }
}

