mod geiger;
mod metrics;

use std::env;
use std::io;

use env_logger::Env;

use metrics::Monitoring;

fn main() -> io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let monitoring = Monitoring::new();

    let port = env::args()
        .skip(1)
        .nth(0)
        .expect("Expected one argument with the serial port path");
    let mut geiger = geiger::Geiger::new(&monitoring, &port)?;
    geiger.run()?;

    Ok(())
}
