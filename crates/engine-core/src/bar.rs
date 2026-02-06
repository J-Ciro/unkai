/**
 * Bar manifest and discovery types
 * Similar to Zebar's zpack.json but simpler for YASB
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bar positioning anchor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BarAnchor {
    #[serde(rename = "top_left")]
    TopLeft,
    #[serde(rename = "top_center")]
    TopCenter,
    #[serde(rename = "top_right")]
    TopRight,
    #[serde(rename = "bottom_left")]
    BottomLeft,
    #[serde(rename = "bottom_center")]
    BottomCenter,
    #[serde(rename = "bottom_right")]
    BottomRight,
}

impl Default for BarAnchor {
    fn default() -> Self {
        BarAnchor::TopCenter
    }
}

impl std::fmt::Display for BarAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarAnchor::TopLeft => write!(f, "top_left"),
            BarAnchor::TopCenter => write!(f, "top_center"),
            BarAnchor::TopRight => write!(f, "top_right"),
            BarAnchor::BottomLeft => write!(f, "bottom_left"),
            BarAnchor::BottomCenter => write!(f, "bottom_center"),
            BarAnchor::BottomRight => write!(f, "bottom_right"),
        }
    }
}

/// Bar window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarWindowConfig {
    /// Window width in pixels
    pub width: u32,
    /// Window height in pixels
    pub height: u32,
    /// Offset X from anchor point
    #[serde(default)]
    pub offset_x: i32,
    /// Offset Y from anchor point
    #[serde(default)]
    pub offset_y: i32,
    /// Where to anchor on screen
    #[serde(default)]
    pub anchor: BarAnchor,
    /// Always on top of other windows
    #[serde(default = "default_true")]
    pub always_on_top: bool,
    /// Transparent window
    #[serde(default = "default_true")]
    pub transparent: bool,
    /// Can user resize window
    #[serde(default)]
    pub resizable: bool,
    /// Show in taskbar
    #[serde(default)]
    pub shown_in_taskbar: bool,
}

fn default_true() -> bool {
    true
}

impl Default for BarWindowConfig {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 40,
            offset_x: 0,
            offset_y: 0,
            anchor: BarAnchor::TopCenter,
            always_on_top: true,
            transparent: true,
            resizable: false,
            shown_in_taskbar: false,
        }
    }
}

/// Bar manifest - defines a custom status bar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarManifest {
    /// Bar name (displayed in UI)
    pub name: String,
    /// Bar version
    #[serde(default)]
    pub version: String,
    /// Bar description
    #[serde(default)]
    pub description: String,
    /// Path to compiled HTML entry point (relative to bar directory)
    pub entry: String,
    /// Window configuration
    #[serde(default)]
    pub window: BarWindowConfig,
    /// Custom metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl BarManifest {
    /// Validate manifest
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("Bar name cannot be empty");
        }
        if self.entry.is_empty() {
            anyhow::bail!("Bar entry point cannot be empty");
        }
        if self.window.width == 0 || self.window.height == 0 {
            anyhow::bail!("Bar width and height must be greater than 0");
        }
        Ok(())
    }
}

/// Bar metadata (without the full manifest)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarInfo {
    /// Unique identifier (directory name)
    pub id: String,
    /// Bar name
    pub name: String,
    /// Bar description
    pub description: String,
    /// Version
    pub version: String,
    /// Path to bar directory
    pub path: String,
    /// Is this bar currently active
    pub active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_anchor_display() {
        assert_eq!(BarAnchor::TopCenter.to_string(), "top_center");
        assert_eq!(BarAnchor::BottomCenter.to_string(), "bottom_center");
    }

    #[test]
    fn test_bar_manifest_validation() {
        let mut manifest = BarManifest {
            name: "Test Bar".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            entry: "dist/index.html".to_string(),
            window: BarWindowConfig::default(),
            metadata: HashMap::new(),
        };

        assert!(manifest.validate().is_ok());

        manifest.name = String::new();
        assert!(manifest.validate().is_err());
    }

    #[test]
    fn test_bar_window_config_defaults() {
        let config = BarWindowConfig::default();
        assert_eq!(config.width, 1200);
        assert_eq!(config.height, 40);
        assert_eq!(config.anchor, BarAnchor::TopCenter);
        assert!(config.always_on_top);
        assert!(config.transparent);
    }
}
