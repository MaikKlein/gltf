
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use image_crate;
use std::{self, fs, io, path};
use std::slice::Iter as SliceIter;
use v2::{
    accessor,
    animation,
    buffer,
    camera,
    image,
    material,
    raw,
    texture,
    scene,
    skin,
    Extras,
    Validate
};

use self::raw::root::{Get, Index, TryGet};
use self::accessor::Accessor;
use self::animation::Animation;
use self::buffer::{Buffer, BufferView};
use self::camera::Camera;
use self::image::Image;
use self::material::Material;
use self::scene::{Node, Scene};
use self::skin::Skin;
use self::texture::{Sampler, Texture};

/// Data described by an `Image`.
#[derive(Debug)]
enum ImageData {
    /// Index of the `BufferView` this image borrows from.
    FromBufferView(usize),

    /// The owned data this image describes.
    Owned(Vec<u8>),
}

/// Return value of `Root::load()`.
#[derive(Debug)]
pub enum LoadError {
    /// Image decoding error.
    Image(image_crate::ImageError),

    /// Standard input / output error.
    Io(std::io::Error),
}

/// An `Iterator` that visits every accessor in a glTF asset.
#[derive(Debug)]
pub struct IterAccessors<'a, X: 'a + Extras> {
    /// Internal accessor iterator.
    iter: SliceIter<'a, raw::accessor::Accessor<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every animation in a glTF asset.
#[derive(Debug)]
pub struct IterAnimations<'a, X: 'a + Extras> {
    /// Internal animation iterator.
    iter: SliceIter<'a, raw::animation::Animation<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every pre-loaded buffer in a glTF asset.
#[derive(Debug)]
pub struct IterBuffers<'a, X: 'a + Extras> {
    /// Index of next buffer in the iteration.
    index: usize,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every pre-loaded buffer view in a glTF asset.
#[derive(Debug)]
pub struct IterBufferViews<'a, X: 'a + Extras> {
    /// Internal buffer view iterator.
    iter: SliceIter<'a, raw::buffer::BufferView<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every camera in a glTF asset.
#[derive(Debug)]
pub struct IterCameras<'a, X: 'a + Extras> {
    /// Internal buffer view iterator.
    iter: SliceIter<'a, raw::camera::Camera<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every pre-loaded image in a glTF asset.
#[derive(Debug)]
pub struct IterImages<'a, X: 'a + Extras> {
    /// Index of next image in the iteration.
    index: usize,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every material in a glTF asset.
#[derive(Debug)]
pub struct IterMaterials<'a, X: 'a + Extras> {
    /// Internal material iterator.
    iter: SliceIter<'a, raw::material::Material<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every node in a glTF asset.
#[derive(Debug)]
pub struct IterNodes<'a, X: 'a + Extras> {
    /// Internal node iterator.
    iter: SliceIter<'a, raw::scene::Node<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every sampler in a glTF asset.
#[derive(Debug)]
pub struct IterSamplers<'a, X: 'a + Extras> {
    /// Internal sampler iterator.
    iter: SliceIter<'a, raw::texture::Sampler<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Debug)]
pub struct IterScenes<'a, X: 'a + Extras> {
    /// Internal scene iterator.
    iter: SliceIter<'a, raw::scene::Scene<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every skin in a glTF asset.
#[derive(Debug)]
pub struct IterSkins<'a, X: 'a + Extras> {
    /// Internal skin iterator.
    iter: SliceIter<'a, raw::skin::Skin<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// An `Iterator` that visits every texture in a glTF asset.
#[derive(Debug)]
pub struct IterTextures<'a, X: 'a + Extras> {
    /// Internal texture iterator.
    iter: SliceIter<'a, raw::texture::Texture<X>>,

    /// The internal root glTF object.
    root: &'a Root<X>,
}

/// The root object for a glTF asset.
#[derive(Debug)]
pub struct Root<X: Extras> {
    /// Pre-loaded buffer data.
    buffer_data: Vec<Vec<u8>>,

