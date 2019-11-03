use std::env;
use std::io;
use std::io::BufRead;
use std::time::{Duration, Instant};

use serial::prelude::*;

const CONVERTION_FACTOR: f64 = 0.00812037037037;

fn main() -> io::Result<()> {
    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg)?;
        configure(&mut port)?;
        interact(&mut port)?;
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

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    let mut reader = io::BufReader::new(port);
    let mut counter = 0;
    let mut now = Instant::now();
    let minute = Duration::from_secs(60);

    loop {
        let mut line = String::new();
        if let Ok(_) = reader.read_line(&mut line) {
            counter += 1;
            if now.elapsed() > minute {
                println!(
                    "{} CPM\t{} μSv/h",
                    counter,
                    f64::from(counter) * CONVERTION_FACTOR
                );
                counter = 0;
                now = Instant::now();
            }
        }
    }
}
