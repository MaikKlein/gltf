
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{raw, Extras, Root};

/// Image data used to create a texture.
#[derive(Debug)]
pub struct Image<'a, X: 'a + Extras> {
    /// Contains the pre-loaded image data this `Image` describes.
    data: &'a [u8],

    /// The internal glTF object data.
    raw: &'a raw::image::Image<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

impl<'a, X: 'a + Extras> Image<'a, X> {
    /// Constructor for a `Image`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::image::Image<X>,
        data: &'a [u8],
    ) -> Self {
        Self {
            data: data,
            raw: raw,
            root: root,
        }
    }
}

