
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::{raw, Extras};

pub use self::raw::camera::{
    CameraExtensions,
    CameraType,
    Orthographic,
    OrthographicExtensions,
    Perspective,
    PerspectiveExtensions,
};

#[derive(Debug)]
pub enum Camera<'a, X: 'a + Extras> {
    /// Orthographic projection.
    Orthographic(&'a Orthographic<X>),
    /// Perspective projection.
    Perspective(&'a Perspective<X>),
}

impl<'a, X: Extras> Camera<'a, X> {
    /// Constructor for a `Camera`.
    pub fn from_raw(raw: &'a raw::camera::Camera<X>) -> Self {
        match raw.ty {
            CameraType::Orthographic => {
                Camera::Orthographic(raw.orthographic.as_ref().unwrap())
            },
            CameraType::Perspective => {
                Camera::Perspective(raw.perspective.as_ref().unwrap())
            },
        }
    }
}

