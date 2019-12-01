use std::io;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::debug;

use serial::prelude::*;
use serial::SystemPort;

use super::metrics::Monitoring;

const CONVERTION_FACTOR: f64 = 0.00812037037037;

pub struct Geiger {
    monitoring: Arc<Mutex<Monitoring>>,
    port: SystemPort,
}

impl Geiger {
    pub fn new(monitoring: Arc<Mutex<Monitoring>>, serial_port: &str) -> io::Result<Geiger> {
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
        loop {
            let mut line = String::new();
            if let Ok(_) = reader.read_line(&mut line) {
                {
                    self.monitoring.lock().unwrap().register_count();
                }
                debug!("Received count");
            }
        }
    }

    pub fn cpm_to_microsieverts(cpm: u32) -> f64 {
        f64::from(cpm) * CONVERTION_FACTOR
    }
}
