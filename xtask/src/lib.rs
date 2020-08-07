//! Library for the xtask binary.
#![warn(rust_2018_idioms)]

pub mod ci;
mod env;
pub mod shell;

pub use env::{cwd, pushd, pushenv};

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// Returns the root of the `cell` project root.
pub fn project_root() -> Result<PathBuf> {
    Path::new(
        &std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .map(|path| path.to_path_buf())
    .ok_or(anyhow!("failed to get project root path"))
}
