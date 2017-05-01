
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{raw, Extras, Root};
use v2::mesh::Mesh;

/// An `Iterator` that visits the children of a node.
#[derive(Debug)]
pub struct IterChildNodes<'a, X: 'a + Extras> {
    index: usize,
    parent: &'a Node<'a, X>,
    root: &'a Root<X>,
}

/// An `Iterator` that visits every node in a scene.
#[derive(Debug)]
pub struct IterNodes<'a, X: 'a + Extras> {
    index: usize,
    root: &'a Root<X>,
    scene: &'a Scene<'a, X>,
}

/// ![Node](../scene/struct.Node.html)
#[derive(Debug)]
pub struct Node<'a, X: 'a + Extras> {
    parent: Option<&'a Node<'a, X>>,
    raw: &'a raw::scene::Node<X>,
    root: &'a Root<X>,
}

/// The root nodes of a scene.
#[derive(Debug)]
pub struct Scene<'a, X: 'a + Extras> {
    root: &'a Root<X>,
    raw: &'a raw::scene::Scene<X>,
}

impl<'a, X: 'a + Extras> Node<'a, X> {
    /// Returns the camera referenced by this node.
    pub fn camera(&'a self) -> Option<&'a raw::camera::Camera<X>> {
        self.raw.camera.as_ref().map(|index| self.root.get(index))
    }
    
    /// Returns the mesh referenced by this node.
    pub fn mesh(&'a self) -> Option<Mesh<X>> {
        self.raw.mesh
            .as_ref()
            .map(|index| Mesh::from_raw(self.root, self.root.get(index)))
    }

    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::scene::Node<X>,
    ) -> Self {
        Self {
            raw: raw,
            parent: None,
            root: root,
        }
    }

    pub fn from_raw_with_parent(
        root: &'a Root<X>,
        raw: &'a raw::scene::Node<X>,
        parent: &'a Node<'a, X>,
    ) -> Self {
        Self {
            raw: raw,
            parent: Some(parent),
            root: root,
        }
    }

    /// Returns this node's parent node.
    pub fn parent(&'a self) -> Option<&'a Node<X>> {
        self.parent
    }

    /// Returns an `Iterator` that visits every child node.
    pub fn iter_child_nodes(&'a self) -> IterChildNodes<'a, X> {
        IterChildNodes {
            index: 0,
            parent: self,
            root: self.root,            
        }
    }
}

impl<'a, X: 'a + Extras> Scene<'a, X> {
    /// Returns an `Iterator` that iters the root nodes in a scene.
    pub fn iter_nodes(&'a self) -> IterNodes<'a, X> {
        IterNodes {
            index: 0,
            root: self.root,
            scene: self,
        }
    }

    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::scene::Scene<X>,
    ) -> Self {
        Self {
            root: root,
            raw: raw,
        }
    }
}

impl<'a, X: 'a + Extras> Iterator for IterChildNodes<'a, X> {
    type Item = Node<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.parent.raw.children.len() {
            self.index += 1;
            let node = Node::from_raw_with_parent(
                self.root,
                self.root.get(&self.parent.raw.children[self.index - 1]),
                self.parent,
            );
            Some(node)
        } else {
            None
        }
    }
}

impl<'a, X: 'a + Extras> Iterator for IterNodes<'a, X> {
    type Item = Node<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.scene.raw.nodes.len() {
            self.index += 1;
            let node = Node::from_raw(
                self.root,
                self.root.get(&self.scene.raw.nodes[self.index - 1]),
            );
            Some(node)
        } else {
            None
        }
    }
}

