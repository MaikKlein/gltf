
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_json;
use std;
use v2::{raw, root, validation, Extras, Root, Validate};

/// Error encountered when importing a glTF 2.0 asset.
#[derive(Debug)]
pub enum ImportError {
    /// Failure when deserializing a .gltf metadata file.
    Deserialize(serde_json::error::Error),
    
    /// A glTF extension required by the asset has not been enabled by the user.
    ExtensionDisabled(String),
    
    /// A glTF extension required by the asset is not supported by the library.
    ExtensionUnsupported(String),
    
    /// The .gltf data is invalid.
    Invalid(String),
    
    /// Standard input / output error.
    Io(std::io::Error),
    
    /// The glTF version of the asset is incompatible with this function.
    IncompatibleVersion(String),

    /// Error encountered when loading the glTF data.
    Load(root::LoadError),

    /// Error encountered when validating glTF.
    Validation(Vec<validation::Error>),
}

/// Imports a standard (plain text JSON) glTF 2.0 asset.
fn import_standard_gltf<'a, X: 'a + Extras>(
    data: Vec<u8>,
) -> Result<raw::root::Root<X>, ImportError> {
    let raw: raw::root::Root<X> = serde_json::from_slice(&data)?;
    Ok(raw)
}

/// Imports a glTF 2.0 asset.
pub fn import<P, X>(path: P) -> Result<Root<X>, ImportError>
    where P: AsRef<std::path::Path>, X: Extras
{
    use std::io::Read;
    use self::ImportError::*;

    let mut file = std::fs::File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let raw: raw::root::Root<X> = if buffer.starts_with(b"glTF") {
        return Err(ExtensionUnsupported("Binary glTF 2.0".to_string()));
    } else {
        file.read_to_end(&mut buffer)?;
        import_standard_gltf(buffer)?
    };

    let root = Root::load(raw, path)?;
    let mut errs = Vec::new();
    {
        let warn_fn = |source: &str, description: &str| {
            // TODO: Do something more useful with this data.
            println!("warning: {}: {}", source, description);
        };
        let err_fn = |source: &str, description: &str| {
            errs.push(validation::Error {
                source: source.to_string(),
                description: description.to_string(),
            });
        };
        root.validate(&root, warn_fn, err_fn);
    }
    if errs.is_empty() {
        Ok(root)
    } else {
        Err(Validation(errs))
    }
}
 
impl From<root::LoadError> for ImportError {
    fn from(err: root::LoadError) -> ImportError {
        ImportError::Load(err)
    }
}

impl From<serde_json::Error> for ImportError {
    fn from(err: serde_json::Error) -> ImportError {
        ImportError::Deserialize(err)
    }
}

impl From<std::io::Error> for ImportError {
    fn from(err: std::io::Error) -> ImportError {
        ImportError::Io(err)
    }
}
