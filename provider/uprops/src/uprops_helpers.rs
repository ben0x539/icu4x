// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader::*;

use crate::uprops_serde;
use eyre::{eyre, WrapErr};
use std::collections::HashMap;
use std::path::Path;
use tinystr::TinyStr16;

pub type TomlEnumerated = HashMap<TinyStr16, uprops_serde::enumerated::EnumeratedPropertyMap>;
pub type TomlBinary = HashMap<TinyStr16, uprops_serde::binary::BinaryProperty>;

pub fn load_binary_from_dir(root_dir: &Path) -> eyre::Result<TomlBinary> {
    let mut result = HashMap::new();
    for path in get_dir_contents(root_dir)? {
        let option_key: Option<TinyStr16> = path
            .file_stem()
            .and_then(|p| p.to_str())
            .ok_or_else(|| eyre::eyre!("Invalid file name: {:?}", path))?
            .parse()
            .ok();
        let key = if let Some(k) = option_key {
            k
        } else {
            #[cfg(feature = "log")]
            log::trace!("Filename does not fit in TinyStr16: {:?}", path);
            continue;
        };
        let toml_str = read_path_to_string(&path)?;
        let toml_obj: uprops_serde::binary::Main = toml::from_str(&toml_str)
            .wrap_err_with(|| format!("Could not parse TOML: {:?}", path))?;
        if let Some(v) = toml_obj.binary_property.into_iter().next() {
            result.insert(key, v);
        }
    }
    Ok(result)
}

pub fn load_enumerated_from_dir(root_dir: &Path) -> eyre::Result<TomlEnumerated> {
    let mut result = HashMap::new();
    for path in get_dir_contents(root_dir)? {
        let option_key: Option<TinyStr16> = path
            .file_stem()
            .and_then(|p| p.to_str())
            .ok_or_else(|| eyre::eyre!("Invalid file name: {:?}", path))?
            .parse()
            .ok();
        let key = if let Some(k) = option_key {
            k
        } else {
            #[cfg(feature = "log")]
            log::trace!("Filename does not fit in TinyStr16: {:?}", path);
            continue;
        };
        let toml_str = read_path_to_string(&path)?;
        let toml_obj: uprops_serde::enumerated::Main = toml::from_str(&toml_str)
            .wrap_err_with(|| format!("Could not parse TOML: {:?}", path))?;
        if let Some(v) = toml_obj.enum_property.into_iter().next() {
            result.insert(key, v);
        }
    }
    Ok(result)
}

pub fn load_script_extensions_from_dir(
    root_dir: &Path,
) -> eyre::Result<uprops_serde::script_extensions::ScriptExtensionsProperty> {
    let mut path = root_dir.join("scx");
    path.set_extension("toml");
    let toml_str = read_path_to_string(&path)?;
    let toml_obj: uprops_serde::script_extensions::Main =
        toml::from_str(&toml_str).wrap_err_with(|| format!("Could not parse TOML: {:?}", path))?;

    toml_obj
        .script_extensions
        .into_iter()
        .next()
        .ok_or_else(|| {
            eyre!(
                "Could not parse Script_Extensions data from TOML {:?}",
                path
            )
        })
}
