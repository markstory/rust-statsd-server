use buckets::Buckets;
use backends::console;
use backends::graphite;

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
pub fn factory(
    console: &bool,
    graphite: &bool,
    graphite_host: &str,
    graphite_port: &u16
) -> Box<[Box<Backend>]> {
    let mut backends: Vec<Box<Backend>> = Vec::with_capacity(2);
    if *console {
        backends.push(Box::new(console::Console::new()));
    }
    if *graphite {
        backends.push(Box::new(graphite::Graphite::new(graphite_host, *graphite_port)));
    }
    backends.into_boxed_slice()
}


#[cfg(test)]
mod test {
    use std::boxed;
    use super::*;
    use super::super::backends::graphite;

    #[test]
    fn factory_makes_graphite() {
        let backends = factory(&false, &true, "127.0.0.1", &2300);
        assert_eq!(1, backends.len());
    }

    #[test]
    fn factory_makes_console() {
        let backends = factory(&true, &false, "127.0.0.1", &2300);
        assert_eq!(1, backends.len());
    }

    #[test]
    fn factory_makes_both() {
        let backends = factory(&true, &true, "127.0.0.1", &2300);
        assert_eq!(2, backends.len());
    }
}