    /// Pre-loaded image data.
    image_data: Vec<ImageData>,
    
    /// The path to the directory of the glTF source.
    ///
    /// Relative paths are determined from this location.
    path: path::PathBuf,

    /// The internal root glTF object data.
    raw: raw::root::Root<X>,
}

/// Reads the contents of a `Buffer`.
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

fn read_image_data<P1, P2>(
    image_path: P1,
    gltf_origin: P2,
) -> image_crate::ImageResult<Vec<u8>>
where
    P1: AsRef<std::path::Path>,
    P2: AsRef<std::path::Path>,
{
    let path = gltf_origin.as_ref().with_file_name(image_path.as_ref());
    let image = image_crate::open(path)?;
    Ok(image.raw_pixels())
}

impl<X: Extras> Root<X> {
    /// Returns the raw glTF data.
    pub fn as_raw(self) -> raw::root::Root<X> {
        self.raw
    }

    /// Retrieves the pre-loaded buffer data at the given index.
    fn buffer_data_impl(&self, index: usize) -> &[u8] {
        self.buffer_data[index].as_slice()
    }
    
    /// Retrieves the pre-loaded buffer data described by the indexed buffer.
    pub fn buffer_data(&self, index: &Index<raw::buffer::Buffer<X>>) -> &[u8] {
        self.buffer_data_impl(index.value() as usize)
    }

    /// Retrieves the pre-loaded image data at the given index.
    fn image_data_impl(&self, index: usize) -> &[u8] {
        match &self.image_data[index as usize] {
            &ImageData::FromBufferView(buffer_view_index) => {
                let buffer_view = &self.raw.buffer_views[buffer_view_index];
                let buffer_data = &self.buffer_data(&buffer_view.buffer);
                let begin = buffer_view.byte_offset as usize;
                let end = begin + buffer_view.byte_length as usize;
                &buffer_data[begin..end]
            },
            &ImageData::Owned(ref data) => data.as_slice(),
        }  
    }

    /// Retrieves the pre-loaded image data described by the indexed image.
    pub fn image_data(&self, index: &Index<raw::image::Image<X>>) -> &[u8] {
        self.image_data_impl(index.value() as usize)
    }

    /// Returns the extensions referenced in this .gltf file.
    pub fn extensions_used(&self) -> &[String] {
        &self.raw.extensions_used
    }

    /// Returns the extensions required to load and render this asset.
    pub fn extensions_required(&self) -> &[String] {
        &self.raw.extensions_required
    }

    /// Constructor for the `Root` object.    ///
    /// It is recommended to use `import()` instead.
    pub fn load<P>(raw: raw::root::Root<X>, path: P) -> Result<Self, LoadError>
        where P: AsRef<path::Path>
    {
        let mut preloaded_buffer_data = Vec::new();
        for buffer in raw.buffers.iter() {
            let buffer_data = read_buffer_data(buffer, &path)?;
            preloaded_buffer_data.push(buffer_data);
        };
        let mut preloaded_image_data = Vec::new();
        for image in raw.images.iter() {
            let image_data = if let Some(index) = image.buffer_view.as_ref() {
                ImageData::FromBufferView(index.value() as usize)
            } else {
                let owned = read_image_data(image.uri.as_ref().unwrap(), &path)?;
                ImageData::Owned(owned)
            };
            preloaded_image_data.push(image_data);
        }
        Ok(Self {
            buffer_data: preloaded_buffer_data,
            image_data: preloaded_image_data,
            path: path.as_ref().to_owned(),
            raw: raw,
        })
    }

