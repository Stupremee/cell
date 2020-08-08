//! `cargo xtask` command workflows.
//!
//! See https://github.com/matklad/cargo-xtask/
//!
//! [`sda`]: askdjas
#![warn(rust_2018_idioms)]

use anyhow::Result;
use pico_args::Arguments;
use xtask::{ci, codegen, project_root, pushd};

const HELP: &str = "\
cargo xtask

Run custom build commands.

USAGE:
    cargo xtask <COMMAND>

COMMANDS:
    ci         Runs everything that is tested in the CI.
    codegen    Pre generate all required source files.
";

fn main() -> Result<()> {
    let _root = pushd(project_root()?);

    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    if args.contains("--help") {
        args.finish()?;
        print!("{}", HELP);
        return Ok(());
    }

    match subcommand.as_str() {
        "ci" => {
            let not_miri = args.contains("--no-miri");
            args.finish()?;
            ci::run_ci(!not_miri)?;
        }
        "codegen" => {
            args.finish()?;
            codegen::generate_syntax()?;
        }
        _ => print!("{}", HELP),
    }

    Ok(())
}
