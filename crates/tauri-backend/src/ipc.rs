//! IPC: Comandos invocables desde frontend

use engine_core::{load_config, get_config_path, Config, SharedState, StateEvent, subscribe};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct CommandResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// get_config: Obtener configuración actual
#[tauri::command]
pub fn get_config(state: tauri::State<SharedState>) -> CommandResponse<Config> {
    let config = state.config.read().clone();
    CommandResponse::ok(config)
}

/// set_config: Guardar nueva configuración
#[tauri::command]
pub fn set_config(
    config: Config,
    state: tauri::State<SharedState>,
) -> CommandResponse<String> {
    // Validar
    match engine_core::validate_config(&config) {
        Ok(_) => {
            *state.config.write() = config;
            CommandResponse::ok("Config updated".to_string())
        }
        Err(e) => CommandResponse::err(format!("Invalid config: {}", e)),
    }
}

/// reload_config: Recargar desde disco
#[tauri::command]
pub fn reload_config(state: tauri::State<SharedState>) -> CommandResponse<Config> {
    match get_config_path().and_then(load_config) {
        Ok(config) => {
            *state.config.write() = config.clone();
            CommandResponse::ok(config)
        }
        Err(e) => CommandResponse::err(format!("Failed to reload: {}", e)),
    }
}

/// get_widget_state: Obtener estado de un widget
#[tauri::command]
pub fn get_widget_state(
    id: String,
    state: tauri::State<SharedState>,
) -> CommandResponse<Value> {
    match engine_core::get_widget(&state, &id) {
        Some(widget) => CommandResponse::ok(serde_json::json!({
            "id": widget.config.id,
            "data": widget.last_data,
            "last_update": widget.last_update,
            "error": widget.error,
        })),
        None => CommandResponse::err(format!("Widget not found: {}", id)),
    }
}

/// get_all_widgets: Obtener todos los widgets
#[tauri::command]
pub fn get_all_widgets(state: tauri::State<SharedState>) -> CommandResponse<Vec<Value>> {
    let widgets = engine_core::get_all_widgets(&state);
    let data = widgets
        .iter()
        .map(|w| {
            serde_json::json!({
                "id": w.config.id,
                "type": w.config.widget_type,
                "data": w.last_data,
                "last_update": w.last_update,
            })
        })
        .collect();

    CommandResponse::ok(data)
}

/// run_widget_action: Ejecutar acción en widget
#[tauri::command]
pub fn run_widget_action(
    widget_id: String,
    action: String,
    payload: Option<Value>,
) -> CommandResponse<String> {
    // Implementación básica
    CommandResponse::ok(format!(
        "Action '{}' queued for widget '{}'",
        action, widget_id
    ))
}

/// export_widgets: Exportar widgets en formato especificado
#[tauri::command]
pub fn export_widgets(
    format: String,
    state: tauri::State<SharedState>,
) -> CommandResponse<String> {
    let export_format = match format.as_str() {
        "json" => engine_core::ExportFormat::JSON,
        "html" => engine_core::ExportFormat::HTML,
        "css" => engine_core::ExportFormat::CSS,
        "zebar" => engine_core::ExportFormat::Zebar,
        _ => return CommandResponse::err(format!("Unknown export format: {}", format)),
    };

    match engine_core::export_widgets(&state, export_format) {
        Ok(exported) => CommandResponse::ok(exported),
        Err(e) => CommandResponse::err(format!("Export failed: {}", e)),
    }
}

