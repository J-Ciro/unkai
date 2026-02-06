//! Scheduler: actualiza widgets periódicamente

use crate::{update_widget_data, SharedState, WidgetRuntime, DataSourceKind, WorkspaceState, update_workspace, komorebi};
use anyhow::Result;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{debug, error, warn};

/// Iniciar scheduler central
pub fn start_scheduler(state: SharedState) -> JoinHandle<()> {
    tokio::spawn(async move {
        debug!("Scheduler started");
        
        loop {
            // Obtener snapshot de widgets
            let widgets: Vec<_> = state
                .widgets
                .iter()
                .map(|entry| entry.value().clone())
                .collect();

            // Actualizar cada widget
            for widget in widgets {
                let state_clone = state.clone();
                
                // Spawnar tarea para cada widget (no bloquear scheduler principal)
                tokio::spawn(async move {
                    if let Err(e) = update_widget(&state_clone, &widget).await {
                        error!("Error updating widget {}: {}", widget.config.id, e);
                    }
                });
            }

            // Pequeña pausa para no saturar CPU
            sleep(Duration::from_millis(100)).await;
        }
    })
}

/// Actualizar un widget específico
async fn update_widget(state: &SharedState, widget: &WidgetRuntime) -> Result<()> {
    if !widget.config.enabled {
        return Ok(());
    }

    // Verificar si es momento de actualizar
    let now = chrono::Utc::now();
    let elapsed = now
        .signed_duration_since(widget.last_update)
        .num_milliseconds() as u64;

    if elapsed < widget.config.update_interval_ms {
        return Ok(());
    }

    debug!(
        "Updating widget: {} (interval: {}ms, elapsed: {}ms)",
        widget.config.id, widget.config.update_interval_ms, elapsed
    );

    // Ejecutar data sources
    for data_source in &widget.config.data_sources {
        if let Err(e) = fetch_data_source(state, &widget.config.id, data_source).await {
            error!(
                "Error fetching {} for widget {}: {}",
                data_source.name, widget.config.id, e
            );
        }
    }

    Ok(())
}

/// Fetch data de una fuente
async fn fetch_data_source(
    state: &SharedState,
    widget_id: &str,
    data_source: &crate::DataSourceConfig,
) -> Result<()> {
    let data = match data_source.kind {
        DataSourceKind::System => fetch_system_metrics().await?,
        DataSourceKind::Process => fetch_process_metrics().await?,
        DataSourceKind::Exec => {
            warn!("Exec data source not implemented yet");
            return Ok(());
        }
        DataSourceKind::Http => {
            warn!("Http data source not implemented yet");
            return Ok(());
        }
    };

    // Actualizar widget con nuevos datos
    update_widget_data(state, widget_id, data)?;

    Ok(())
}

/// Fetch system metrics (CPU, RAM, Network)
async fn fetch_system_metrics() -> Result<serde_json::Value> {
    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_percent = (used_memory as f64 / total_memory as f64) * 100.0;

    let mut total_cpu_percent = 0.0;
    for cpu in sys.cpus() {
        total_cpu_percent += cpu.cpu_usage();
    }
    if !sys.cpus().is_empty() {
        total_cpu_percent /= sys.cpus().len() as f32;
    }

    // Network stats (simplified - sysinfo 0.30 requires Networks type)
    let total_bytes_in = 0u64;
    let total_bytes_out = 0u64;

    Ok(serde_json::json!({
        "cpu_percent": total_cpu_percent,
        "memory_percent": memory_percent,
        "memory_mb": used_memory / 1024,
        "total_memory_mb": total_memory / 1024,
        "network_in_bytes": total_bytes_in,
        "network_out_bytes": total_bytes_out,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Fetch process metrics
async fn fetch_process_metrics() -> Result<serde_json::Value> {
    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut top_processes = Vec::new();

    for (pid, process) in sys.processes().iter().take(10) {
        top_processes.push(serde_json::json!({
            "pid": pid.as_u32(),
            "name": process.name(),
            "cpu_percent": process.cpu_usage(),
            "memory_mb": process.memory() / 1024,
        }));
    }

    Ok(serde_json::json!({
        "top_processes": top_processes,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Iniciar listener de eventos de Komorebi
/// Se conecta al socket de Komorebi y actualiza estado de workspace
pub fn start_komorebi_listener(state: SharedState) -> JoinHandle<()> {
    tokio::spawn(async move {
        debug!("Komorebi listener starting...");
        
        let mut client = komorebi::KomorebiClient::new();
        
        match client.subscribe_events().await {
            Ok(mut listener) => {
                debug!("Connected to Komorebi socket");
                
                loop {
                    match listener.next().await {
                        Ok(event) => {
                            debug!("Komorebi event received: WS {} layout {}", 
                                event.workspace_id, event.layout);
                            
                            let workspace = WorkspaceState {
                                workspace_id: event.workspace_id,
                                monitor_index: event.monitor_index,
                                layout: event.layout,
                                window_count: event.window_count,
                                focused: event.focused,
                            };
                            
                            if let Err(e) = update_workspace(&state, workspace) {
                                error!("Failed to update workspace: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Komorebi listener error: {}", e);
                            // Intentar reconectar después de un delay
                            sleep(Duration::from_secs(5)).await;
                            
                            match client.subscribe_events().await {
                                Ok(new_listener) => {
                                    debug!("Komorebi reconnected");
                                    listener = new_listener;
                                }
                                Err(e) => {
                                    warn!("Failed to reconnect to Komorebi: {}", e);
                                    // Seguir intentando en bucle
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Failed to connect to Komorebi: {}", e);
                warn!("Workspace state will not update from Komorebi events");
                warn!("Make sure Komorebi is running: https://github.com/LGUG2Z/komorebi");
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[tokio::test]
    async fn test_scheduler_starts() {
        let config = Config {
            version: "1.0".to_string(),
            widgets: vec![],
            styles: Default::default(),
            displays: vec![],
            global: Default::default(),
        };

        let state = crate::init(config).unwrap();
        let handle = start_scheduler(state);

        // Esperar un bit para que el scheduler arranque
        tokio::time::sleep(Duration::from_millis(100)).await;

        // No debería haber panics
        assert!(!handle.is_finished());

        handle.abort();
    }

    #[tokio::test]
    async fn test_fetch_system_metrics() {
        let result = fetch_system_metrics().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert!(data.get("cpu_percent").is_some());
        assert!(data.get("memory_percent").is_some());
        assert!(data.get("memory_mb").is_some());
    }

    #[tokio::test]
    async fn test_fetch_process_metrics() {
        let result = fetch_process_metrics().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert!(data.get("top_processes").is_some());
    }
}
