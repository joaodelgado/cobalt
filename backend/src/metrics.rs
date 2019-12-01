use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

pub struct Monitoring {
    registry: Registry,
    raw_counts: Counter,
}

impl Monitoring {
    pub fn new() -> Monitoring {
        // Create a Counter.
        let opts = Opts::new(
            "raw_counts",
            "Counter with raw counts from the Geiger counter",
        );
        let c = Counter::with_opts(opts).unwrap();

        // Create a Registry and register Counter.
        let r = Registry::new();
        r.register(Box::new(c.clone())).unwrap();

        Monitoring {
            registry: r,
            raw_counts: c,
        }
    }

    pub fn register_count(&self) {
        self.raw_counts.inc()
    }

    pub fn report(&self) -> String {
        // Gather the metrics.
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        // Output to the standard output.
        String::from_utf8(buffer).unwrap()
    }
}
