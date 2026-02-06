use serde::{Deserialize, Serialize};

/// Battery state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BatteryState {
    #[serde(rename = "charging")]
    Charging,
    #[serde(rename = "discharging")]
    Discharging,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for BatteryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryState::Charging => write!(f, "Charging"),
            BatteryState::Discharging => write!(f, "Discharging"),
            BatteryState::Full => write!(f, "Full"),
            BatteryState::Empty => write!(f, "Empty"),
            BatteryState::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Battery status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryStatus {
    pub charge_percent: f32,
    pub state: BatteryState,
    pub health_percent: Option<f32>,
    pub time_till_empty: Option<u64>,
    pub time_till_full: Option<u64>,
    pub power_consumption: Option<f32>,
}

/// Battery provider for system battery information
pub struct BatteryProvider;

impl BatteryProvider {
    /// Get current battery status
    pub fn get_status() -> anyhow::Result<Option<BatteryStatus>> {
        // Platform-specific implementation would go here
        // For now, return None (no battery)
        Ok(None)
    }

    /// Check if device is charging
    pub fn is_charging() -> anyhow::Result<bool> {
        match Self::get_status()? {
            Some(status) => Ok(status.state == BatteryState::Charging),
            None => Ok(false),
        }
    }

    /// Get battery percentage
    pub fn get_charge_percent() -> anyhow::Result<Option<f32>> {
        match Self::get_status()? {
            Some(status) => Ok(Some(status.charge_percent)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_state_display() {
        assert_eq!(BatteryState::Charging.to_string(), "Charging");
        assert_eq!(BatteryState::Discharging.to_string(), "Discharging");
        assert_eq!(BatteryState::Full.to_string(), "Full");
        assert_eq!(BatteryState::Empty.to_string(), "Empty");
        assert_eq!(BatteryState::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_battery_status_creation() {
        let status = BatteryStatus {
            charge_percent: 75.0,
            state: BatteryState::Discharging,
            health_percent: Some(95.0),
            time_till_empty: Some(3600000),
            time_till_full: None,
            power_consumption: Some(10.5),
        };

        assert_eq!(status.charge_percent, 75.0);
        assert_eq!(status.state, BatteryState::Discharging);
    }

    #[test]
    fn test_battery_state_equality() {
        assert_eq!(BatteryState::Charging, BatteryState::Charging);
        assert_ne!(BatteryState::Charging, BatteryState::Discharging);
    }
}
