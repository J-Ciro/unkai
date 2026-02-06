//! Configuración: lectura, parseo y validación YAML

use crate::{Config, DataSourceConfig, DataSourceKind, WidgetConfig, WidgetType, WindowPosition, MonitorSelector};
use anyhow::{anyhow, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Obtener path de config (Windows)
pub fn get_config_path() -> Result<PathBuf> {
    let app_data = env::var("APPDATA")
        .context("APPDATA environment variable not found")?;
    Ok(PathBuf::from(app_data).join("unkai").join("config.yaml"))
}

/// Leer y parsear config YAML
pub fn load_config(path: Option<PathBuf>) -> Result<Config> {
    let config_path = if let Some(p) = path {
        p
    } else {
        get_config_path()?
    };

    debug!("Loading config from: {}", config_path.display());

    // Leer archivo
    if !config_path.exists() {
        return Err(anyhow!(
            "Config file not found: {}",
            config_path.display()
        ));
    }

    let yaml_content = fs::read_to_string(&config_path)
        .context("Failed to read config file")?;

    // Parsear YAML
    let config: Config = serde_yaml::from_str(&yaml_content)
        .context("Failed to parse YAML")?;

    info!("Config loaded successfully: {} widgets", config.widgets.len());

    // Validar
    crate::validate_config(&config)?;

    Ok(config)
}

/// Crear config default si no existe
pub fn create_default_config(path: &Path) -> Result<Config> {
    use crate::{GlobalConfig, GlobalStyles};
    use std::collections::HashMap;

    let mut variables = HashMap::new();
    variables.insert("primary-color".to_string(), "#00ff00".to_string());
    variables.insert("secondary-color".to_string(), "#0000ff".to_string());

    let config = Config {
        version: "1.0".to_string(),
        widgets: vec![
            WidgetConfig {
                id: "system-stats".to_string(),
                widget_type: WidgetType::System,
                template: "builtin:system-stats".to_string(),
                monitor: MonitorSelector::Primary,
                position: WindowPosition::TopLeft,
                width: Some("300px".to_string()),
                height: Some("100px".to_string()),
                z_index: 1,
                update_interval_ms: 1000,
                enabled: true,
                data_sources: vec![
                    DataSourceConfig {
                        name: "cpu_memory".to_string(),
                        kind: DataSourceKind::System,
                        config: serde_json::json!({
                            "metrics": ["cpu", "memory", "network"]
                        }),
                        cache_ttl_ms: 1000,
                    },
                ],
                css_overrides: None,
            },
        ],
        styles: GlobalStyles {
            variables,
            theme: "dark".to_string(),
        },
        displays: vec![],
        global: GlobalConfig {
            debug: false,
            log_level: "info".to_string(),
            selected_bar: None,
        },
    };

    // Crear directorio
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create config directory")?;
    }

    // Escribir archivo
    let yaml_str = serde_yaml::to_string(&config)?;
    fs::write(path, yaml_str)
        .context("Failed to create default config file")?;

    info!("Created default config at: {}", path.display());

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_widget_empty_id() {
        let widget = WidgetConfig {
            id: String::new(),
            widget_type: WidgetType::System,
            template: "test".to_string(),
            monitor: MonitorSelector::Primary,
            position: WindowPosition::TopLeft,
            width: None,
            height: None,
            z_index: 1,
            update_interval_ms: 1000,
            enabled: true,
            data_sources: vec![],
            css_overrides: None,
        };

        assert!(crate::validate_widget_config(&widget).is_err());
    }

    #[test]
    fn test_validate_widget_invalid_id() {
        let widget = WidgetConfig {
            id: "test@widget!".to_string(),
            widget_type: WidgetType::System,
            template: "test".to_string(),
            monitor: MonitorSelector::Primary,
            position: WindowPosition::TopLeft,
            width: None,
            height: None,
            z_index: 1,
            update_interval_ms: 1000,
            enabled: true,
            data_sources: vec![],
            css_overrides: None,
        };

        assert!(crate::validate_widget_config(&widget).is_err());
    }
}
