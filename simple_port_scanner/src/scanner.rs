use serde::Serialize;
use std::net::{IpAddr, TcpStream};
use std::time::Duration;

use crate::config::Config;

#[derive(Serialize)]
struct ScanResult {
    ip: IpAddr,
    port: u16,
    status: String,
}

pub fn scan(ip: IpAddr, ports: &[u16], config: Option<&Config>) {
    let timeout = Duration::from_secs(2);
    let mut results = Vec::new();

    for port in ports {
        let addr = format!("{}:{}", ip, port);
        let status = match TcpStream::connect_timeout(&addr.parse().unwrap(), timeout) {
            Ok(_) => "open".to_string(),
            Err(_) => "closed".to_string(),
        };

        let result = ScanResult {
            ip,
            port: *port,
            status: status.clone(),
        };

        if config.is_none() {
            // Regular mode: print to stdout
            println!("Port {} {} on IP {}", port, status, ip);
        }

        results.push(result);
    }

    if let Some(cfg) = config {
        // Mapping mode: print results as JSON
        let json = serde_json::to_string_pretty(&results).unwrap();
        println!("{}", json);

        // Optional: send to web app via POST
        if let Err(err) = send_results(&cfg.mapper.endpoint, &results) {
            eprintln!("Error sending results to mapper: {}", err);
        }
    }
}

fn send_results(endpoint: &str, results: &[ScanResult]) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client.post(endpoint).json(results).send()?;

    if !res.status().is_success() {
        return Err(format!("Server responded with status: {}", res.status()).into());
    }

    Ok(())
}
