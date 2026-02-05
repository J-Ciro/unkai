//! Komorebi socket IPC client
//! 
//! Connects to Komorebi tiling window manager via Unix socket
//! Receives workspace/layout changes and manages state

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::task::JoinHandle;

/// Komorebi socket path
pub fn komorebi_socket_path() -> PathBuf {
    // On Windows with Komorebi, socket is typically in user home
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".komorebi.sock")
}

/// Komorebi workspace event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEvent {
    pub workspace_id: i32,
    pub monitor_index: i32,
    pub layout: String,
    pub window_count: i32,
    pub focused: bool,
}

/// Komorebi layout modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LayoutMode {
    BSP,
    Columns,
    Rows,
    Grid,
    Maximized,
}

impl LayoutMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            LayoutMode::BSP => "bsp",
            LayoutMode::Columns => "columns",
            LayoutMode::Rows => "rows",
            LayoutMode::Grid => "grid",
            LayoutMode::Maximized => "maximized",
        }
    }
}

/// Komorebi state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KomorebiState {
    pub workspace_id: i32,
    pub monitor_index: i32,
    pub layout: LayoutMode,
    pub window_count: i32,
    pub focused_window: Option<String>,
    pub focused: bool,
}

/// Komorebi socket client
pub struct KomorebiClient {
    socket: Option<UnixStream>,
    path: PathBuf,
}

impl KomorebiClient {
    pub fn new() -> Self {
        Self {
            socket: None,
            path: komorebi_socket_path(),
        }
    }

    /// Connect to Komorebi socket
    pub async fn connect(&mut self) -> Result<()> {
        match UnixStream::connect(&self.path).await {
            Ok(socket) => {
                self.socket = Some(socket);
                Ok(())
            }
            Err(e) => Err(anyhow!("Failed to connect to Komorebi socket: {}", e)),
        }
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.socket.is_some()
    }

    /// Send command to Komorebi
    pub async fn send_command(&mut self, command: &str) -> Result<String> {
        let socket = self
            .socket
            .as_mut()
            .ok_or_else(|| anyhow!("Not connected to Komorebi"))?;

        // Send command
        socket.write_all(command.as_bytes()).await?;
        socket.write_all(b"\n").await?;

        // Read response
        let mut reader = BufReader::new(socket);
        let mut response = String::new();
        reader.read_line(&mut response).await?;

        Ok(response)
    }

    /// Query current workspace state
    pub async fn query_state(&mut self) -> Result<KomorebiState> {
        let response = self.send_command("state").await?;
        let state: KomorebiState = serde_json::from_str(&response)?;
        Ok(state)
    }

    /// Subscribe to workspace events
    pub async fn subscribe_events(&mut self) -> Result<WorkspaceEventListener> {
        self.connect().await?;
        self.send_command("subscribe").await?;

        let socket = self
            .socket
            .take()
            .ok_or_else(|| anyhow!("Socket lost"))?;

        Ok(WorkspaceEventListener {
            reader: BufReader::new(socket),
        })
    }
}

impl Default for KomorebiClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Listener for workspace events from Komorebi
pub struct WorkspaceEventListener {
    reader: BufReader<UnixStream>,
}

impl WorkspaceEventListener {
    /// Wait for next workspace event
    pub async fn next(&mut self) -> Result<WorkspaceEvent> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;

        let event: WorkspaceEvent = serde_json::from_str(&line)?;
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_mode_str() {
        assert_eq!(LayoutMode::BSP.as_str(), "bsp");
        assert_eq!(LayoutMode::Columns.as_str(), "columns");
        assert_eq!(LayoutMode::Maximized.as_str(), "maximized");
    }

    #[test]
    fn test_komorebi_state_serialization() {
        let state = KomorebiState {
            workspace_id: 1,
            monitor_index: 0,
            layout: LayoutMode::BSP,
            window_count: 3,
            focused_window: Some("chrome.exe".to_string()),
            focused: true,
        };

        let json = serde_json::to_string(&state).unwrap();
        let parsed: KomorebiState = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.workspace_id, 1);
        assert_eq!(parsed.layout, LayoutMode::BSP);
    }

    #[test]
    fn test_workspace_event_parsing() {
        let json = r#"{
            "workspace_id": 1,
            "monitor_index": 0,
            "layout": "bsp",
            "window_count": 2,
            "focused": true
        }"#;

        let event: WorkspaceEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.workspace_id, 1);
        assert_eq!(event.window_count, 2);
        assert!(event.focused);
    }
}
