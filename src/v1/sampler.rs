// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v1::texture::Filter;
use v1::texture::Wrap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Sampler {
    /// Magnification filter.
    #[serde(rename = "magFilter")]
    #[serde(default = "sample_mag_filter_default")]
    pub mag_filter: Filter,

    /// Minification filter.
    #[serde(rename = "minFilter")]
    #[serde(default = "sample_min_filter_default")]
    pub min_filter: Filter,

    /// s wrapping mode.
    #[serde(rename = "wrapS")]
    #[serde(default = "sample_wrap_s_default")]
    pub wrap_s: Wrap,

    /// t wrapping mode.
    #[serde(rename = "wrapT")]
    #[serde(default = "sample_wrap_t_default")]
    pub wrap_t: Wrap,

    pub name: Option<String>,
}

fn sample_mag_filter_default() -> Filter {
    Filter::Linear
}

fn sample_min_filter_default() -> Filter {
    Filter::NearestMipmapLinear
}

fn sample_wrap_s_default() -> Wrap {
    Wrap::Repeat
}

fn sample_wrap_t_default() -> Wrap {
    Wrap::Repeat
}
