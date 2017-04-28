
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
    
    /// Maps attribute names (only `"POSITION"`, `"NORMAL"`, and `"TANGENT"`) to
    /// their deviations in the morph target.
    #[serde(default)]
    pub targets: Vec<HashMap<Semantic, Index<accessor::Accessor<E>>>>,
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
    pub fn validate<Fw: FnMut(&str, &str), Fe: FnMut(&str, &str)>(
        &self,
        root: &Root<E>,
        mut warn: Fw,
        mut err: Fe,
    ) {
        for (i, primitive) in self.primitives.iter().enumerate() {
            let warn_fn = |source: &str, description: &str| {
                let source = format!("primitive[{}].{}", i, source);
                warn(&source, description);
            };
            let err_fn = |source: &str, description: &str| {
                let source = format!("primitive[{}].{}", i, source);
                err(&source, description);
            };
            primitive.validate(root, warn_fn, err_fn);
        }
    }
}

impl<E: Extras> Primitive<E> {  
    #[doc(hidden)]
    pub fn validate<Fw: FnMut(&str, &str), Fe: FnMut(&str, &str)>(
        &self,
        root: &Root<E>,
        mut warn: Fw,
        mut err: Fe,
    ) {
        use self::Semantic::*;
        use v2::accessor::ComponentType::*;
        use v2::accessor::Kind::*;
        for (semantic, accessor_index) in &self.attributes {
            let source = format!("attribute[{}]", semantic.to_string());
            let accessor = root.try_get(accessor_index);
            if accessor.is_err() {
                err(&source, "Index out of range");
                return;
            }
            let accessor = accessor.unwrap();
            let ty = (accessor.component_type, accessor.kind);
            match semantic {
                &Color(_) => match ty {
                    (U8, Vec3)
                        | (U8, Vec4)
                        | (U16, Vec3)
                        | (U16, Vec4)
                        | (F32, Vec3)
                        | (F32, Vec4) => {},
                    _ => err(&source, "Invalid accessor for attribute COLOR_*"),
                },
                &Extra(ref name) if !name.starts_with("_") => {
                    warn(&source, "User defined attributes should start with an underscore (`_`)");
                },
                &Extra(_) => {},
                &Joint(_) => match ty {
                    (U8, Vec4) | (U16, Vec4) => {},
                    _ => err(&source, "Invalid accessor for attribute JOINT_*"),
                },
                &Normal => match ty {
                    (F32, Vec3) => {},
                    _ => err(&source, "Invalid accessor for attribute NORMAL"),
                },
                &Position => match ty {
                    (F32, Vec3) => {},
                    _ => err(&source, "Invalid accessor for attribute POSITION"),
                },
                &Tangent => match ty {
                    (F32, Vec4) => {},
                    _ => err(&source, "Invalid accessor for attribute TANGENT"),
                },
                &TexCoord(_) => match ty {
                    (U8, Vec2) | (U16, Vec2) | (F32, Vec2) => {},
                    _ => err(&source, "Invalid accessor for attribute TEXCOORD_*"),
                },
                &Weight(_) => match ty {
                    (U8, Vec4) | (U16, Vec4) => {},
                    _ => err(&source, "Invalid accessor for attribute WEIGHT_*"),
                },
            }
        }

        if let Some(indices) = self.indices.as_ref() {
            match root.try_get(indices) {
                Ok(accessor) => {
                    let ty = (accessor.component_type, accessor.kind);
                    match ty {
                        (U8, Scalar) | (U16, Scalar) | (U32, Scalar) => {},
                        _ => err("indices", "Invalid accessor"),
                    }
                }
                Err(_) => err("indices", "Index out of range"),
            }
        }

        if let Err(_) = root.try_get(&self.material) {
            err("material", "Index out of range");
        }
        for (i, map) in self.targets.iter().enumerate() {
            for (semantic, accessor) in map {
                let source = format!("targets[{}][{}]", i, semantic.to_string());
                if let Err(_) = root.try_get(accessor) {
                    err(&source, "Index out of range");
                }
                match semantic {
                    &Normal | &Position | &Tangent => {},
                    _ => err(&source, "Invalid attribute for Morph target"),
                }
            }
        }
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
        let semantic = match s {
            "NORMAL" => Normal,
            "POSITION" => Position,
            "TANGENT" => Tangent,
            _ if s.starts_with("COLOR_") => {
                let set = s["COLOR_".len()..].parse().map_err(|_| ())?;
                Color(set)
            },
            _ if s.starts_with("TEXCOORD_") => {
                let set = s["TEXCOORD_".len()..].parse().map_err(|_| ())?;
                TexCoord(set)
            },
            _ if s.starts_with("JOINT_") => {
                let set = s["JOINT_".len()..].parse().map_err(|_| ())?;
                Joint(set)
            },
            _ if s.starts_with("WEIGHT_") => {
                let set = s["WEIGHT_".len()..].parse().map_err(|_| ())?;
                Weight(set)
            },
            other => Extra(other.to_string()),
        };
        Ok(semantic)
    }
}

impl std::string::ToString for Semantic {
    fn to_string(&self) -> String {
        use self::Semantic::*;
        match self {
            &Color(set) => format!("COLOR_{}", set),
            &Extra(ref semantic) => semantic.clone(),
            &Joint(set) => format!("JOINT_{}", set),
            &Normal => "NORMAL".to_string(),
            &Position => "POSITION".to_string(),
            &Tangent => "TANGENT".to_string(),
            &TexCoord(set) => format!("TEXCOORD_{}", set),
            &Weight(set) => format!("WEIGHT_{}", set),
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
        s.serialize_str(&self.to_string())
    }
}
