use serde::Serialize;
use std::net::{IpAddr, TcpStream};
use std::time::Duration;

#[derive(Serialize)]
pub struct ScanResult {
    pub ip: IpAddr,
    pub port: u16,
    pub status: String,
}

pub fn scan(ip: IpAddr, ports: &[u16]) -> Vec<ScanResult> {
    let timeout = Duration::from_secs(2);
    let mut results = Vec::new();

    for port in ports {
        let addr = format!("{}:{}", ip, port);
        let status = match TcpStream::connect_timeout(&addr.parse().unwrap(), timeout) {
            Ok(_) => "open".to_string(),
            Err(_) => "closed".to_string(),
        };

        results.push(ScanResult {
            ip,
            port: *port,
            status,
        });
    }

    results
}
