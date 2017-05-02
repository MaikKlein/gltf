
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{self, fs, io, path};
use v2::{raw, scene, Extras, Validate};
use self::raw::root::{Get, Index, TryGet};
use self::scene::Scene;

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Debug)]
pub struct IterScenes<'a, X: 'a + Extras> {
    /// Internal scene iterator.
    iter: std::slice::Iter<'a, raw::scene::Scene<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// The root object for a glTF asset.
#[derive(Debug)]
pub struct Root<X: Extras> {
    /// Preloaded buffer data.
    buffer_data: Vec<Vec<u8>>,

    /// The path to the directory of the glTF source.
    ///
    /// Relative paths are determined from this location.
    path: path::PathBuf,

    /// The internal root glTF object data.
    raw: raw::root::Root<X>,
}

/// Reads the entire contents of a `Buffer`.
fn read_buffer_data<X, P>(
    buffer: &raw::buffer::Buffer<X>,
    gltf_origin: P,
) -> io::Result<Vec<u8>>
where
    X: Extras,
    P: AsRef<path::Path>,
{
    use self::io::Read;
    let path = gltf_origin.as_ref().with_file_name(&buffer.uri);
    let mut file = fs::File::open(&path)?;
    let mut data = Vec::with_capacity(buffer.byte_length as usize);
    unsafe {
        data.set_len(buffer.byte_length as usize);
    }
    file.read_exact(&mut data[..])?;
    Ok(data)
}

impl<X: Extras> Root<X> {
    /// Returns the raw glTF data.
    pub fn as_raw(self) -> raw::root::Root<X> {
        self.raw
    }

    /// Retrieves the pre-loaded buffer data described by the indexed buffer.
    pub fn buffer_data(&self, index: &Index<raw::buffer::Buffer<X>>) -> &[u8] {
        &self.buffer_data[index.value() as usize]
    }

    /// Returns the extensions referenced in this .gltf file.
    pub fn extensions_used(&self) -> &[String] {
        &self.raw.extensions_used
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&self) -> &[String] {
        &self.raw.extensions_required
    }

    /// Constructor for the `Root` object.
    ///
    /// It is recommended to use `import()` instead.
    pub fn from_raw<P>(raw: raw::root::Root<X>, path: P) -> io::Result<Self>
        where P: AsRef<path::Path>
    {
        let mut preloaded_buffer_data = Vec::new();
        for buffer in raw.buffers.iter() {
            let buffer_data = read_buffer_data(buffer, &path)?;
            preloaded_buffer_data.push(buffer_data);
        };
        Ok(Self {
            buffer_data: preloaded_buffer_data,
            path: path.as_ref().to_owned(),
            raw: raw,
        })
    }

    /// Returns an `Iterator` that walks the scenes of the glTF asset.
    pub fn iter_scenes<'a>(&'a self) -> IterScenes<'a, X> {
        IterScenes {            
            iter: self.raw.scenes.iter(),
            root: self,
        }
    }

    /// Returns the path to the directory of the glTF source.
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
    
    /// Returns a single item from the root object.
    ///
    /// # Panics
    ///
    /// If the index is out of range, but note that all values of type
    /// `raw::Index<T>` are validated during `import()`. If you are using the
    /// raw version, then it is best to use `try_get()` instead.
    pub fn get<T>(&self, index: &Index<T>) -> &T
        where Self: Get<T>
    {
        (self as &Get<T>).get(index)
    }

    /// Returns a single item from the root object if the index is in range.
    ///
    /// Note that if the glTF was loaded using `import()` then all values of
    /// type `raw::Index<T>` will have been pre-checked for validity for your
    /// convenience so that you can just call `get()`.
    pub fn try_get<T>(&self, index: &Index<T>) -> Result<&T, ()>
        where Self: TryGet<T>
    {
        (self as &TryGet<T>).try_get(index)
    }
}
    
impl<X: Extras> Validate<X> for Root<X> {
    /// Validates the entire glTF tree.
    fn validate<W, E>(&self, _root: &Root<X>, mut warn: W, mut err: E)
        where W: FnMut(&str, &str), E: FnMut(&str, &str)
    {
        for (i, accessor) in self.raw.accessors.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("accessors[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("accessors[{}].{}", i, source);
                err(&source, description);
            };
            accessor.validate(self, warn_fn, err_fn);
        }

