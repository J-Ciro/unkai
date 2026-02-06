//! # Engine Core
//! 
//! Núcleo de la aplicación YASB.
//! Gestiona estado global, widgets, data sources y scheduler.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// =============================================================================
// ENUMS
// =============================================================================

/// Tipo de widget soportado
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WidgetType {
    /// Widget de sistema (CPU, RAM, Network)
    System,
    /// Widget de procesos (tasklist, específico)
    Process,
    /// Widget que ejecuta comandos
    Command,
    /// Widget que consume HTTP
    Http,
    /// Widget personalizado
    Custom,
}

/// Tipo de fuente de datos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DataSourceKind {
    /// Métricas del sistema (CPU, RAM, network)
    System,
    /// Procesos (tasklist)
    Process,
    /// Ejecutar comando
    Exec,
    /// HTTP request
    Http,
}

/// Posición del widget en la pantalla
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowPosition {
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Top center
    TopCenter,
    /// Bottom center
    BottomCenter,
    /// Custom coordinates (x, y)
    Custom(i32, i32),
}

/// Monitor selector
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitorSelector {
    /// Primary monitor
    Primary,
    /// Monitor by ID
    ById(String),
    /// Monitor by name
    ByName(String),
    /// All monitors
    All,
}

// =============================================================================
// CONFIG STRUCTURES
// =============================================================================

/// Configuración raíz del archivo YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Versión de formato config
    #[serde(default = "default_config_version")]
    pub version: String,

    /// Lista de widgets
    pub widgets: Vec<WidgetConfig>,

    /// Estilos globales (CSS)
    #[serde(default)]
    pub styles: GlobalStyles,

    /// Configuraciones por monitor
    #[serde(default)]
    pub displays: Vec<DisplayConfig>,

    /// Configuración global
    #[serde(default)]
    pub global: GlobalConfig,
}

fn default_config_version() -> String {
    "1.0".to_string()
}

/// Configuración global
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Modo debug
    pub debug: bool,
    /// Log level
    #[serde(default)]
    pub log_level: String,
    /// Selected status bar ID
    #[serde(default)]
    pub selected_bar: Option<String>,
}

/// Estilos globales
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalStyles {
    /// CSS variables globales
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
    /// Theme selector
    #[serde(default)]
    pub theme: String,
}

/// Configuración de display/monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Monitor selector
    pub monitor: MonitorSelector,
    /// Widgets en este display
    #[serde(default)]
    pub widgets: Vec<String>, // IDs de widgets
}

/// Configuración de un widget individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    /// ID único del widget
    pub id: String,

    /// Tipo de widget
    #[serde(rename = "type")]
    pub widget_type: WidgetType,

    /// Template HTML (path o builtin)
    pub template: String,

    /// Monitor selector
    #[serde(default = "default_monitor_selector")]
    pub monitor: MonitorSelector,

    /// Posición en pantalla
    pub position: WindowPosition,

    /// Ancho (px o %)
    #[serde(default)]
    pub width: Option<String>,

    /// Alto (px o %)
    #[serde(default)]
    pub height: Option<String>,

    /// Z-index
    #[serde(default)]
    pub z_index: i32,

    /// Interval de actualización (ms)
    #[serde(default = "default_update_interval")]
    pub update_interval_ms: u64,

    /// ¿Widget habilitado?
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Data sources
    #[serde(default)]
    pub data_sources: Vec<DataSourceConfig>,

    /// CSS overrides
    #[serde(default)]
    pub css_overrides: Option<String>,
}

fn default_monitor_selector() -> MonitorSelector {
    MonitorSelector::Primary
}

fn default_update_interval() -> u64 {
    1000 // 1 segundo
}

fn default_enabled() -> bool {
    true
}

/// Configuración de fuente de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceConfig {
    /// Nombre de la fuente
    pub name: String,

    /// Tipo de fuente
    pub kind: DataSourceKind,

    /// Configuración específica (JSON genérico)
    pub config: serde_json::Value,

    /// TTL del cache (ms)
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_ms: u64,
}

fn default_cache_ttl() -> u64 {
    5000 // 5 segundos
}

// =============================================================================
// RUNTIME STRUCTURES
// =============================================================================

/// Estado en tiempo real de un widget
#[derive(Debug, Clone)]
pub struct WidgetRuntime {
    /// Configuración del widget
    pub config: WidgetConfig,

    /// Último dato obtenido
    pub last_data: Option<serde_json::Value>,

    /// Timestamp del último update
    pub last_update: DateTime<Utc>,

    /// Número de subscribers
    pub subscribers_count: u32,

    /// Error (si ocurrió)
    pub error: Option<String>,
}

