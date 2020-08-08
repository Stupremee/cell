//! A no-op profiler that is imported when the `profiler`
//! feature is disabled

/// A no-op method
pub fn trace(_category: &str, _label: &str) -> () {
    ()
}
