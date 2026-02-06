use serde::{Deserialize, Serialize};

/// Audio device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub device_id: String,
    pub name: String,
    pub volume: f32,
    pub device_type: AudioDeviceType,
    pub is_default: bool,
}

/// Audio device type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioDeviceType {
    #[serde(rename = "playback")]
    Playback,
    #[serde(rename = "recording")]
    Recording,
}

/// Audio provider for system audio information
pub struct AudioProvider;

impl AudioProvider {
    /// Get all playback devices
    pub fn get_playback_devices() -> anyhow::Result<Vec<AudioDevice>> {
        // Platform-specific implementation would go here
        // For now, return empty vector (placeholder)
        Ok(vec![])
    }

    /// Get all recording devices
    pub fn get_recording_devices() -> anyhow::Result<Vec<AudioDevice>> {
        // Platform-specific implementation would go here
        Ok(vec![])
    }

    /// Get default playback device
    pub fn get_default_playback_device() -> anyhow::Result<Option<AudioDevice>> {
        // Platform-specific implementation would go here
        Ok(None)
    }

    /// Get default recording device
    pub fn get_default_recording_device() -> anyhow::Result<Option<AudioDevice>> {
        // Platform-specific implementation would go here
        Ok(None)
    }

    /// Set volume for a device (0-100)
    pub fn set_volume(_device_id: &str, volume: f32) -> anyhow::Result<()> {
        // Validate volume range
        if volume < 0.0 || volume > 100.0 {
            anyhow::bail!("Volume must be between 0 and 100");
        }

        // Platform-specific implementation would go here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_device_creation() {
        let device = AudioDevice {
            device_id: "default".to_string(),
            name: "Speakers".to_string(),
            volume: 50.0,
            device_type: AudioDeviceType::Playback,
            is_default: true,
        };

        assert_eq!(device.name, "Speakers");
        assert_eq!(device.volume, 50.0);
    }

    #[test]
    fn test_volume_validation() {
        assert!(AudioProvider::set_volume("default", 50.0).is_ok());
        assert!(AudioProvider::set_volume("default", 0.0).is_ok());
        assert!(AudioProvider::set_volume("default", 100.0).is_ok());
        assert!(AudioProvider::set_volume("default", -1.0).is_err());
        assert!(AudioProvider::set_volume("default", 101.0).is_err());
    }
}