impl WidgetRuntime {
    /// Crear nuevo runtime desde config
    pub fn new(config: WidgetConfig) -> Self {
        Self {
            config,
            last_data: None,
            last_update: Utc::now(),
            subscribers_count: 0,
            error: None,
        }
    }

    /// Actualizar datos
    pub fn update_data(&mut self, data: serde_json::Value) {
        self.last_data = Some(data);
        self.last_update = Utc::now();
        self.error = None;
    }

    /// Registrar error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.last_update = Utc::now();
    }
}

/// Información de monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    /// ID único del monitor
    pub id: String,
    /// Nombre del monitor
    pub name: String,
    /// DPI scale factor
    pub dpi: u32,
    /// Posición X
    pub x: i32,
    /// Posición Y
    pub y: i32,
    /// Ancho en píxeles
    pub width: i32,
    /// Alto en píxeles
    pub height: i32,
    /// ¿Es primary?
    pub is_primary: bool,
}

// =============================================================================
// WORKSPACE STATE (Komorebi integration)
// =============================================================================

/// Estado de workspace desde Komorebi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    /// ID del workspace
    pub workspace_id: i32,
    /// Índice del monitor
    pub monitor_index: i32,
    /// Modo de layout (bsp, columns, rows, grid, maximized)
    pub layout: String,
    /// Número de ventanas en el workspace
    pub window_count: i32,
    /// ¿Está enfocado?
    pub focused: bool,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            workspace_id: 1,
            monitor_index: 0,
            layout: "bsp".to_string(),
            window_count: 0,
            focused: false,
        }
    }
}

// =============================================================================
// GLOBAL STATE
// =============================================================================

/// Estado compartido global de la app
pub struct GlobalStateInner {
    /// Widgets activos: id -> WidgetRuntime
    pub widgets: DashMap<String, WidgetRuntime>,

    /// Monitores detectados
    pub monitors: parking_lot::RwLock<Vec<MonitorInfo>>,

    /// Configuración actual
    pub config: parking_lot::RwLock<Config>,

    /// Estado de workspace (Komorebi)
    pub workspace: parking_lot::RwLock<WorkspaceState>,

    /// Event broadcast channel
    pub event_tx: tokio::sync::broadcast::Sender<StateEvent>,
}

/// Tipo de evento de estado
#[derive(Debug, Clone, Serialize)]
pub enum StateEvent {
    /// Widget datos actualizados
    WidgetUpdated {
        widget_id: String,
        data: serde_json::Value,
        timestamp: DateTime<Utc>,
    },
    /// Monitores cambiaron
    MonitorsChanged {
        monitors: Vec<MonitorInfo>,
    },
    /// Config recargada
    ConfigReloaded {
        config: Arc<Config>,
    },
    /// Workspace cambió (Komorebi)
    WorkspaceChanged {
        workspace: WorkspaceState,
        timestamp: DateTime<Utc>,
    },
    /// Error en widget
    WidgetError {
        widget_id: String,
        error: String,
        timestamp: DateTime<Utc>,
    },
}

/// Alias para estado compartido
pub type SharedState = Arc<GlobalStateInner>;

// =============================================================================
// PUBLIC FUNCTIONS
// =============================================================================

/// Inicializar estado global
pub fn init(config: Config) -> Result<SharedState> {
    let (event_tx, _) = tokio::sync::broadcast::channel(100);

    let state = Arc::new(GlobalStateInner {
        widgets: DashMap::new(),
        monitors: parking_lot::RwLock::new(Vec::new()),
        config: parking_lot::RwLock::new(config.clone()),
        workspace: parking_lot::RwLock::new(WorkspaceState::default()),
        event_tx,
    });

    // Inicializar widgets desde config
    for widget_cfg in config.widgets {
        if widget_cfg.enabled {
            let runtime = WidgetRuntime::new(widget_cfg);
            state.widgets.insert(runtime.config.id.clone(), runtime);
        }
    }

    Ok(state)
}

/// Obtener widget desde estado
pub fn get_widget(state: &SharedState, widget_id: &str) -> Option<WidgetRuntime> {
    state
        .widgets
        .get(widget_id)
        .map(|entry| entry.clone())
}

/// Actualizar datos de widget
pub fn update_widget_data(
    state: &SharedState,
    widget_id: &str,
    data: serde_json::Value,
) -> Result<()> {
    if let Some(mut widget) = state.widgets.get_mut(widget_id) {
        widget.update_data(data.clone());
        let _ = state.event_tx.send(StateEvent::WidgetUpdated {
            widget_id: widget_id.to_string(),
            data,
            timestamp: Utc::now(),
        });
        Ok(())
    } else {
        Err(anyhow::anyhow!("Widget not found: {}", widget_id))
    }
}

/// Get all widgets snapshot
pub fn get_all_widgets(state: &SharedState) -> Vec<WidgetRuntime> {
    state
        .widgets
        .iter()
        .map(|entry| entry.value().clone())
        .collect()
}

