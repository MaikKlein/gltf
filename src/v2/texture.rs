
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to tho2se terms.

use v2::{image, raw, Extras, Root};

pub use self::raw::texture::{
    DataType,
    Format,
    MagFilter,
    MinFilter,
    SamplerExtensions,
    Target,
    TextureExtensions,
    WrappingMode,
};

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug)]
pub struct Sampler<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::texture::Sampler<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

#[derive(Clone, Debug)]
pub struct Texture<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::texture::Texture<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

/// Reference to a `Texture`.
#[derive(Clone, Debug)]
pub struct TextureInfo<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::texture::TextureInfo<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

impl<'a, X: 'a + Extras> Sampler<'a, X> {
    /// Constructor for a `Sampler`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::texture::Sampler<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }
}

impl<'a, X: 'a + Extras> Texture<'a, X> {
    /// Constructor for a `Texture`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::texture::Texture<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    /// The sampler used by this texture.
    pub fn sampler(&self) -> Sampler<X> {
        self.root.iter_samplers().nth(self.raw.sampler.value() as usize).unwrap()
    }

    /// The image used by this texture.
    pub fn source(&self) -> image::Image<'a, X> {
        self.root.iter_images().nth(self.raw.source.value() as usize).unwrap()
    }
}

impl<'a, X: 'a + Extras> TextureInfo<'a, X> {
    /// Constructor for a `TextureInfo`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::texture::TextureInfo<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    /// Retrieves the referenced `Texture`.
    pub fn texture(&self) -> Texture<'a, X> {
        self.root.iter_textures().nth(self.raw.index.value() as usize).unwrap()
    }
}

