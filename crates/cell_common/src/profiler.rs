//! The Profiler used by the compiler.

#[cfg(feature = "profiler")]
mod profiler;

#[cfg(feature = "profiler")]
pub use profiler::*;

#[cfg(not(feature = "profiler"))]
mod nop;

#[cfg(not(feature = "profiler"))]
pub use nop::*;
