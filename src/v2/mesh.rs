
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::collections::HashMap;
use v2::{accessor, material, Extras, Index, Root};

enum_number! {
    Mode {
        Points = 0,
        Lines = 1,
        LineLoop = 2,
        LineStrip = 3,
        Triangles = 4,
        TriangleStrip = 5,
        TriangleFan = 6,
    }
}

/// Extension specific data for `Mesh`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MeshExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh<E: Extras> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: MeshExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::Mesh,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// Defines the geometry to be renderered with a material.
    pub primitives: Vec<Primitive<E>>,

    /// Defines the weights to be applied to the morph targets.
    #[serde(default)]
    pub weights: Vec<f32>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Primitive<E: Extras> {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    #[serde(default)]
    pub attributes: HashMap<Semantic, Index<accessor::Accessor<E>>>,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: PrimitiveExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <E as Extras>::MeshPrimitive,
    
    /// The `Accessor` that contains the indices.
    pub indices: Option<Index<accessor::Accessor<E>>>,
    
    /// The material to apply to this primitive when rendering.
    pub material: Index<material::Material<E>>,
    
    /// The type of primitives to render.
    #[serde(default)]
    pub mode: Mode,
    
    /// Maps attribute names (only `"POSITION"` and `"NORMAL"`) to their
    /// deviations in the morph target.
    // TODO: Confirm that this the correct implementation.
    #[serde(default)]
    pub targets: Vec<HashMap<String, Index<accessor::Accessor<E>>>>,
}

/// Extension specific data for `Primitive`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PrimitiveExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Semantic {
    /// RGB(A) vertex color.
    Color(u32),
    
    /// User-defined vertex attribute.
    Extra(String),

    /// Joints
    Joint(u32),

    /// XYZ vertex normal.
    Normal,

    /// XYZ vertex position.
    Position,

    /// XYZW vertex tangent, where W determines the handedness of the tangent.
    Tangent,

    /// UV texture co-ordinate.
    TexCoord(u32),

    /// Weight.
    Weight(u32),
}

impl<E: Extras> Mesh<E> {
    #[doc(hidden)]
    pub fn range_check(&self, root: &Root<E>) -> Result<(), ()> {
        for primitive in &self.primitives {
            for accessor in primitive.attributes.values() {
                let _ = root.try_get(accessor)?;
            }
            if let Some(ref indices) = primitive.indices {
                let _ = root.try_get(indices)?;
            }
            let _ = root.try_get(&primitive.material)?;
            for map in &primitive.targets {
                for accessor in map.values() {
                    let _ = root.try_get(accessor)?;
                }
            }
        }
        Ok(())
    }

    #[doc(hidden)]
    pub fn size_check(&self, root: &Root<E>) -> Result<(), ()> {
        for primitive in &self.primitives {
            let _ = primitive.size_check(root)?;
        }
        Ok(())
    }
}

impl<E: Extras> Primitive<E> {
    #[doc(hidden)]
    pub fn size_check(&self, root: &Root<E>) -> Result<(), ()> {
        use self::Semantic::*;
        use v2::accessor::ComponentType::*;
        use v2::accessor::Kind::*;
        for (semantic, index) in self.attributes.iter() {
            // N.B. Calling `try_get()` here instead of `get()` eliminates the
            // need to call `range_check()` beforehand.
            let accessor = root.try_get(index)?;
            let ty = (accessor.component_type, accessor.kind);
            match semantic {
                &Color(_) => match ty {
                    (U8, Vec3)
                        | (U8, Vec4)
                        | (U16, Vec3)
                        | (U16, Vec4)
                        | (F32, Vec3)
                        | (F32, Vec4) => {},
                    _ => return Err(()),
                },
                &Extra(_) => {},
                &Joint(_) | &Weight(_) => match ty {
                    (U8, Vec4) | (U16, Vec4) => {},
                    _ => return Err(()),
                },
                &Normal | &Position => match ty {
                    (F32, Vec3) => {},
                    _ => return Err(()),
                },
                &Tangent => match ty {
                    (F32, Vec4) => {},
                    _ => return Err(()),
                },
                &TexCoord(_) => match ty {
                    (U8, Vec2) | (U16, Vec2) | (F32, Vec2) => {},
                    _ => return Err(()),
                },
            }
        }
        Ok(())
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Triangles
    }
}

impl std::str::FromStr for Semantic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Semantic::*;
        match s {
            "NORMAL" => Ok(Normal),
            "POSITION" => Ok(Position),
            "TANGENT" => Ok(Tangent),
            _ if s.starts_with("COLOR_") => {
                let set = s["COLOR_".len()..].parse().map_err(|_| ())?;
                Ok(Color(set))
            },
            _ if s.starts_with("TEXCOORD_") => {
                let set = s["TEXCOORD_".len()..].parse().map_err(|_| ())?;
                Ok(TexCoord(set))
            },
            _ if s.starts_with("JOINT_") => {
                let set = s["JOINT_".len()..].parse().map_err(|_| ())?;
                Ok(Joint(set))
            },
            _ if s.starts_with("WEIGHT_") => {
                let set = s["WEIGHT_".len()..].parse().map_err(|_| ())?;
                Ok(Weight(set))
            },
            other if s.starts_with("_") => Ok(Extra(other.to_string())),
            _ => Err(()),
        }

    }
}

impl ::serde::de::Deserialize for Semantic {
    fn deserialize<D>(deserializer: D) -> Result<Semantic, D::Error>
        where D: ::serde::de::Deserializer
    {
        struct Visitor;              
        impl ::serde::de::Visitor for Visitor {
            type Value = Semantic;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter)
                         -> ::std::fmt::Result
            {
                let _ = formatter.write_str(concat!("<semantic>[_set]\n"))?;
                Ok(())
            }

            fn visit_str<E>(self, value: &str)-> Result<Self::Value, E>
                where E: ::serde::de::Error
            {
                match value.parse() {
                    Ok(semantic) => Ok(semantic),
                    Err(()) => {
                        let msg = format!("invalid semantic \"{}\"", value);
                        Err(E::custom(msg))
                    },
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ::serde::ser::Serialize for Semantic {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where S: ::serde::ser::Serializer
    {
        use self::Semantic::*;
        match self {
            &Color(set) => s.serialize_str(&format!("COLOR_{}", set)),
            &Extra(ref semantic) => s.serialize_str(semantic),
            &Joint(set) => s.serialize_str(&format!("JOINT_{}", set)),
            &Normal => s.serialize_str("NORMAL"),
            &Position => s.serialize_str("POSITION"),
            &Tangent => s.serialize_str("TANGENT"),
            &TexCoord(set) => s.serialize_str(&format!("TEXCOORD_{}", set)),
            &Weight(set) => s.serialize_str(&format!("WEIGHT_{}", set)),
        }
    }
}
