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
    let local_reporting = monitoring.clone();
    let exporter = monitoring.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        local_reporting.lock().unwrap().report();
    });

    thread::spawn(move || {
        use hyper::header::CONTENT_TYPE;
        use hyper::rt::Future;
        use hyper::service::service_fn_ok;
        use hyper::{Body, Response, Server};

        let addr = ([127, 0, 0, 1], 9898).into();
        println!("Listening address: {:?}", addr);

        let exporter_service = move || {
            let exporter = exporter.clone();
            service_fn_ok(move |_request| {
                Response::builder()
                    .status(200)
                    .header(CONTENT_TYPE, "text/application")
                    .body(Body::from(exporter.lock().unwrap().prometheus_export()))
                    .unwrap()
            })
        };

        let server = Server::bind(&addr)
            .serve(exporter_service)
            .map_err(|e| eprintln!("Server error: {}", e));

        hyper::rt::run(server);
    });

    let port = env::args()
        .skip(1)
        .nth(0)
        .expect("Expected one argument with the serial port path");
    let mut geiger = geiger::Geiger::new(monitoring.clone(), &port)?;
    geiger.run()?;

    Ok(())
}
