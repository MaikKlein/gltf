
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use v2::{Extras, Root};

/// Trait for validating glTF data post-deserialization.
pub trait Validate<X: Extras> {
    /// Validates the data.
    fn validate<W, E>(&self, root: &Root<X>, warn: W, err: E)
        where W: FnMut(&str, &str), E: FnMut(&str, &str);
}

/// Source and description of malformed glTF.
#[derive(Clone, Debug)]
pub struct Error {
    /// The name of the field the error originates from.
    ///
    /// For example, if the `buffer_view` member of the second `Accessor` was out
    /// of range, then source would be `buffer_view`.
    pub source: String,

    /// A short description of the detected error condition.
    pub description: String,
}

impl std::fmt::Display for self::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.source, self.description)
    }
}

impl std::error::Error for self::Error {
    fn description(&self) -> &str {
        &self.description
    }
}
