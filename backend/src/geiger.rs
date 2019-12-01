use std::io;
use std::io::BufRead;
use std::time::{Duration, Instant};

use log::{debug, info};

use serial::prelude::*;
use serial::SystemPort;

use super::metrics::Monitoring;

const CONVERTION_FACTOR: f64 = 0.00812037037037;

pub struct Geiger<'a> {
    monitoring: &'a Monitoring,
    port: SystemPort,
}

impl<'a> Geiger<'a> {
    pub fn new(monitoring: &'a Monitoring, serial_port: &str) -> io::Result<Geiger<'a>> {
        let mut port = serial::open(&serial_port)?;
        Geiger::configure(&mut port)?;
        Ok(Geiger { monitoring, port })
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

    pub fn run(&mut self) -> io::Result<()> {
        let mut reader = io::BufReader::new(&mut self.port);
        let mut counter = 0;
        let mut metrics_timer = Instant::now();
        let mut now = Instant::now();
        let minute = Duration::from_secs(60);
        let ten_seconds = Duration::from_secs(10);

        loop {
            let mut line = String::new();
            if let Ok(_) = reader.read_line(&mut line) {
                self.monitoring.register_count();
                debug!("Received count");
                counter += 1;
                if metrics_timer.elapsed() > ten_seconds {
                    println!("{}", self.monitoring.report());
                    metrics_timer = Instant::now();
                }
                if now.elapsed() > minute {
                    info!(
                        "{} CPM\t{:.4} μSv/h",
                        counter,
                        Geiger::cpm_to_microsieverts(counter),
                    );
                    counter = 0;
                    now = Instant::now();
                }
            }
        }
    }

    pub fn cpm_to_microsieverts(cpm: u32) -> f64 {
        f64::from(cpm) * CONVERTION_FACTOR
    }
}
