
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::marker::PhantomData;
use v2::Extras;
use v2::raw::*;

/// Helper trait for retrieving top-level objects by a universal identifier.
pub trait Get<T> {
    /// Retrieves a single value at the given index.
    fn get(&self, id: &Index<T>) -> &T;
}

/// Helper trait for attempting to retrieve top-level objects by a universal
/// identifier.
pub trait TryGet<T> {
    /// Attempts to retrieve a single value at the given index.
    fn try_get(&self, id: &Index<T>) -> Result<&T, ()>;
}

/// Represents an offset into an array of type `T` owned by the root glTF object.
#[derive(Clone, Copy, Debug)]
pub struct Index<T>(u32, PhantomData<T>);

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Root<X: Extras> {
    #[serde(default)]
    pub accessors: Vec<accessor::Accessor<X>>,
    
    #[serde(default)]
    pub animations: Vec<animation::Animation<X>>,

    /// Metadata about the glTF asset.
    pub asset: asset::Asset<X>,
    
    #[serde(default)]
    pub buffers: Vec<buffer::Buffer<X>>,
    
    #[serde(default, rename = "bufferViews")]
    pub buffer_views: Vec<buffer::BufferView<X>>,

    /// The default scene.
    #[serde(default = "root_scene_default", rename = "scene")]
    pub default_scene: Index<scene::Scene<X>>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: RootExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Root,
    
    /// Names of glTF extensions used somewhere in this asset.
    #[serde(default, rename = "extensionsUsed")]
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(default, rename = "extensionsRequired")]
    pub extensions_required: Vec<String>,
    
    #[serde(default)]
    pub cameras: Vec<camera::Camera<X>>,
    
    #[serde(default)]
    pub images: Vec<image::Image<X>>,
    
    #[serde(default)]
    pub materials: Vec<material::Material<X>>,
    
    #[serde(default)]
    pub meshes: Vec<mesh::Mesh<X>>,
    
    #[serde(default)]
    pub nodes: Vec<scene::Node<X>>,
    
    #[serde(default)]
    pub samplers: Vec<texture::Sampler<X>>,
    
    #[serde(default)]
    pub scenes: Vec<scene::Scene<X>>,
    
    #[serde(default)]
    pub skins: Vec<skin::Skin<X>>,
    
    #[serde(default)]
    pub textures: Vec<texture::Texture<X>>,
}

fn root_scene_default<X: Extras>() -> Index<scene::Scene<X>> {
    Index::new(0)
}

/// Extension specific data for `Root`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RootExtensions {
    _allow_extra_fields: (),
}

impl<T> Index<T> {
    /// Creates a new `Index` representing an offset into an array containing `T`.
    fn new(value: u32) -> Self {
        Index(value, PhantomData)
    }

    /// Returns the internal offset value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl<T> serde::Serialize for Index<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        serializer.serialize_u64(self.value() as u64)
    }
}

impl<T> serde::Deserialize for Index<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        struct Visitor<T>(PhantomData<T>);
        impl<T> serde::de::Visitor for Visitor<T> {
            type Value = Index<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GLenum")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                Ok(Index::new(value as u32))
            }
        }
        deserializer.deserialize_u64(Visitor::<T>(PhantomData))
    }
}

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<X: Extras> Get<$ty> for Root<X> {
            fn get(&self, index: &Index<$ty>) -> &$ty {
                &self.$field[index.value() as usize]
            }
        }
    }
}

macro_rules! impl_try_get {
    ($ty:ty, $field:ident) => {
        #[doc(hidden)]
        impl<X: Extras> TryGet<$ty> for Root<X> {
            fn try_get(&self, index: &Index<$ty>) -> Result<&$ty, ()> {
                self.$field.get(index.value() as usize).ok_or(())
            }
        }
    }
}

impl_get!(accessor::Accessor<X>, accessors);
impl_get!(animation::Animation<X>, animations);
impl_get!(buffer::Buffer<X>, buffers);
impl_get!(buffer::BufferView<X>, buffer_views);
impl_get!(camera::Camera<X>, cameras);
impl_get!(image::Image<X>, images);
impl_get!(material::Material<X>, materials);
impl_get!(mesh::Mesh<X>, meshes);
impl_get!(scene::Node<X>, nodes);
impl_get!(texture::Sampler<X>, samplers);
impl_get!(scene::Scene<X>, scenes);
impl_get!(skin::Skin<X>, skins);
impl_get!(texture::Texture<X>, textures);

impl_try_get!(accessor::Accessor<X>, accessors);
impl_try_get!(animation::Animation<X>, animations);
impl_try_get!(buffer::Buffer<X>, buffers);
impl_try_get!(buffer::BufferView<X>, buffer_views);
impl_try_get!(camera::Camera<X>, cameras);
impl_try_get!(image::Image<X>, images);
impl_try_get!(material::Material<X>, materials);
impl_try_get!(mesh::Mesh<X>, meshes);
impl_try_get!(scene::Node<X>, nodes);
impl_try_get!(texture::Sampler<X>, samplers);
impl_try_get!(scene::Scene<X>, scenes);
impl_try_get!(skin::Skin<X>, skins);
impl_try_get!(texture::Texture<X>, textures);