/// Event broadcaster task: Escucha StateEvents y emite a frontend
pub async fn broadcast_events(
    state: SharedState,
    app_handle: AppHandle,
) {
    let mut subscriber = subscribe(&state);

    while let Some(event) = subscriber.next().await {
        match event {
            StateEvent::WidgetUpdated { widget_id, data, timestamp } => {
                let _ = app_handle.emit_all(
                    "widget:updated",
                    serde_json::json!({
                        "widget_id": widget_id,
                        "data": data,
                        "timestamp": timestamp
                    }),
                );
            }
            StateEvent::MonitorsChanged { monitors } => {
                let _ = app_handle.emit_all("monitors:changed", monitors);
            }
            StateEvent::ConfigReloaded { config } => {
                let _ = app_handle.emit_all("config:reloaded", serde_json::to_value(&config).ok());
            }
            StateEvent::WidgetError { widget_id, error, timestamp } => {
                let _ = app_handle.emit_all(
                    "widget:error",
                    serde_json::json!({
                        "widget_id": widget_id,
                        "error": error,
                        "timestamp": timestamp
                    }),
                );
            }
        }
    }
}

// =============================================================================
// SYSTEM PROVIDERS IPC COMMANDS
// =============================================================================

/// get_memory_stats: Get current memory statistics
#[tauri::command]
pub fn get_memory_stats() -> CommandResponse<engine_core::MemoryStats> {
    match engine_core::MemoryProvider::get_stats() {
        Ok(stats) => CommandResponse::ok(stats),
        Err(e) => CommandResponse::err(format!("Failed to get memory stats: {}", e)),
    }
}

/// get_battery_status: Get current battery status
#[tauri::command]
pub fn get_battery_status() -> CommandResponse<Option<engine_core::BatteryStatus>> {
    match engine_core::BatteryProvider::get_status() {
        Ok(status) => CommandResponse::ok(status),
        Err(e) => CommandResponse::err(format!("Failed to get battery status: {}", e)),
    }
}

/// get_audio_playback_devices: Get all playback devices
#[tauri::command]
pub fn get_audio_playback_devices() -> CommandResponse<Vec<engine_core::AudioDevice>> {
    match engine_core::AudioProvider::get_playback_devices() {
        Ok(devices) => CommandResponse::ok(devices),
        Err(e) => CommandResponse::err(format!("Failed to get playback devices: {}", e)),
    }
}

/// get_audio_recording_devices: Get all recording devices
#[tauri::command]
pub fn get_audio_recording_devices() -> CommandResponse<Vec<engine_core::AudioDevice>> {
    match engine_core::AudioProvider::get_recording_devices() {
        Ok(devices) => CommandResponse::ok(devices),
        Err(e) => CommandResponse::err(format!("Failed to get recording devices: {}", e)),
    }
}

/// get_default_playback_device: Get default playback device
#[tauri::command]
pub fn get_default_playback_device() -> CommandResponse<Option<engine_core::AudioDevice>> {
    match engine_core::AudioProvider::get_default_playback_device() {
        Ok(device) => CommandResponse::ok(device),
        Err(e) => CommandResponse::err(format!("Failed to get default playback device: {}", e)),
    }
}

/// get_default_recording_device: Get default recording device
#[tauri::command]
pub fn get_default_recording_device() -> CommandResponse<Option<engine_core::AudioDevice>> {
    match engine_core::AudioProvider::get_default_recording_device() {
        Ok(device) => CommandResponse::ok(device),
        Err(e) => CommandResponse::err(format!("Failed to get default recording device: {}", e)),
    }
}

/// set_audio_volume: Set volume for a device (0-100)
#[tauri::command]
pub fn set_audio_volume(device_id: String, volume: f32) -> CommandResponse<String> {
    match engine_core::AudioProvider::set_volume(&device_id, volume) {
        Ok(_) => CommandResponse::ok(format!("Volume set to {}", volume)),
        Err(e) => CommandResponse::err(format!("Failed to set volume: {}", e)),
    }
}

/// get_network_interfaces: Get all network interfaces
#[tauri::command]
pub fn get_network_interfaces() -> CommandResponse<Vec<engine_core::NetworkInterface>> {
    match engine_core::NetworkProvider::get_interfaces() {
        Ok(interfaces) => CommandResponse::ok(interfaces),
        Err(e) => CommandResponse::err(format!("Failed to get network interfaces: {}", e)),
    }
}

