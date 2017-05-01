
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::hash_map::Iter as HashMapIter;
use std::slice::Iter as SliceIter;
use v2::{accessor, raw, Extras, Root};

/// XYZ vertex normals of type `[f32; 3]`.
pub type Normals<'a> = accessor::Iter<'a, [f32; 3]>;

/// XYZ vertex positions of type `[f32; 3]`.
pub type Positions<'a> = accessor::Iter<'a, [f32; 3]>;

/// XYZW vertex tangents of type `[f32; 4]` where the `w` component is a
/// sign value (-1 or +1) indicating the handedness of the tangent basis.
pub type Tangents<'a> = accessor::Iter<'a, [f32; 4]>;

/// Vertex attribute data.
pub enum Attribute<'a, X: 'a + Extras> {
    /// Vertex colors.
    Colors(u32, Colors<'a>),

    /// Untyped user-defined vertex attributes.
    Extras(&'a str, accessor::Accessor<'a, X>),

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
    RgbU8(accessor::Iter<'a, [u8; 3]>),

    /// RGBA vertex color of type `[u8; 4]>`.
    RgbaU8(accessor::Iter<'a, [u8; 4]>),

    /// RGB vertex color of type `[u16; 3]>`.
    RgbU16(accessor::Iter<'a, [u16; 3]>),

    /// RGBA vertex color of type `[u16; 4]>`.
    RgbaU16(accessor::Iter<'a, [u16; 4]>),

    /// RGB vertex color of type `[f32; 3]`.
    RgbF32(accessor::Iter<'a, [f32; 3]>),

    /// RGBA vertex color of type `[f32; 4]`.
    RgbaF32(accessor::Iter<'a, [f32; 4]>),
}

/// Index data.
pub enum Indices<'a> {
    /// Index data of type U8
    U8(accessor::Iter<'a, u8>),
    /// Index data of type U16
    U16(accessor::Iter<'a, u16>),
    /// Index data of type U32
    U32(accessor::Iter<'a, u32>),
}

/// An `Iterator` that visits the vertex attributes of a mesh primitive.
pub struct IterAttributes<'a, X: 'a + Extras> {
    iter: HashMapIter<'a, raw::mesh::Semantic, raw::Index<raw::accessor::Accessor<X>>>,
    root: &'a Root<X>,
}

/// An `Iterator that visits every primitive in a mesh.
pub struct IterPrimitives<'a, X: 'a + Extras> {
    iter: SliceIter<'a, raw::mesh::Primitive<X>>,
    root: &'a Root<X>,
}

/// Vertex joints.
/// Refer to the documentation on morph targets and skins for more
/// information.
#[derive(Clone, Debug)]
pub enum Joints<'a> {
    /// Joints of type `[u8; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U8(accessor::Iter<'a, [u8; 4]>),
    
    /// Joints of type `[u16; 4]`.
    /// Refer to the documentation on morph targets and skins for more
    /// information.
    U16(accessor::Iter<'a, [u16; 4]>),
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Debug)]
pub struct Mesh<'a, X: 'a + Extras> {
    raw: &'a raw::mesh::Mesh<X>,
    root: &'a Root<X>,
}

#[derive(Debug)]
pub struct Primitive<'a, X: 'a + Extras> {
    raw: &'a raw::mesh::Primitive<X>,
    root: &'a Root<X>,
}

/// UV texture co-ordinates.
#[derive(Clone, Debug)]
pub enum TexCoords<'a> {
    /// UV texture co-ordinates of type `[f32; 2]`.
    F32(accessor::Iter<'a, [f32; 2]>),

    /// UV texture co-ordinates of type `[u8; 2]>`.
    U8(accessor::Iter<'a, [u8; 2]>),

    /// UV texture co-ordinates of type `[u16; 2]>`.
    U16(accessor::Iter<'a, [u16; 2]>),
}

/// Weights,
/// Refer to the documentation on morph targets for more information.
#[derive(Clone, Debug)]
pub enum Weights<'a> {
    /// Weights of type `[f32; 4]`.
    F32(accessor::Iter<'a, [f32; 4]>),

    /// Weights of type `[u8; 4]`.
    U8(accessor::Iter<'a, [u8; 4]>),

    /// Weights of type `[u16; 4]`.
    U16(accessor::Iter<'a, [u16; 4]>),
}

impl<'a, X: 'a + Extras> Mesh<'a, X> {
    /// Returns an `Iterator` that visits every primitive.
    pub fn iter_primitives(&'a self) -> IterPrimitives<'a, X> {
        IterPrimitives {
            iter: self.raw.primitives.iter(),
            root: self.root,
        }
    }

    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::mesh::Mesh<X>,
    ) -> Self {
        Self {
            raw: raw,
            root: root,
        }
    }
}

impl<'a, X: 'a + Extras> Primitive<'a, X> {
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

    /// Returns an `Iterator` over the primitive vertex attributes.
    pub fn iter_attributes(&'a self) -> IterAttributes<'a, X> {
        IterAttributes {
            iter: self.raw.attributes.iter(),
            root: self.root,
        }
    }

    /// Returns the referenced index buffer view.
    pub fn indices(&'a self) -> Option<Indices<'a>> {
        use self::raw::accessor::ComponentType::*;
        self.raw.indices
            .as_ref()
            .map(|index| {
                let accessor = accessor::Accessor::from_raw(
                    self.root,
                    self.root.get(index),
                );
                unsafe {
                    match accessor.ty() {
                        U8 => Indices::U8(accessor.iter()),
                        U16 => Indices::U16(accessor.iter()),
                        U32 => Indices::U32(accessor.iter()),
                        _ => unreachable!(),
                    }
                }
            })
    }

    pub fn from_raw(
        root: &'a Root<X>,
        raw: &'a raw::mesh::Primitive<X>,
    ) -> Self {
        Self {
            raw: raw,
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

impl<'a, X: 'a + Extras> Iterator for IterAttributes<'a, X> {
    type Item = Attribute<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        use self::raw::accessor::ComponentType::*;
        use self::raw::accessor::Kind::*;
        use self::raw::mesh::*;
        self.iter.next().map(|(semantic, index)| {
            let accessor = accessor::Accessor::from_raw(
                self.root,
                self.root.get(index),
            );
            match (semantic, accessor.ty(), accessor.kind()) {
                (&Semantic::Color(set), F32, Vec3) => unsafe {
                    Attribute::Colors(
                        set,
                        Colors::RgbF32(accessor.iter()),
                    )
                },
                (&Semantic::Color(set), F32, Vec4) => unsafe {
                    Attribute::Colors(
                        set,
                        Colors::RgbaF32(accessor.iter()),
                    )
                },
                (&Semantic::Color(set), U8, Vec3) => unsafe {
                    Attribute::Colors(
                        set,
                        Colors::RgbU8(accessor.iter()),
                    )
                },
                (&Semantic::Color(set), U8, Vec4) => unsafe {
                    Attribute::Colors(
                        set,
                        Colors::RgbaU8(accessor.iter()),
                    )
                },
                (&Semantic::Color(set), U16, Vec3) => unsafe {
                    Attribute::Colors(
                        set,
                        Colors::RgbU16(accessor.iter()),
                    )
                },
                (&Semantic::Color(set), U16, Vec4) => unsafe {
                    Attribute::Colors(
                        set,
                        Colors::RgbaU16(accessor.iter()),
                    )
                },
                (&Semantic::Joint(set), U8, Vec4) => unsafe {
                    Attribute::Joints(
                        set,
                        Joints::U8(accessor.iter()),
                    )
                },
                (&Semantic::Joint(set), U16, Vec4) => unsafe {
                    Attribute::Joints(
                        set,
                        Joints::U16(accessor.iter()),
                    )
                },
                (&Semantic::Normal, F32, Vec3) => unsafe {
                    Attribute::Normals(accessor.iter())
                },
                (&Semantic::Position, F32, Vec3) => unsafe {
                    Attribute::Positions(accessor.iter())
                },
                (&Semantic::Tangent, F32, Vec3) => unsafe {
                    Attribute::Tangents(accessor.iter())
                },
                (&Semantic::TexCoord(set), F32, Vec2) => unsafe {
                    Attribute::TexCoords(
                        set,
                        TexCoords::F32(accessor.iter()),
                    )
                },
                (&Semantic::TexCoord(set), U8, Vec2) => unsafe {
                    Attribute::TexCoords(
                        set,
                        TexCoords::U8(accessor.iter()),
                    )
                },
                (&Semantic::TexCoord(set), U16, Vec2) => unsafe {
                    Attribute::TexCoords(
                        set,
                        TexCoords::U16(accessor.iter()),
                    )
                },
                (&Semantic::Weight(set), U8, Vec4) => unsafe {
                    Attribute::Weights(
                        set,
                        Weights::U8(accessor.iter()),
                    )
                },
                (&Semantic::Weight(set), U16, Vec4) => unsafe {
                    Attribute::Weights(
                        set,
                        Weights::U16(accessor.iter()),
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

impl<'a, X: 'a + Extras> Iterator for IterPrimitives<'a, X> {
    type Item = Primitive<'a, X>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|primitive| {
            Primitive::from_raw(self.root, primitive)
        })                    
    }
}
