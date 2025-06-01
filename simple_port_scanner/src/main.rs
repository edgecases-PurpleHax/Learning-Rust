mod scanner;
use scanner::scan;
use std::env;
use std::fs;
use std::net::IpAddr;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "Usage: {} (-i <IP> | -f <file>) (-p <port> | -r <start-end> | -a)",
            args[0]
        );
        return;
    }

    let targets: Vec<IpAddr> = match args[1].as_str() {
        "-i" | "--ip" => {
            vec![IpAddr::from_str(&args[2]).expect("Invalid IP address")]
        }
        "-f" | "--file" => {
            let contents = fs::read_to_string(&args[2]).expect("Failed to read file");
            contents
                .lines()
                .filter_map(|line| IpAddr::from_str(line.trim()).ok())
                .collect()
        }
        _ => {
            eprintln!("Invalid target option: {}", args[1]);
            return;
        }
    };

    let ports: Vec<u16> = match args[3].as_str() {
        "-p" | "--port" => {
            if args.len() < 5 {
                eprintln!("Missing port number for {}", args[3]);
                return;
            }
            vec![args[4].parse::<u16>().expect("Invalid port")]
        }
        "-r" | "--range" => {
            if args.len() < 5 {
                eprintln!("Missing range for {}", args[3]);
                return;
            }
            let range_parts: Vec<&str> = args[4].split('-').collect();
            if range_parts.len() != 2 {
                eprintln!("Invalid range format. Use <start>-<end>");
                return;
            }
            let start: u16 = range_parts[0].parse().expect("Invalid start port");
            let end: u16 = range_parts[1].parse().expect("Invalid end port");
            if start > end {
                eprintln!("Start port must be <= end port");
                return;
            }
            (start..=end).collect()
        }
        "-a" | "--all" => (1..=65535).collect(),
        _ => {
            eprintln!("Invalid port option: {}", args[3]);
            return;
        }
    };

    for ip in targets {
        println!("Scanning {} on ports {:?}", ip, ports);
        scan(ip, &ports);
    }
}
