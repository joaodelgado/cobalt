mod geiger;
mod metrics;

use std::env;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use env_logger::Env;

use metrics::Monitoring;

fn main() -> io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let monitoring = Arc::new(Mutex::new(Monitoring::new()));
    let reporting = monitoring.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        reporting.clone().lock().unwrap().report();
    });

    let port = env::args()
        .skip(1)
        .nth(0)
        .expect("Expected one argument with the serial port path");
    let mut geiger = geiger::Geiger::new(monitoring.clone(), &port)?;
    geiger.run()?;

    Ok(())
}
