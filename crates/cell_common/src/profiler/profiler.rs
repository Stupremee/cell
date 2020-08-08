//! Profiler implementation using [`measureme`].
//!
//! [`measureme`]: https://docs.rs/measureme

use measureme::{EventId, TimingGuard};
use once_cell::sync::Lazy;
use std::{path::Path, thread::ThreadId};

/// MmapSerializatioSink is faster on macOS and Linux
/// but FileSerializationSink is faster on Windows
#[cfg(not(windows))]
type Sink = measureme::MmapSerializationSink;
#[cfg(windows)]
type Sink = measureme::FileSerializationSink;

/// The global profiler.
static PROFILER: Lazy<Profiler> = Lazy::new(Profiler::default);

/// Starts tracing an event which will end if the returned guard is dropped.
pub fn trace(category: &str, label: &str) -> TimingGuard<'static, Sink> {
    PROFILER.trace(category.as_ref(), label.as_ref())
}

/// Wrapper around a [`measureme::Profiler`].
pub struct Profiler {
    profiler: measureme::Profiler<Sink>,
}

impl Profiler {
    /// Creates a new `Profiler` with the default path (`./trace/cell-<pid>`).
    fn default() -> Self {
        let path = format!("./trace/cell-{}", std::process::id());
        let path = Path::new(&path);
        Self {
            profiler: measureme::Profiler::new(path).expect("failed to create profiler"),
        }
    }

    /// Starts profiling an event with the given `category` and `label`.
    fn trace(&self, category: &str, label: &str) -> TimingGuard<'_, Sink> {
        let kind = self.profiler.alloc_string(category);

        let label = self.profiler.alloc_string(label);
        let id = EventId::from_label(label);
        let thread_id = current_thread_id() as u32;

        self.profiler
            .start_recording_interval_event(kind, id, thread_id)
    }
}

/// Gets the current thread id and transmutes it into a
/// `u64`.
fn current_thread_id() -> u64 {
    // TODO: Remove unsafe if https://github.com/rust-lang/rust/issues/67939 is resolved.
    let tid = std::thread::current().id();
    unsafe { std::mem::transmute::<ThreadId, u64>(tid) }
}
