use std::time::{Duration, Instant};

use log::info;
use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

use super::geiger;

pub struct Monitoring {
    registry: Registry,
    raw_counts: Counter,

    // Use for local debugging.
    // Retains only the count from the last minute
    local_counts: Vec<Instant>,
}

impl Monitoring {
    pub fn new() -> Monitoring {
        // Create a Counter.
        let opts = Opts::new(
            "geiger_raw_counts",
            "Counter with raw counts from the Geiger counter",
        );
        let c = Counter::with_opts(opts).unwrap();

        // Create a Registry and register Counter.
        let r = Registry::new();
        r.register(Box::new(c.clone())).unwrap();

        Monitoring {
            registry: r,
            raw_counts: c,
            local_counts: Vec::new(),
        }
    }

    pub fn register_count(&mut self) {
        self.raw_counts.inc();
        self.local_counts.push(Instant::now());
    }

    pub fn prometheus_export(&self) -> String {
        // Gather the metrics.
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        // Output to the standard output.
        String::from_utf8(buffer).unwrap()
    }

    pub fn report(&mut self) {
        let minute = Duration::from_secs(60);

        self.local_counts.retain(|t| t.elapsed() <= minute);

        info!(
            "{} CPM\t{:.4} μSv/h",
            self.local_counts.len(),
            geiger::Geiger::cpm_to_microsieverts(self.local_counts.len() as u32),
        );
    }
}
