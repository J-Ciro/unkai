use serde::{Deserialize, Serialize};

/// Network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub is_up: bool,
    pub is_default: bool,
}

/// Network traffic information (bytes per second or total)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTraffic {
    pub received_per_sec: u64,
    pub transmitted_per_sec: u64,
    pub total_received: u64,
    pub total_transmitted: u64,
}

impl NetworkTraffic {
    /// Convert bytes per second to megabits per second
    pub fn received_mbps(&self) -> f32 {
        (self.received_per_sec as f32 * 8.0) / 1_000_000.0
    }

    /// Convert transmitted bytes per second to megabits per second
    pub fn transmitted_mbps(&self) -> f32 {
        (self.transmitted_per_sec as f32 * 8.0) / 1_000_000.0
    }

    /// Convert total received bytes to gigabytes
    pub fn total_received_gb(&self) -> f32 {
        self.total_received as f32 / 1_000_000_000.0
    }

    /// Convert total transmitted bytes to gigabytes
    pub fn total_transmitted_gb(&self) -> f32 {
        self.total_transmitted as f32 / 1_000_000_000.0
    }
}

/// Network provider for system network information
pub struct NetworkProvider;

impl NetworkProvider {
    /// Get all network interfaces
    pub fn get_interfaces() -> anyhow::Result<Vec<NetworkInterface>> {
        // Platform-specific implementation would go here
        Ok(vec![])
    }

    /// Get default network interface
    pub fn get_default_interface() -> anyhow::Result<Option<NetworkInterface>> {
        let interfaces = Self::get_interfaces()?;
        Ok(interfaces.into_iter().find(|i| i.is_default))
    }

    /// Get network traffic for default interface
    pub fn get_traffic() -> anyhow::Result<NetworkTraffic> {
        // Platform-specific implementation would go here
        Ok(NetworkTraffic {
            received_per_sec: 0,
            transmitted_per_sec: 0,
            total_received: 0,
            total_transmitted: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_interface_creation() {
        let iface = NetworkInterface {
            name: "eth0".to_string(),
            ip_address: Some("192.168.1.100".to_string()),
            mac_address: Some("00:11:22:33:44:55".to_string()),
            is_up: true,
            is_default: true,
        };

        assert_eq!(iface.name, "eth0");
        assert!(iface.is_up);
    }

    #[test]
    fn test_network_traffic_mbps() {
        let traffic = NetworkTraffic {
            received_per_sec: 1_000_000,
            transmitted_per_sec: 500_000,
            total_received: 1_000_000_000_000,
            total_transmitted: 500_000_000_000,
        };

        assert_eq!(traffic.received_mbps() as u32, 8); // 8 Mbps
        assert_eq!(traffic.transmitted_mbps() as u32, 4); // 4 Mbps
    }

    #[test]
    fn test_network_traffic_gb() {
        let traffic = NetworkTraffic {
            received_per_sec: 0,
            transmitted_per_sec: 0,
            total_received: 1_000_000_000,
            total_transmitted: 2_000_000_000,
        };

        assert_eq!(traffic.total_received_gb() as u32, 1); // 1 GB
        assert_eq!(traffic.total_transmitted_gb() as u32, 2); // 2 GB
    }
}
