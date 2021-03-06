// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum_string! {
    CameraType {
        Orthographic = "orthographic",
        Perspective = "perspective",
    }
}

impl Default for CameraType {
    fn default() -> CameraType {
        CameraType::Perspective
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Perspective {
    /// The floating-point aspect ratio of the field of view.
    ///
    /// When this is undefined, the aspect ratio of the canvas is used.
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: Option<f32>,

    /// The floating-point vertical field of view in radians.
    #[serde(rename = "yfov")]
    pub y_fov: f32,

    /// The floating-point distance to the far clipping plane.
    ///
    /// `z_far` must be greater than `z_near`.
    #[serde(rename = "zfar")]
    pub z_far: f32,
    /// The floating-point distance to the near clipping plane.
    ///
    /// `z_far` must be greater than `z_near`.
    #[serde(rename = "znear")]
    pub z_near: f32,

    // TODO: extension
    // TODO: extras
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Orthographic {
    /// The floating-point horizontal magnification of the view.
    #[serde(rename = "xmag")]
    pub x_mag: f32,

    /// The floating-point vertical magnification of the view.
    #[serde(rename = "ymag")]
    pub y_mag: f32,

    /// The floating-point distance to the far clipping plane.
    #[serde(rename = "zfar")]
    pub z_far: f32,

    /// The floating-point distance to the near clipping plane.
    #[serde(rename = "znear")]
    pub z_near: f32,

    // TODO: extension
    // TODO: extras
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Camera {
    /// An orthographic camera containing properties to create an orthographic
    /// projection matrix.
    pub orthographic: Option<Orthographic>,

    /// A perspective camera containing properties to create a perspective
    /// projection matrix.
    pub perspective: Option<Perspective>,

    /// Specifies if the camera uses a perspective or orthographic projection.
    ///
    /// Based on this, either the camera's perspective or orthographic property
    /// will be defined.
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: CameraType,

    /// The user-defined name of this object.
    ///
    /// This is not necessarily unique, e.g., a camera and a buffer could have
    /// the same name, or two cameras could even have the same name.
    pub name: Option<String>,

    // TODO: extension
    // TODO: extras
}
