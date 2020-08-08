//! Library for the xtask binary.
#![warn(rust_2018_idioms)]

mod ast;
pub mod ci;
pub mod codegen;
mod env;
pub mod shell;
pub mod util;

pub use env::{cwd, pushd, pushenv};

use anyhow::{anyhow, bail, Result};
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

/// Formats the given text using rustfmt.
fn reformat(text: impl std::fmt::Display) -> Result<String> {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    let stdout = run!(
        "rustfmt --config-path {} --config fn_single_line=true", project_root()?.join("rustfmt.toml").display();
        |text.to_string().as_bytes()
    )?;
    let preamble = "Generated file, do not edit by hand, see `xtask/src/codegen`";
    Ok(format!("//! {}\n\n{}\n", preamble, stdout))
}

fn ensure_rustfmt() -> Result<()> {
    let out = run!("rustfmt --version"; echo = false)?;
    if !out.contains("stable") {
        bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
    Ok(())
}
