
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to tho2se terms.

use v2::{raw, texture, Extras, Root};
use self::texture::{Texture, TextureInfo};

pub use self::raw::material::AlphaMode;

/// The material appearance of a primitive.
#[derive(Clone, Debug)]
pub struct Material<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::material::Material<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug)]
pub struct NormalTexture<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::material::NormalTexture<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug)]
pub struct OcclusionTexture<'a, X: 'a + Extras> {
    /// The internal glTF object data.
    raw: &'a raw::material::OcclusionTexture<X>,

    /// The root glTF object.
    root: &'a Root<X>,
}

impl<'a, X: 'a + Extras> Material<'a, X> {
    /// Constructor for a `Material`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::material::Material<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    /// The base color texture.
    ///
    /// This texture contains RGB(A) components in sRGB color space.
    ///
    /// * The first three components (RGB) specify the base color of the material
    /// * If the fourth component (A) is present, it represents the alpha
    ///   coverage of the material and otherwise, an alpha of 1.0 is assumed
    /// * The `alpha_mode` field specifies how alpha is interpreted.
    ///
    /// The stored texels must not be premultiplied.
    pub fn base_color_texture(&self) -> TextureInfo<'a, X> {
        TextureInfo::from_raw(
            self.root,
            &self.raw.pbr_metallic_roughness.base_color_texture,
        )
    }

    /// The emissive map texture.
    ///
    /// The emissive map controls the color and intensity of the light being
    /// emitted by the material.
    ///
    /// This texture contains RGB components in sRGB color space.
    ///
    /// If a fourth component (A) is present, it is ignored.
    pub fn emissive_texture(&self) -> Option<TextureInfo<'a, X>> {
        self.raw.emissive_texture.as_ref().map(|raw| {
            TextureInfo::from_raw(self.root, raw)
        })
    }

    /// The metallic-roughness texture.
    ///
    /// This texture has two components:
    ///
    /// * The first component (R) contains the metallic-ness of the material.
    /// * The second component (G) contains the roughness of the material.
    /// * If the third component (B) and/or the fourth component (A) are present
    ///   then they are ignored.
    pub fn metallic_roughness_texture(&self) -> TextureInfo<'a, X> {
        TextureInfo::from_raw(
            self.root,
            &self.raw.pbr_metallic_roughness.metallic_roughness_texture,
        )
    }

    /// A tangent space normal map.
    ///
    /// Each texel represents the XYZ components of a normal vector in tangent
    /// space.
    pub fn normal_texture(&self) -> Option<NormalTexture<'a, X>> {
        self.raw.normal_texture.as_ref().map(|raw| {
            NormalTexture::from_raw(self.root, raw)
        })
    }

    /// The occlusion map texture.
    ///
    /// The occlusion map is a greyscale texture, with white indicating areas that
    /// should receive full indirect lighting and black indicating no indirect
    /// lighting.
    pub fn occlusion_texture(&self) -> Option<OcclusionTexture<'a, X>> {
        self.raw.occlusion_texture.as_ref().map(|raw| {
            OcclusionTexture::from_raw(self.root, raw)
        })
    }
}

impl<'a, X: 'a + Extras> NormalTexture<'a, X> {
    /// Constructor for a `NormalTexture`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::material::NormalTexture<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    /// Retrieves the referenced texture.
    pub fn texture(&self) -> Texture<'a, X> {
        self.root.iter_textures().nth(self.raw.index.value() as usize).unwrap()
    }
}
    
impl<'a, X: 'a + Extras> OcclusionTexture<'a, X> {
    /// Constructor for an `OcclusionTexture`.
    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::material::OcclusionTexture<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }

    /// Retrieves the referenced texture.
    pub fn texture(&self) -> Texture<'a, X> {
        self.root.iter_textures().nth(self.raw.index.value() as usize).unwrap()
    }
}
