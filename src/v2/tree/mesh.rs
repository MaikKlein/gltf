
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::hash_map::Iter as HashMapIter;
use std::slice::Iter as SliceIter;
use v2::{self, tree, Extras};

use self::tree::accessor::Accessor;

/// XYZ vertex normals of type `[f32; 3]`.
pub type Normals<'a> = tree::accessor::Iter<'a, [f32; 3]>;

/// XYZ vertex positions of type `[f32; 3]`.
pub type Positions<'a> = tree::accessor::Iter<'a, [f32; 3]>;

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
pub type Tangents<'a> = tree::accessor::Iter<'a, [f32; 4]>;

/// Vertex attribute data.
pub enum Attribute<'a, E: 'a + Extras> {
    /// Vertex colors.
    Colors(u32, Colors<'a>),

    /// Untyped user-defined vertex attributes.
    Extras(&'a str, Accessor<'a, E>),

    /// Vertex joints.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    Joints(u32, Joints<'a>),

    /// XYZ vertex positions of type `[f32; 3]`.
    Positions(Positions<'a>),

    /// XYZ vertex normals of type `[f32; 3]`.
    Normals(Normals<'a>),

    /// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
    /// sign value (-1 or +1) indicating the handedness of the tangent basis.
    Tangents(Tangents<'a>),

    /// UV texture co-ordinates.
    TexCoords(u32, TexCoords<'a>),

    /// Weights.
    /// Refer to the documentation on morph targets for more information.
    Weights(u32, Weights<'a>),
}

/// Vertex colors.
#[derive(Clone, Debug)]
pub enum Colors<'a> {
    /// RGB vertex color of type `[u8; 3]>`.
    RgbU8(tree::accessor::Iter<'a, [u8; 3]>),

    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(tree::accessor::Iter<'a, [u8; 4]>),

    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(tree::accessor::Iter<'a, [u16; 3]>),

    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(tree::accessor::Iter<'a, [u16; 4]>),

    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(tree::accessor::Iter<'a, [f32; 3]>),

    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(tree::accessor::Iter<'a, [f32; 4]>),
}

/// Index data.
pub enum Indices<'a> {
    /// Index data of type U8
    U8(tree::accessor::Iter<'a, u8>),
    /// Index data of type U16
    U16(tree::accessor::Iter<'a, u16>),
    /// Index data of type U32
    U32(tree::accessor::Iter<'a, u32>),
}

/// An `Iterator` that visits the vertex attributes of a mesh primitive.
pub struct IterAttributes<'a, E: 'a + Extras> {
    iter: HashMapIter<'a, v2::mesh::Semantic, v2::root::Index<v2::accessor::Accessor<E>>>,
    root: &'a tree::root::Root<'a, E>,
}

/// An `Iterator that visits every primitive in a mesh.
pub struct IterPrimitives<'a, E: 'a + Extras> {
    iter: SliceIter<'a, v2::mesh::Primitive<E>>,
    root: &'a tree::root::Root<'a, E>,
}

/// Vertex joints.
/// Refer to the documentation on morph targets and skins for more
/// information.
#[derive(Clone, Debug)]
pub enum Joints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(tree::accessor::Iter<'a, [u8; 4]>),
    
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(tree::accessor::Iter<'a, [u16; 4]>),
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Debug)]
pub struct Mesh<'a, E: 'a + Extras> {
    mesh: &'a v2::mesh::Mesh<E>,
    root: &'a tree::root::Root<'a, E>,
}

#[derive(Debug)]
pub struct Primitive<'a, E: 'a + Extras> {
    primitive: &'a v2::mesh::Primitive<E>,
    root: &'a tree::root::Root<'a, E>,
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
pub enum TexCoords<'a> {
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(tree::accessor::Iter<'a, [f32; 2]>),

    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(tree::accessor::Iter<'a, [u8; 2]>),

    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(tree::accessor::Iter<'a, [u16; 2]>),
}

/// Weights,
/// Refer to the documentation on morph targets for more information.
#[derive(Clone, Debug)]
pub enum Weights<'a> {
    /// Weights of type `[f32; 4]`.
    F32(tree::accessor::Iter<'a, [f32; 4]>),

    /// Weights of type `[u8; 4]`.
    U8(tree::accessor::Iter<'a, [u8; 4]>),

    /// Weights of type `[u16; 4]`.
    U16(tree::accessor::Iter<'a, [u16; 4]>),
}

impl<'a, E: 'a + Extras> Mesh<'a, E> {
    /// Returns an `Iterator` that visits every primitive.
    pub fn iter_primitives(&'a self) -> IterPrimitives<'a, E> {
        IterPrimitives {
            iter: self.mesh.primitives.iter(),
            root: self.root,
        }
    }

    #[doc(hidden)]
    pub fn new(
        root: &'a tree::root::Root<E>,
        mesh: &'a v2::mesh::Mesh<E>,
    ) -> Self {
        Mesh {
            mesh: mesh,
            root: root,
        }
    }
}

impl<'a, E: 'a + Extras> Primitive<'a, E> {
    /// Returns the vertex colors of the given set.
    pub fn colors(&'a self, set: u32) -> Option<Colors<'a>> {
        for attribute in self.iter_attributes() {
            if let Attribute::Colors(set_, colors) = attribute {
                if set_ == set {
                    return Some(colors);
                }
            }
        }
        None
    }

    /// Returns the internal glTF object data.
    pub fn data(&'a self) -> &'a v2::mesh::Primitive<E> {
        &self.primitive
    }

    /// Returns an `Iterator` over the primitive vertex attributes.
    pub fn iter_attributes(&'a self) -> IterAttributes<'a, E> {
        IterAttributes {
            iter: self.primitive.attributes.iter(),
            root: self.root,
        }
    }

    /// Returns the referenced index buffer view.
    pub fn indices(&'a self) -> Option<Indices<'a>> {
        use v2::accessor::ComponentType::*;
        self.primitive.indices
            .as_ref()
            .map(|index| {
                let accessor = Accessor::new(self.root, self.root.get(index));
                match accessor.ty() {
                    U8 => Indices::U8(accessor.iter().unwrap()),
                    U16 => Indices::U16(accessor.iter().unwrap()),
                    U32 => Indices::U32(accessor.iter().unwrap()),
                    _ => panic!(),
                }
            })
    }

    #[doc(hidden)]
    pub fn new(
        root: &'a tree::root::Root<E>,
        primitive: &'a v2::mesh::Primitive<E>,
    ) -> Self {
        Primitive {
            primitive: primitive,
            root: root,
        }
    }

    /// Returns the vertex normals.
    pub fn normals(&'a self) -> Option<Normals<'a>> {
        for attribute in self.iter_attributes() {
            if let Attribute::Normals(normals) = attribute {
                return Some(normals);
            }
        }
        None
    }

    /// Returns the vertex positions.
    pub fn positions(&'a self) -> Option<Positions<'a>> {
        for attribute in self.iter_attributes() {
            if let Attribute::Positions(positions) = attribute {
                return Some(positions);
            }
        }
        None
    }

    /// Returns the UV texture co-ordinates of the given set.
    pub fn tex_coords(&'a self, set: u32) -> Option<TexCoords<'a>> {
        for attribute in self.iter_attributes() {
            if let Attribute::TexCoords(set_, tex_coords) = attribute {
                if set_ == set {
                    return Some(tex_coords);
                }
            }
        }
        None
    }
}

impl<'a, E: 'a + Extras> Iterator for IterAttributes<'a, E> {
    type Item = Attribute<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        use v2::accessor::ComponentType::*;
        use v2::accessor::Kind::*;
        use v2::mesh::*;
        self.iter.next().map(|(semantic, index)| {
            let accessor = Accessor::new(self.root, self.root.get(index));
            match (semantic, accessor.ty(), accessor.kind()) {
                (&Semantic::Color(set), F32, Vec3) => {
                    Attribute::Colors(
                        set,
                        Colors::RgbF32(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Color(set), F32, Vec4) => {
                    Attribute::Colors(
                        set,
                        Colors::RgbaF32(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Color(set), U8, Vec3) => {
                    Attribute::Colors(
                        set,
                        Colors::RgbU8(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Color(set), U8, Vec4) => {
                    Attribute::Colors(
                        set,
                        Colors::RgbaU8(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Color(set), U16, Vec3) => {
                    Attribute::Colors(
                        set,
                        Colors::RgbU16(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Color(set), U16, Vec4) => {
                    Attribute::Colors(
                        set,
                        Colors::RgbaU16(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Joint(set), U8, Vec4) => {
                    Attribute::Joints(
                        set,
                        Joints::U8(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Joint(set), U16, Vec4) => {
                    Attribute::Joints(
                        set,
                        Joints::U16(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Normal, F32, Vec3) => {
                    Attribute::Normals(accessor.iter().unwrap())
                },
                (&Semantic::Position, F32, Vec3) => {
                    Attribute::Positions(accessor.iter().unwrap())
                },
                (&Semantic::Tangent, F32, Vec3) => {
                    Attribute::Tangents(accessor.iter().unwrap())
                },
                (&Semantic::TexCoord(set), F32, Vec2) => {
                    Attribute::TexCoords(
                        set,
                        TexCoords::F32(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::TexCoord(set), U8, Vec2) => {
                    Attribute::TexCoords(
                        set,
                        TexCoords::U8(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::TexCoord(set), U16, Vec2) => {
                    Attribute::TexCoords(
                        set,
                        TexCoords::U16(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Weight(set), U8, Vec4) => {
                    Attribute::Weights(
                        set,
                        Weights::U8(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Weight(set), U16, Vec4) => {
                    Attribute::Weights(
                        set,
                        Weights::U16(accessor.iter().unwrap()),
                    )
                },
                (&Semantic::Extra(ref name), _, _) => {
                    Attribute::Extras(name, accessor)
                },
                _ => unreachable!(),
            }
        })
    }
}

impl<'a, E: 'a + Extras> Iterator for IterPrimitives<'a, E> {
    type Item = tree::mesh::Primitive<'a, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|primitive| Primitive::new(self.root, primitive))
    }
}
