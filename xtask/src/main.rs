//! `cargo xtask` command workflows.
//!
//! See https://github.com/matklad/cargo-xtask/
#![warn(rust_2018_idioms)]

mod env;

use anyhow::{bail, Context, Result};
use pico_args::Arguments;

fn main() -> Result<()> {
    let args = Arguments::from_env();

    Ok(())
}
