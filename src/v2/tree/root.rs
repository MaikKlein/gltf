
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use v2::{self, tree, Extras};

/// Error encountered when constructing a glTF 2.0 tree wrapper.
#[derive(Debug)]
pub enum CreationError {
    /// Error encountered when pre-loading buffer data.
    Preload(tree::buffer::PreloadError),
}

/// An `Iterator` that visits every scene in a glTF asset.
#[derive(Debug)]
pub struct IterScenes<'a, E: 'a + Extras> {
    /// The current index in the iteration.
    index: usize,

    /// The internal root glTF object.
    root: &'a tree::root::Root<'a, E>,
}

/// The root object for a glTF asset.
#[derive(Debug)]
pub struct Root<'a, E: 'a + Extras> {
    /// The path the glTF asset was sourced from.
    ///
    /// Relative paths are determined from this location.
    path: &'a std::path::Path,

    /// Pre-loaded buffer data.
    ///
    /// The indices of each buffer exactly match the indices of each buffer in the
    /// root glTF object.
    preloaded_buffers: Vec<tree::buffer::PreloadedBuffer<'a, E>>,

    /// The internal root glTF object.
    root: &'a v2::root::Root<E>,
}

impl<'a, E: 'a + Extras> Root<'a, E> {
    /// Returns the internal glTF object data.
    pub fn data(&'a self) -> &'a v2::root::Root<E> {
        &self.root
    }

    pub fn preloaded_buffer(
        &self,
        index: &v2::Index<v2::buffer::Buffer<E>>,
    ) -> &v2::tree::buffer::PreloadedBuffer<'a, E> {
        &self.preloaded_buffers[index.value() as usize]
    }
    
    /// Returns a reference to the glTF root object that can be used to perform
    /// tree traversal operations.
    pub fn new(
        root: &'a v2::root::Root<E>,
        path: &'a std::path::Path,
    ) -> Result<Self, CreationError> {
        let mut preloaded_buffers = Vec::new();
        for buffer in root.buffers().iter() {
            let preloaded_buffer = tree::buffer::preload(buffer, path)
                .map_err(CreationError::Preload)?;
            preloaded_buffers.push(preloaded_buffer)
        }
        Ok(Self {
            path: path,
            preloaded_buffers: preloaded_buffers,
            root: root,
        })
    }

    /// Returns an `Iterator` that iters the scenes of the glTF asset.
    pub fn iter_scenes(&'a self) -> IterScenes<'a, E> {
        IterScenes {            
            index: 0,
            root: self,
        }
    }

    /// Returns the path of the glTF asset.
    pub fn path(&'a self) -> &'a std::path::Path {
        self.path
    }
}

impl<'a, E: 'a + Extras> Iterator for IterScenes<'a, E> {
    type Item = tree::scene::Scene<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.root.scenes().len() {
            self.index += 1;
            Some(tree::scene::Scene::new(
                self.root,
                &self.root.scenes()[self.index - 1],
            ))
        } else {
            None
        }
    }
}

impl<'a, E: 'a + Extras> std::ops::Deref for Root<'a, E> {
    type Target = v2::root::Root<E>;
    fn deref(&self) -> &Self::Target {
        self.root
    }
}

