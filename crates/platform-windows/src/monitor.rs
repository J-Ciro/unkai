//! Utilidades para monitores

use engine_core::MonitorInfo;

/// Helpers para búsqueda de monitores
pub trait MonitorExt {
    /// Find monitor by name
    fn find_by_name(&self, name: &str) -> Option<&MonitorInfo>;
    /// Find primary monitor
    fn find_primary(&self) -> Option<&MonitorInfo>;
}

impl MonitorExt for Vec<MonitorInfo> {
    fn find_by_name(&self, name: &str) -> Option<&MonitorInfo> {
        self.iter().find(|m| m.name.contains(name))
    }

    fn find_primary(&self) -> Option<&MonitorInfo> {
        self.iter().find(|m| m.is_primary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_ext_find_primary() {
        let monitors = vec![
            MonitorInfo {
                id: "mon_1".to_string(),
                name: "Monitor 1".to_string(),
                dpi: 96,
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                is_primary: true,
            },
            MonitorInfo {
                id: "mon_2".to_string(),
                name: "Monitor 2".to_string(),
                dpi: 96,
                x: 1920,
                y: 0,
                width: 1920,
                height: 1080,
                is_primary: false,
            },
        ];

        let primary = monitors.find_primary();
        assert!(primary.is_some());
        assert_eq!(primary.unwrap().id, "mon_1");
    }

    #[test]
    fn test_monitor_ext_find_by_name() {
        let monitors = vec![
            MonitorInfo {
                id: "mon_1".to_string(),
                name: "HDMI-1".to_string(),
                dpi: 96,
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                is_primary: true,
            },
        ];

        let monitor = monitors.find_by_name("HDMI");
        assert!(monitor.is_some());
        assert_eq!(monitor.unwrap().name, "HDMI-1");
    }
}
