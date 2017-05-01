
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Buffer`, `BufferView`, and other related data structures.
pub mod buffer;

/// Contains the names of 2.0 extensions enabled and supported by the library.
pub mod extensions;

/// Contains convenience implementations of the `Extra` trait.
pub mod extras;

/// Contains the 'raw' versions of all glTF objects.
pub mod raw;

/// Contains functions for importing glTF 2.0 assets.
pub mod import;

/// Contains `Mesh` and other related data structures.
pub mod mesh;

/// Contains `Root`, the root glTF object.
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures.
pub mod scene;

/// Contains data structures associated with glTF validation.
pub mod validation;

pub use self::extras::Extras;
pub use self::import::{import, ImportError};
pub use self::root::Root;
pub use self::validation::Validate;

