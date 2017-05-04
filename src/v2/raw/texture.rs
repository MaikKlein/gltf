
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{raw, Extras, Root, Validate};
use v2::raw::root::Index;

enum_number! {
    DataType {
        U8 = 5121,
        U16R5G6B5 = 33635,
        U16R4G4B4A4 = 32819,
        U16R5G5B5A1 = 32820,
    }
}

enum_number! {
    Format {
        Alpha = 6406,
        Rgb = 6407,
        Rgba = 6408,
        Luminance = 6409,
        LuminanceAlpha = 6410,
    }
}

enum_number! {
    MagFilter {
        Nearest = 9728,
        Linear = 9729,
    }
}

enum_number! {
    MinFilter {
        Nearest = 9728,
        Linear = 9729,
        NearestMipmapNearest = 9984,
        LinearMipmapNearest = 9985,
        NearestMipmapLinear = 9986,
        LinearMipmapLinear = 9987,
    }
}

enum_number! {
    Target {
        Texture2d = 3553,
    }
}

enum_number! {
    WrappingMode {
        ClampToEdge = 33071,
        MirroredRepeat = 33648,
        Repeat = 10497,
    }
}

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sampler<X: Extras> {
    /// Magnification filter.
    #[serde(default, rename = "magFilter")]
    pub mag_filter: MagFilter,

    /// Minification filter.
    #[serde(default, rename = "minFilter")]
    pub min_filter: MinFilter,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// `s` wrapping mode.
    #[serde(default, rename = "wrapS")]
    pub wrap_s: WrappingMode,

    /// `t` wrapping mode.
    #[serde(default, rename = "wrapT")]
    pub wrap_t: WrappingMode,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: SamplerExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Sampler,
}

/// Extension specific data for `Sampler`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Texture<X: Extras> {
    /// Texel data type.
    #[serde(default, rename = "type")]
    pub data_type: DataType,

    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The texture format.
    #[serde(default)]
    pub format: Format,

    /// The texture internal format.
    #[serde(default, rename = "internalFormat")]
    pub internal_format: Format,

    /// The index of the sampler used by this texture.
    pub sampler: Index<Sampler<X>>,

    /// The index of the image used by this texture.
    pub source: Index<raw::image::Image<X>>,

    /// The target the texture should be bound to.
    #[serde(default)]
    pub target: Target,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: TextureExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Texture,
}

/// Extension specific data for `Texture`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextureExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
/// Reference to a `Texture`.
// Parent structures are expected to include validation for `index` in their own
// `Validate` implementation.
pub struct TextureInfo<X: Extras> {
    /// The index of the texture.
    pub index: Index<Texture<X>>,

    /// The set index of the texture's `TEXCOORD` attribute.
    #[serde(default, rename = "texCoord")]
    pub tex_coord: u32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: TextureInfoExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::TextureInfo,
}

/// Extension specific data for `TextureInfo`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TextureInfoExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<X: Extras> Validate<X> for Texture<X> {
    fn validate<W, E>(&self, root: &Root<X>, _warn: W, mut err: E)
        where W: FnMut(&str, &str), E: FnMut(&str, &str)
    {
        if let Err(_) = root.try_get(&self.sampler) {
            err("sampler", "Index out of range");
        }
        if let Err(_) = root.try_get(&self.source) {
            err("source", "Index out of range");
        }
    }
}

impl Default for DataType {
    fn default() -> Self {
        DataType::U8
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Rgba
    }
}

impl Default for MagFilter {
    fn default() -> Self {
        MagFilter::Linear
    }
}

impl Default for MinFilter {
    fn default() -> Self {
        MinFilter::NearestMipmapLinear
    }
}

impl Default for Target {
    fn default() -> Self {
        Target::Texture2d
    }
}

impl Default for WrappingMode {
    fn default() -> Self {
        WrappingMode::Repeat
    }
}
