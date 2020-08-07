//! Module for bash like functions to execute commands
//! using `popd` and `pushd` and a general `run!` macro
//! to execute commands.

use std::{
    cell::RefCell,
    env,
    ffi::OsString,
    path::{Path, PathBuf},
};

/// Returns the path at the top of the stack.
pub fn cwd() -> PathBuf {
    Env::with(|env| env.cwd().into())
}

/// Object that will pop the pushed path if dropped.
pub struct Pushd {
    _private: (),
}

/// Pushes the given path onto the directory stack and set the cwd to the path.
///
/// The path will be dropped as soon as the returned object is dropped.
pub fn pushd(path: impl Into<PathBuf>) -> Pushd {
    Env::with(|env| {
        env.pushd(path.into());
        Pushd { _private: () }
    })
}

impl Drop for Pushd {
    fn drop(&mut self) {
        Env::with(|env| env.popd())
    }
}

/// Object that will pop the pushed environment variable if dropped.
pub struct Pushenv {
    _private: (),
}

/// Pushes the given environemnt variable (name and value) onto the env stack
/// and set the environment variable.
pub fn pushenv(var: &str, val: &str) -> Pushenv {
    Env::with(|env| {
        env.pushenv(var.into(), val.into());
        Pushenv { _private: () }
    })
}

impl Drop for Pushenv {
    fn drop(&mut self) {
        Env::with(|env| env.popenv())
    }
}

/// The environment that has two stacks, for environment variables
/// and a path stack.
struct Env {
    path_stack: Vec<PathBuf>,
    env_stack: Vec<(OsString, Option<OsString>)>,
}

impl Env {
    fn new() -> Self {
        Self {
            path_stack: vec![env::current_dir().unwrap()],
            env_stack: vec![],
        }
    }

    fn with<F: FnOnce(&mut Env) -> T, T>(action: F) -> T {
        thread_local! {
            static ENV: RefCell<Env> = RefCell::new(Env::new());
        }
        ENV.with(|env| action(&mut *env.borrow_mut()))
    }

    fn pushd(&mut self, path: PathBuf) {
        let dir = self.cwd().join(path);
        self.path_stack.push(dir);
        self.set_cwd();
    }

    fn popd(&mut self) {
        self.path_stack.pop().unwrap();
        self.set_cwd();
    }

    fn pushenv(&mut self, var: OsString, val: OsString) {
        self.env_stack.push((var.clone(), env::var_os(&var)));
        env::set_var(var, val)
    }

    fn popenv(&mut self) {
        let (var, val) = self.env_stack.pop().unwrap();
        match val {
            None => env::remove_var(var),
            Some(val) => env::set_var(var, val),
        }
    }

    fn set_cwd(&self) {
        env::set_current_dir(self.cwd())
            .unwrap_or_else(|err| panic!("failed to set cwd to {}: {}", self.cwd().display(), err));
    }

    fn cwd(&self) -> &Path {
        self.path_stack.last().unwrap()
    }
}
