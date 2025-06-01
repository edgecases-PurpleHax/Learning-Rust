use serde::Deserialize;

/// Represents a single scan result coming from the network scanner.
#[derive(Debug, Deserialize)]
pub struct ScanResult {
    pub ip: String,
    pub port: u16,
    pub status: String,
}
