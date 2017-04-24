
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{self, tree, Extras};

/// An `Iterator` that visits the children of a node.
#[derive(Debug)]
pub struct IterChildNodes<'a, E: 'a + Extras> {
    index: usize,
    parent: &'a Node<'a, E>,
    root: &'a tree::root::Root<'a, E>,
}

/// An `Iterator` that visits every node in a scene.
#[derive(Debug)]
pub struct IterNodes<'a, E: 'a + Extras> {
    index: usize,
    root: &'a tree::root::Root<'a, E>,
    scene: &'a v2::scene::Scene<E>,
}

/// ![Node](../scene/struct.Node.html)
#[derive(Debug)]
pub struct Node<'a, E: 'a + Extras> {
    node: &'a v2::scene::Node<E>,
    parent: Option<&'a Node<'a, E>>,
    root: &'a tree::root::Root<'a, E>,
}

/// The root nodes of a scene.
#[derive(Debug)]
pub struct Scene<'a, E: 'a + Extras> {
    root: &'a tree::root::Root<'a, E>,
    scene: &'a v2::scene::Scene<E>,
}

impl<'a, E: 'a + Extras> Node<'a, E> {
    /// Returns the camera referenced by this node.
    pub fn camera(&'a self) -> Option<&'a v2::camera::Camera<E>> {
        self.node.camera.as_ref().map(|index| self.root.get(index))
    }

    /// Returns the internal glTF object data.
    pub fn data(&'a self) -> &'a v2::scene::Node<E> {
        &self.node
    }
    
    /// Returns the mesh referenced by this node.
    pub fn mesh(&'a self) -> Option<tree::mesh::Mesh<E>> {
        self.node.mesh
            .as_ref()
            .map(|index| {
                tree::mesh::Mesh::new(
                    self.root,
                    self.root.get(index),
                )
            })
    }

    #[doc(hidden)]
    pub fn new(
        root: &'a tree::root::Root<E>,
        parent: Option<&'a Node<'a, E>>,
        node: &'a v2::scene::Node<E>,
    ) -> Self {
        Node {
            node: node,
            parent: parent,
            root: root,
        }
    }

    /// Returns this node's parent node.
    pub fn parent(&'a self) -> Option<&'a Node<E>> {
        self.parent
    }
    
    /// Returns the skin referenced by this node.
    pub fn skin(&'a self) -> Option<&'a v2::skin::Skin<E>> {
        self.node.skin.as_ref().map(|index| self.root.get(index))
    }

    /// Returns an `Iterator` that visits every child node.
    pub fn iter_child_nodes(&'a self) -> IterChildNodes<'a, E> {
        IterChildNodes {
            index: 0,
            parent: self,
            root: self.root,            
        }
    }
}

impl<'a, E: 'a + Extras> Scene<'a, E> {
    /// Returns the internal glTF object data.
    pub fn data(&'a self) -> &'a v2::scene::Scene<E> {
        &self.scene
    }

    /// Returns an `Iterator` that iters the root nodes in a scene.
    pub fn iter_nodes(&'a self) -> IterNodes<'a, E> {
        IterNodes {
            index: 0,
            root: self.root,
            scene: self.scene,
        }
    }

    #[doc(hidden)]
    pub fn new(
        root: &'a tree::root::Root<E>,
        scene: &'a v2::scene::Scene<E>,
    ) -> Self {
        Scene {
            root: root,
            scene: scene,
        }
    }
}

impl<'a, E: 'a + Extras> Iterator for IterChildNodes<'a, E> {
    type Item = Node<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.parent.node.children.len() {
            self.index += 1;
            Some(Node {
                node: self.root.get(&self.parent.node.children[self.index - 1]),
                parent: Some(self.parent),
                root: self.root,
            })
        } else {
            None
        }
    }
}

impl<'a, E: 'a + Extras> Iterator for IterNodes<'a, E> {
    type Item = Node<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.scene.nodes.len() {
            self.index += 1;
            Some(Node {
                node: self.root.get(&self.scene.nodes[self.index - 1]),
                parent: None,
                root: self.root,
            })
        } else {
            None
        }
    }
}

