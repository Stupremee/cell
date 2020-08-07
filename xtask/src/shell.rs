//! A `run` macro to run commands in a simple shell like language.

use crate::env;
use anyhow::{anyhow, bail, Context, Result};
use std::{
    io::Write,
    process::{Command, Stdio},
};

/// Runs the given process in the current environment and
/// will print the current executing command to stdout
#[macro_export]
macro_rules! run {
    ($($expr:expr),*) => {
        run!($($expr),*; echo = true)
    };

    ($($expr:expr),* ; echo = $echo:expr) => {
        $crate::shell::run_process(format!($($expr),*), $echo, None)
    };

    ($($expr:expr),* ; |$stdin:expr) => {
        $crate::shell::run_process(format!($($expr),*), false, Some(($stdin).as_ref()))
    };
}

#[inline]
pub fn run_process(cmd: impl AsRef<str>, echo: bool, stdin: Option<&[u8]>) -> Result<String> {
    let cmd = cmd.as_ref();
    run_process_inner(cmd, echo, stdin)
        .with_context(|| format!("failed to execute command `{}`", cmd))
}

fn run_process_inner(cmd: &str, echo: bool, stdin: Option<&[u8]>) -> Result<String> {
    let mut args = cmd.split_whitespace();
    let binary = args
        .next()
        .ok_or(anyhow!("expected at least one argument"))?;
    let cwd = env::cwd();

    if echo {
        println!("> {}", cmd);
    }

    let mut cmd = Command::new(binary);
    cmd.args(args).current_dir(cwd).stderr(Stdio::inherit());

    let output = match stdin {
        None => cmd.stdin(Stdio::null()).output(),
        Some(stdin) => {
            cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
            let mut process = cmd.spawn()?;
            process.stdin.take().unwrap().write_all(stdin)?;
            process.wait_with_output()
        }
    }?;
    let stdout = String::from_utf8(output.stdout)?;
    if echo {
        println!("{}", stdout);
    }

    if !output.status.success() {
        bail!("{}", output.status)
    }
    Ok(stdout.trim().to_string())
}
