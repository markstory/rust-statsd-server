use buckets::Buckets;
use super::console;

/// Defines the interface that backends use to publish
/// metrics to their storage system.
pub trait Backend {
    /// This method should flush the current data to the backend.
    ///
    /// Called on server `flush` events, which occur on a timer
    /// (every 10 seconds by default).
    fn flush_buckets(&mut self, buckets: &Buckets) -> ();
}


/// Creates the collection of backends based on the paraemeters
///
pub fn factory(console: &bool, graphite: &bool) -> Box<[console::Console]> {
    let mut backends = Vec::with_capacity(22);
    if *console {
        backends.push(console::Console::new());
    }
    backends.into_boxed_slice()
}
