//! Implementation of the CI subcommand.

use crate::{project_root, pushd, run};
use anyhow::{bail, Context, Result};

/// Runs every step that should be run on the CI.
pub fn run_ci(miri: bool) -> Result<()> {
    let _d = pushd(project_root()?);

    run!("cargo fmt -- --check").context("rustfmt failed")?;
    run!("cargo clippy --all-features --all-targets --tests").context("clippy failed")?;
    run!("cargo test").context("tests failed")?;

    if miri {
        run_miri()?;
    }

    Ok(())
}

fn run_miri() -> Result<()> {
    if run!("cargo miri --help"; echo = false).is_err() {
        bail!(
            "Failed to run miri.\
               Run 'rustup component add miri' to install miri"
        )
    }
    run!("cargo miri test")?;
    Ok(())
}