    /// Returns an `Iterator` that visits the accessors of the glTF asset.
    pub fn iter_accessors<'a>(&'a self) -> IterAccessors<'a, X> {
        IterAccessors {
            iter: self.raw.accessors.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the animations of the glTF asset.
    pub fn iter_animations<'a>(&'a self) -> IterAnimations<'a, X> {
        IterAnimations {
            iter: self.raw.animations.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffers of the glTF
    /// asset.
    pub fn iter_buffers<'a>(&'a self) -> IterBuffers<'a, X> {
        IterBuffers {
            index: 0,
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded buffer views of the glTF
    /// asset.
    pub fn iter_buffer_views<'a>(&'a self) -> IterBufferViews<'a, X> {
        IterBufferViews {
            iter: self.raw.buffer_views.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the cameras of the glTF asset.
    pub fn iter_cameras<'a>(&'a self) -> IterCameras<'a, X> {
        IterCameras {
            iter: self.raw.cameras.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the pre-loaded images of the glTF asset.
    pub fn iter_images<'a>(&'a self) -> IterImages<'a, X> {
        IterImages {
            index: 0,
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the materials of the glTF asset.
    pub fn iter_materials<'a>(&'a self) -> IterMaterials<'a, X> {
        IterMaterials {            
            iter: self.raw.materials.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the nodes of the glTF asset.
    pub fn iter_nodes<'a>(&'a self) -> IterNodes<'a, X> {
        IterNodes {            
            iter: self.raw.nodes.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the scenes of the glTF asset.
    pub fn iter_samplers<'a>(&'a self) -> IterSamplers<'a, X> {
        IterSamplers {            
            iter: self.raw.samplers.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the samplers of the glTF asset.
    pub fn iter_scenes<'a>(&'a self) -> IterScenes<'a, X> {
        IterScenes {            
            iter: self.raw.scenes.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the skins of the glTF asset.
    pub fn iter_skins<'a>(&'a self) -> IterSkins<'a, X> {
        IterSkins {            
            iter: self.raw.skins.iter(),
            root: self,
        }
    }

    /// Returns an `Iterator` that visits the textures of the glTF asset.
    pub fn iter_textures<'a>(&'a self) -> IterTextures<'a, X> {
        IterTextures {            
            iter: self.raw.textures.iter(),
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

impl<'a, X: 'a + Extras> Iterator for IterAccessors<'a, X> {
    type Item = Accessor<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Accessor::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterAnimations<'a, X> {
    type Item = Animation<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Animation::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterBuffers<'a, X> {
    type Item = Buffer<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.root.raw.buffers.len() {
            let raw = &self.root.raw.buffers[self.index];
            let buffer_data = self.root.buffer_data_impl(self.index);
            let item = Buffer::from_raw(self.root, raw, buffer_data);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, X: 'a + Extras> Iterator for IterBufferViews<'a, X> {
    type Item = BufferView<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| BufferView::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterCameras<'a, X> {
    type Item = Camera<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Camera::from_raw(raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterImages<'a, X> {
    type Item = Image<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.root.raw.images.len() {
            let raw = &self.root.raw.images[self.index];
            let data = self.root.image_data_impl(self.index);
            self.index += 1;
            Some(Image::from_raw(self. root, raw, data))
        } else {
            None
        }
    }
}

impl<'a, X: 'a + Extras> Iterator for IterMaterials<'a, X> {
    type Item = Material<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Material::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterNodes<'a, X> {
    type Item = Node<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Node::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterSamplers<'a, X> {
    type Item = Sampler<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Sampler::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterScenes<'a, X> {
    type Item = Scene<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Scene::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterSkins<'a, X> {
    type Item = Skin<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Skin::from_raw(self.root, raw))
    }
}

impl<'a, X: 'a + Extras> Iterator for IterTextures<'a, X> {
    type Item = Texture<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|raw| Texture::from_raw(self.root, raw))
    }
}

impl From<image_crate::ImageError> for LoadError {
    fn from(err: image_crate::ImageError) -> LoadError {
        LoadError::Image(err)
    }
}

impl From<std::io::Error> for LoadError {
    fn from(err: std::io::Error) -> LoadError {
        LoadError::Io(err)
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
