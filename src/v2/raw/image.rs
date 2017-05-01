
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{raw, Extras, Root, Validate};
use v2::raw::root::Index;

/// Image data used to create a texture.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image<X: Extras> {
    /// The `BufferView` that contains the image if `uri` is `None`.
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<Index<raw::buffer::BufferView<X>>>,

    /// The image's MIME type.
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The uniform resource identifier of the of the image if `buffer_view` is
    /// `None`.
    ///
    /// Relative paths are relative to the .gltf file.
    ///
    /// The image format must be jpg, png, bmp, or gif.
    pub uri: Option<String>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: ImageExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Image,
}

/// Extension specific data for `Image`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ImageExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<X: Extras> Validate<X> for Image<X> {
    fn validate<W, E>(&self, root: &Root<X>, _warn: W, mut err: E)
        where W: FnMut(&str, &str), E: FnMut(&str, &str)
    {
        if let Some(ref buffer_view) = self.buffer_view {
            if let Err(_) = root.try_get(buffer_view) {
                err("bufferView", "Index out of range");
            }
        }
    }
}