/// Obtener estado del workspace actual
pub fn get_workspace(state: &SharedState) -> WorkspaceState {
    state.workspace.read().clone()
}

/// Actualizar estado del workspace desde Komorebi
pub fn update_workspace(state: &SharedState, workspace: WorkspaceState) -> Result<()> {
    *state.workspace.write() = workspace.clone();

    // Emitir evento
    let _ = state.event_tx.send(StateEvent::WorkspaceChanged {
        workspace,
        timestamp: Utc::now(),
    });

    Ok(())
}

/// Validar estructura de config
pub fn validate_config(config: &Config) -> Result<()> {
    // Validar widgets
    for widget in &config.widgets {
        validate_widget_config(widget)?;
    }

    // Validar que no haya IDs duplicados
    let ids: Vec<_> = config.widgets.iter().map(|w| &w.id).collect();
    let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
    if ids.len() != unique_ids.len() {
        return Err(anyhow::anyhow!("Duplicate widget IDs found in config"));
    }

    Ok(())
}

pub mod config;
pub use config::{get_config_path, load_config, create_default_config};

pub mod scheduler;
pub use scheduler::start_scheduler;

pub mod data_sources;
pub use data_sources::{DataSource, DataSourceCache};

pub mod event_bus;
pub use event_bus::{subscribe, emit, EventSubscriber};

pub mod komorebi;
pub use komorebi::{KomorebiClient, KomorebiState, WorkspaceEvent, LayoutMode};

pub mod export;
pub use export::{Exporter, ExportFormat, ZebarExport, ZebarWidget};

pub mod template;
pub use template::{Template, TemplateContext};

pub mod providers;
pub use providers::{AudioProvider, AudioDevice, BatteryProvider, BatteryStatus, BatteryState, MemoryProvider, MemoryStats, NetworkProvider, NetworkInterface, NetworkTraffic};

pub mod bar;
pub use bar::{BarManifest, BarAnchor, BarWindowConfig, BarInfo};

pub mod bar_discovery;
pub use bar_discovery::{discover_bars, get_bar, get_bar_manifest, get_bar_entry_path, create_bar_scaffold, get_bars_directory};

/// Validar configuración de un widget (pub para tests en config.rs)
pub fn validate_widget_config(widget: &WidgetConfig) -> Result<()> {
    // Validar ID
    if widget.id.is_empty() {
        return Err(anyhow::anyhow!("Widget ID cannot be empty"));
    }

    if !widget.id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(anyhow::anyhow!(
            "Invalid widget ID: {}. Must be alphanumeric, dash or underscore",
            widget.id
        ));
    }

    // Validar update_interval
    if widget.update_interval_ms == 0 {
        return Err(anyhow::anyhow!(
            "Widget {}: update_interval_ms must be > 0",
            widget.id
        ));
    }

    // Validar data sources
    for ds in &widget.data_sources {
        if ds.name.is_empty() {
            return Err(anyhow::anyhow!("Data source name cannot be empty"));
        }
    }

    Ok(())
}

/// Export all widgets in specified format
pub fn export_widgets(state: &SharedState, format: ExportFormat) -> Result<String> {
    let widgets = get_all_widgets(state);

    match format {
        ExportFormat::JSON => Exporter::to_json(&widgets)
            .map_err(|e| anyhow::anyhow!("JSON export failed: {}", e)),
        ExportFormat::Zebar => Exporter::to_zebar(&widgets)
            .map_err(|e| anyhow::anyhow!("Zebar export failed: {}", e)),
        ExportFormat::HTML => {
            let html_export = Exporter::to_html(&widgets);
            Ok(html_export.html)
        }
        ExportFormat::CSS => Ok(Exporter::to_css()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_config_defaults() {
        let cfg = WidgetConfig {
            id: "test".to_string(),
            widget_type: WidgetType::System,
            template: "cpu.html".to_string(),
            monitor: MonitorSelector::Primary,
            position: WindowPosition::TopLeft,
            width: None,
            height: None,
            z_index: 1,
            update_interval_ms: 1000,
            enabled: true,
            data_sources: Vec::new(),
            css_overrides: None,
        };

        assert_eq!(cfg.id, "test");
        assert_eq!(cfg.update_interval_ms, 1000);
    }

    #[test]
    fn test_state_init() {
        let config = Config {
            version: "1.0".to_string(),
            widgets: vec![],
            styles: GlobalStyles::default(),
            displays: vec![],
            global: GlobalConfig::default(),
        };

        let state = init(config).unwrap();
        assert_eq!(state.widgets.len(), 0);
    }

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

        assert!(validate_widget_config(&widget).is_err());
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

        assert!(validate_widget_config(&widget).is_err());
    }
}
