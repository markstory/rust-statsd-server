use buckets::Buckets;

/// Defines the interface that backends use to publish
/// metrics to their storage system.
pub trait Backend {
    /// This method should flush the current data to the backend.
    ///
    /// Called on server `flush` events, which occur on a timer
    /// (every 10 seconds by default).
    fn flush_buckets(&mut self, &Buckets) -> ();
}
