mod metrics;

use std::env;
use std::io;
use std::io::BufRead;
use std::time::{Duration, Instant};

use serial::prelude::*;

use env_logger::Env;
use log::{debug, info};

use metrics::Monitoring;

const CONVERTION_FACTOR: f64 = 0.00812037037037;

fn main() -> io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let monitoring = Monitoring::new();

    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg)?;
        configure(&mut port)?;
        interact(&monitoring, &mut port)?;
    }

    Ok(())
}

fn configure<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;
    port.set_timeout(Duration::from_millis(10000))?;
    Ok(())
}

fn interact<T: SerialPort>(monitoring: &Monitoring, port: &mut T) -> io::Result<()> {
    let mut reader = io::BufReader::new(port);
    let mut counter = 0;
    let mut metrics_timer = Instant::now();
    let mut now = Instant::now();
    let minute = Duration::from_secs(60);
    let ten_seconds = Duration::from_secs(10);

    loop {
        let mut line = String::new();
        if let Ok(_) = reader.read_line(&mut line) {
            monitoring.register_count();
            debug!("Received count");
            counter += 1;
            if metrics_timer.elapsed() > ten_seconds {
                println!("{}", monitoring.report());
                metrics_timer = Instant::now();
            }
            if now.elapsed() > minute {
                info!(
                    "{} CPM\t{:.4} μSv/h",
                    counter,
                    f64::from(counter) * CONVERTION_FACTOR
                );
                counter = 0;
                now = Instant::now();
            }
        }
    }
}