/// get_default_network_interface: Get default network interface
#[tauri::command]
pub fn get_default_network_interface() -> CommandResponse<Option<engine_core::NetworkInterface>> {
    match engine_core::NetworkProvider::get_default_interface() {
        Ok(iface) => CommandResponse::ok(iface),
        Err(e) => CommandResponse::err(format!("Failed to get default interface: {}", e)),
    }
}

/// get_network_traffic: Get network traffic
#[tauri::command]
pub fn get_network_traffic() -> CommandResponse<engine_core::NetworkTraffic> {
    match engine_core::NetworkProvider::get_traffic() {
        Ok(traffic) => CommandResponse::ok(traffic),
        Err(e) => CommandResponse::err(format!("Failed to get network traffic: {}", e)),
    }
}

// =============================================================================
// BAR MANAGEMENT IPC COMMANDS
// =============================================================================

/// discover_available_bars: Get all available status bars
#[tauri::command]
pub fn discover_available_bars(state: tauri::State<SharedState>) -> CommandResponse<Vec<engine_core::BarInfo>> {
    match engine_core::discover_bars() {
        Ok(mut bars) => {
            // Mark current selected bar as active
            if let Ok(config) = state.config.try_read() {
                if let Some(selected_bar_id) = &config.global.selected_bar {
                    for bar in &mut bars {
                        bar.active = bar.id == *selected_bar_id;
                    }
                }
            }
            CommandResponse::ok(bars)
        }
        Err(e) => CommandResponse::err(format!("Failed to discover bars: {}", e)),
    }
}

/// select_bar: Set the active status bar
#[tauri::command]
pub fn select_bar(bar_id: String, state: tauri::State<SharedState>) -> CommandResponse<String> {
    // Verify bar exists
    match engine_core::get_bar(&bar_id) {
        Ok(_) => {
            // Update config
            let mut config = state.config.write();
            config.global.selected_bar = Some(bar_id.clone());
            
            match save_config_file(&config) {
                Ok(_) => {
                    // Emit event
                    if let Ok(config_json) = serde_json::to_value(&*config) {
                        let _ = engine_core::emit(StateEvent::ConfigReloaded {
                            config: config.clone(),
                        });
                    }
                    CommandResponse::ok(format!("Selected bar: {}", bar_id))
                }
                Err(e) => CommandResponse::err(format!("Failed to save config: {}", e)),
            }
        }
        Err(e) => CommandResponse::err(format!("Bar not found: {}", e)),
    }
}

/// get_selected_bar: Get currently selected status bar
#[tauri::command]
pub fn get_selected_bar(state: tauri::State<SharedState>) -> CommandResponse<Option<engine_core::BarInfo>> {
    let config = state.config.read();
    if let Some(selected_bar_id) = &config.global.selected_bar {
        match engine_core::get_bar(selected_bar_id) {
            Ok(bar_info) => CommandResponse::ok(Some(bar_info)),
            Err(e) => CommandResponse::err(format!("Selected bar not found: {}", e)),
        }
    } else {
        CommandResponse::ok(None)
    }
}

/// create_bar_scaffold: Create a new bar template
#[tauri::command]
pub fn create_bar_scaffold(bar_name: String) -> CommandResponse<String> {
    match engine_core::create_bar_scaffold(&bar_name) {
        Ok(bar_id) => {
            CommandResponse::ok(format!("Created bar at: ~/.unkai/bars/{}", bar_id))
        }
        Err(e) => CommandResponse::err(format!("Failed to create bar: {}", e)),
    }
}

/// get_bar_manifest: Get bar manifest details
#[tauri::command]
pub fn get_bar_manifest(bar_id: String) -> CommandResponse<engine_core::BarManifest> {
    match engine_core::get_bar_manifest(&bar_id) {
        Ok((_, manifest)) => CommandResponse::ok(manifest),
        Err(e) => CommandResponse::err(format!("Failed to get bar manifest: {}", e)),
    }
}