        for (i, animation) in self.raw.animations.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("animations[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("animations[{}].{}", i, source);
                err(&source, description);
            };
            animation.validate(self, warn_fn, err_fn);
        }

        for (i, buffer) in self.raw.buffers.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("buffers[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("buffers[{}].{}", i, source);
                err(&source, description);
            };
            buffer.validate(self, warn_fn, err_fn);
        }

        for (i, buffer_view) in self.raw.buffer_views.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("bufferViews[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("bufferViews[{}].{}", i, source);
                err(&source, description);
            };
            buffer_view.validate(self, warn_fn, err_fn);
        }

        for (i, camera) in self.raw.cameras.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("cameras[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("cameras[{}].{}", i, source);
                err(&source, description);
            };
            camera.validate(self, warn_fn, err_fn);            
        }
        
        for (i, image) in self.raw.images.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("images[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("images[{}].{}", i, source);
                err(&source, description);
            };
            image.validate(self, warn_fn, err_fn);
        }
        
        for (i, material) in self.raw.materials.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("materials[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("materials[{}].{}", i, source);
                err(&source, description);
            };
            material.validate(self, warn_fn, err_fn);
        }

        for (i, mesh) in self.raw.meshes.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("mesh[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("mesh[{}].{}", i, source);
                err(&source, description);
            };
            mesh.validate(self, warn_fn, err_fn);
        }

        for (i, node) in self.raw.nodes.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("node[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("node[{}].{}", i, source);
                err(&source, description);
            };
            node.validate(self, warn_fn, err_fn);   
        }

        for (i, scene) in self.raw.scenes.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("scenes[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("scenes[{}].{}", i, source);
                err(&source, description);
            };
            scene.validate(self, warn_fn, err_fn);
        }

        for (i, skin) in self.raw.skins.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("skins[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("skins[{}].{}", i, source);
                err(&source, description);
            };
            skin.validate(self, warn_fn, err_fn);            
        }
        
        if let Err(_) = self.try_get(&self.raw.default_scene) {
            err("scene", "Index out of range");
        }

        for (i, texture) in self.raw.textures.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("textures[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("textures[{}].{}", i, source);
                err(&source, description);
            };
            texture.validate(self, warn_fn, err_fn);
        }
    }
}

impl<'a, X: 'a + Extras> Iterator for IterScenes<'a, X> {
    type Item = Scene<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Scene::from_raw(self.root, raw))
    }
}

macro_rules! impl_get {
    ($ty:ty, $field:ident) => {
        impl<X: Extras> Get<$ty> for Root<X> {
            fn get(&self, index: &Index<$ty>) -> &$ty {
                &self.raw.$field[index.value() as usize]
            }
        }
    }
}

macro_rules! impl_try_get {
    ($ty:ty, $field:ident) => {
        #[doc(hidden)]
        impl<X: Extras> TryGet<$ty> for Root<X> {
            fn try_get(&self, index: &Index<$ty>) -> Result<&$ty, ()> {
                self.raw.$field.get(index.value() as usize).ok_or(())
            }
        }
    }
}

impl_get!(raw::accessor::Accessor<X>, accessors);
impl_get!(raw::animation::Animation<X>, animations);
impl_get!(raw::buffer::Buffer<X>, buffers);
impl_get!(raw::buffer::BufferView<X>, buffer_views);
impl_get!(raw::camera::Camera<X>, cameras);
impl_get!(raw::image::Image<X>, images);
impl_get!(raw::material::Material<X>, materials);
impl_get!(raw::mesh::Mesh<X>, meshes);
impl_get!(raw::scene::Node<X>, nodes);
impl_get!(raw::texture::Sampler<X>, samplers);
impl_get!(raw::scene::Scene<X>, scenes);
impl_get!(raw::skin::Skin<X>, skins);
impl_get!(raw::texture::Texture<X>, textures);

impl_try_get!(raw::accessor::Accessor<X>, accessors);
impl_try_get!(raw::animation::Animation<X>, animations);
impl_try_get!(raw::buffer::Buffer<X>, buffers);
impl_try_get!(raw::buffer::BufferView<X>, buffer_views);
impl_try_get!(raw::camera::Camera<X>, cameras);
impl_try_get!(raw::image::Image<X>, images);
impl_try_get!(raw::material::Material<X>, materials);
impl_try_get!(raw::mesh::Mesh<X>, meshes);
impl_try_get!(raw::scene::Node<X>, nodes);
impl_try_get!(raw::texture::Sampler<X>, samplers);
impl_try_get!(raw::scene::Scene<X>, scenes);
impl_try_get!(raw::skin::Skin<X>, skins);
impl_try_get!(raw::texture::Texture<X>, textures);
