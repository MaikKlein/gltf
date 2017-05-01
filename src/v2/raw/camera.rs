
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{Extras, Root, Validate};

enum_string! {
    CameraType {
        Orthographic = "orthographic",
        Perspective = "perspective",
    }
}
    
/// A camera's projection.
///
/// A node can reference a camera to apply a transform to place the camera in the
/// scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Camera<X: Extras> {
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// An orthographic camera containing properties to create an orthographic
    /// projection matrix.
    pub orthographic: Option<Orthographic<X>>,

    /// A perspective camera containing properties to create a perspective
    /// projection matrix.
    pub perspective: Option<Perspective<X>>,

    /// Specifies if the camera uses a perspective or orthographic projection.
    #[serde(rename = "type")]
    pub ty: CameraType,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: CameraExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::Camera,
}

/// Extension specific data for `Camera`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CameraExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Values for an orthographic camera.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Orthographic<X: Extras> {
    /// The horizontal magnification of the view.
    #[serde(default)]
    pub xmag: f32,

    /// The vertical magnification of the view.
    #[serde(default)]
    pub ymag: f32,

    /// The distance to the far clipping plane.
    #[serde(default)]
    pub zfar: f32,

    /// The distance to the near clipping plane.
    #[serde(default)]
    pub znear: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: OrthographicExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::CameraOrthographic,
}

/// Extension specific data for `Orthographic`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrthographicExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

/// Values for a perspective camera.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Perspective<X: Extras> {
    /// Aspect ratio of the field of view.
    #[serde(default)]
    pub aspect_ratio: f32,

    /// The vertical field of view in radians.
    #[serde(default)]
    pub yfov: f32,

    /// The distance to the far clipping plane.
    #[serde(default)]
    pub zfar: f32,

    /// The distance to the near clipping plane.
    #[serde(default)]
    pub znear: f32,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: PerspectiveExtensions,

    /// Optional application specific data.
    #[serde(default)]
    pub extras: <X as Extras>::CameraPerspective,
}

/// Extension specific data for `Perspective`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PerspectiveExtensions {
    #[serde(default)]
    _allow_extra_fields: (),
}

impl<X: Extras> Validate<X> for Camera<X> {
    fn validate<W, E>(&self, _root: &Root<X>, mut warn: W, mut err: E)
        where W: FnMut(&str, &str), E: FnMut(&str, &str)
    {
        match self.ty {
            CameraType::Orthographic => {
                if self.orthographic.is_none() {
                    err("orthographic", "Missing data");
                }
                if self.perspective.is_some() {
                    warn("perspective", "Redundant data");
                }
            }
            CameraType::Perspective => {
                if self.perspective.is_none() {
                    err("perspective", "Missing data");
                }
                if self.orthographic.is_some() {
                    warn("orthographic", "Redundant data");
                }
            }
        }
    }
}
