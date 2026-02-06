#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ipc;

use engine_core::{init, load_config, get_config_path, create_default_config, start_scheduler, start_komorebi_listener};
use tauri::Manager;

fn main() {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Cargar config
    let config = match get_config_path().and_then(load_config) {
        Ok(cfg) => cfg,
        Err(_) => {
            // Crear default si no existe
            match get_config_path().and_then(|p| create_default_config(&p)) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Failed to load or create config: {}", e);
                    std::process::exit(1);
                }
            }
        }
    };

    // Inicializar engine-core
    let state = match init(config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize engine-core: {}", e);
            std::process::exit(1);
        }
    };

    // Iniciar scheduler en background thread con tokio runtime
    let state_clone = state.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            start_scheduler(state_clone).await;
        });
    });

    // Iniciar Komorebi listener (opcional, solo si Komorebi está disponible)
    let state_komorebi = state.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            start_komorebi_listener(state_komorebi).await;
        });
    });

    // Tauri app
    tauri::Builder::default()
        .manage(state.clone())
        .setup(|app| {
            let handle = app.handle().clone();
            let state_clone = state.clone();

            // Spawnar task de broadcast de eventos
            tauri::async_runtime::spawn(ipc::broadcast_events(state_clone, handle.clone()));

            handle.emit_all("app:ready", "unkai backend ready").ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::get_config,
            ipc::set_config,
            ipc::reload_config,
            ipc::get_widget_state,
            ipc::get_all_widgets,
            ipc::run_widget_action,
            ipc::export_widgets,
            // System providers
            ipc::get_memory_stats,
            ipc::get_battery_status,
            ipc::get_audio_playback_devices,
            ipc::get_audio_recording_devices,
            ipc::get_default_playback_device,
            ipc::get_default_recording_device,
            ipc::set_audio_volume,
            ipc::get_network_interfaces,
            ipc::get_default_network_interface,
            ipc::get_network_traffic,
            // Bar management
            ipc::discover_available_bars,
            ipc::select_bar,
            ipc::get_selected_bar,
            ipc::create_bar_scaffold,
            ipc::get_bar_manifest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
