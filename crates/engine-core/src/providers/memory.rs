use serde::{Deserialize, Serialize};

/// Memory statistics in bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub available: u64,
    pub buffers: Option<u64>,
    pub cached: Option<u64>,
    pub swap_total: Option<u64>,
    pub swap_used: Option<u64>,
    pub swap_free: Option<u64>,
}

impl MemoryStats {
    /// Get memory usage percentage (0-100)
    pub fn usage_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.used as f32 / self.total as f32) * 100.0
        }
    }

    /// Convert bytes to megabytes
    pub fn used_mb(&self) -> f32 {
        self.used as f32 / 1024.0 / 1024.0
    }

    /// Convert total bytes to megabytes
    pub fn total_mb(&self) -> f32 {
        self.total as f32 / 1024.0 / 1024.0
    }
}

/// Memory provider for system memory information
pub struct MemoryProvider;

impl MemoryProvider {
    /// Get current memory statistics
    pub fn get_stats() -> anyhow::Result<MemoryStats> {
        // Platform-specific implementation would go here
        // For now, return default stats
        Ok(MemoryStats {
            total: 16_000_000_000, // 16 GB
            used: 8_000_000_000,
            free: 8_000_000_000,
            available: 8_000_000_000,
            buffers: None,
            cached: None,
            swap_total: None,
            swap_used: None,
            swap_free: None,
        })
    }

    /// Get memory usage percentage
    pub fn get_usage_percent() -> anyhow::Result<f32> {
        let stats = Self::get_stats()?;
        Ok(stats.usage_percent())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_stats_usage_percent() {
        let stats = MemoryStats {
            total: 1_000_000_000,
            used: 500_000_000,
            free: 500_000_000,
            available: 500_000_000,
            buffers: None,
            cached: None,
            swap_total: None,
            swap_used: None,
            swap_free: None,
        };

        assert_eq!(stats.usage_percent(), 50.0);
    }

    #[test]
    fn test_memory_stats_mb_conversion() {
        let stats = MemoryStats {
            total: 16_000_000_000,
            used: 8_000_000_000,
            free: 8_000_000_000,
            available: 8_000_000_000,
            buffers: None,
            cached: None,
            swap_total: None,
            swap_used: None,
            swap_free: None,
        };

        assert_eq!(stats.used_mb() as u32, 7629); // ~7629 MB
        assert_eq!(stats.total_mb() as u32, 15258); // ~15258 MB
    }

    #[test]
    fn test_memory_stats_zero_total() {
        let stats = MemoryStats {
            total: 0,
            used: 0,
            free: 0,
            available: 0,
            buffers: None,
            cached: None,
            swap_total: None,
            swap_used: None,
            swap_free: None,
        };

        assert_eq!(stats.usage_percent(), 0.0);
    }
}
