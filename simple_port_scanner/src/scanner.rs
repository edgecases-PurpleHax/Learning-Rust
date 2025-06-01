use std::net::{IpAddr, TcpStream};
use std::time::Duration;

pub fn scan(ip: IpAddr, ports: &[u16]) {
    let timeout = Duration::from_secs(2);

    println!("Scanning {} on ports {:?}", ip, ports);

    for port in ports {
        let addr = format!("{}:{}", ip, port);
        match TcpStream::connect_timeout(&addr.parse().unwrap(), timeout) {
            Ok(_) => println!("Port {} open on IP {}", port, ip),
            Err(_) => println!("Port {} closed on IP {}", port, ip),
        }
    }
}
