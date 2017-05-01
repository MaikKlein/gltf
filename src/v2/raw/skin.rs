
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{raw, Extras, Root, Validate};
use v2::raw::root::Index;

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Skin<X: Extras> {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SkinExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Skin,
    
    /// The index of the accessor containing the 4x4 inverse-bind matrices.
    ///
    /// When `None`,each matrix is assumed to be the 4x4 identity matrix
    /// which implies that the inverse-bind matrices were pre-applied.
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<Index<raw::accessor::Accessor<X>>>,
    
    /// Indices of skeleton nodes used as joints in this skin.
    ///
    /// The array length must be the same as the `count` property of the
    /// `inverse_bind_matrices` `Accessor` (when defined).
    pub joints: Vec<Index<raw::scene::Node<X>>>,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// The index of the node used as a skeleton root.
    ///
    /// When `None`, joints transforms resolve to scene root.
    pub skeleton: Option<Index<raw::scene::Node<X>>>,
}

/// Extension specific data for `Skin`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SkinExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<X: Extras> Validate<X> for Skin<X> {
    fn validate<W, E>(&self, root: &Root<X>, _warn: W, mut err: E)
        where W: FnMut(&str, &str), E: FnMut(&str, &str)
    {
        if let Some(accessor) = self.inverse_bind_matrices.as_ref() {
            if let Err(_) = root.try_get(accessor) {
                err("accessor", "Index out of range");
            }
        }
        for (i, joint) in self.joints.iter().enumerate() {
            if let Err(_) = root.try_get(joint) {
                let source = format!("joints[{}]", i);
                err(&source, "Index out of range");
            }
        }
        if let Some(node) = self.skeleton.as_ref() {
            if let Err(_) = root.try_get(node) {
                err("skeleton", "Index out of range");
            }
        }
    }
}
