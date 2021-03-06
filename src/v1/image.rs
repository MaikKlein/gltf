// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Image {
    /// The uri of the image.
    ///
    /// Relative paths are relative to the .gltf file. Instead of referencing an
    /// external file, the uri can also be a data-uri. The image format must be
    /// jpg, png, bmp, or gif.
    pub uri: String,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., an image and a buffer could have
    /// the same name, or two images could even have the same name.
    pub name: Option<String>, 

    // TODO: extension
    // TODO: extras
}
