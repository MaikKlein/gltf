
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf as gltf_crate;

use gltf_crate::v2 as gltf;

type Extras = gltf::extras::None;
type Gltf = gltf::Root<Extras>;

fn visit_nodes(node: &gltf::scene::Node<Extras>, level: u32) {
    println!("Node {}", level);
    for child in node.iter_child_nodes() {
        visit_nodes(&child, level + 1);
    }
}

fn main() {
    let path = "glTF-Sample-Models/2.0/Lantern/glTF/Lantern.gltf";
    let gltf: Gltf = gltf::import(path).unwrap();
    for scene in gltf.iter_scenes() {
        for node in scene.iter_nodes() {
            visit_nodes(&node, 1);
        }
    }
}

