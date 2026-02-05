/**
 * Bar discovery and management
 * Scans ~/.unkai/bar/ for available custom status bars
 */

use crate::bar::{BarAnchor, BarInfo, BarManifest, BarWindowConfig};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Default bar directory path
pub fn get_bars_directory() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    Ok(home.join(".unkai").join("bars"))
}

/// Scan for available bars
pub fn discover_bars() -> Result<Vec<BarInfo>> {
    let bars_dir = get_bars_directory()?;

    if !bars_dir.exists() {
        fs::create_dir_all(&bars_dir)?;
        return Ok(vec![]);
    }

    let mut bars = vec![];

    for entry in fs::read_dir(&bars_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        // Look for barpack.json in directory
        let manifest_path = path.join("barpack.json");
        if !manifest_path.exists() {
            continue;
        }

        match load_bar_manifest(&manifest_path) {
            Ok(manifest) => {
                let bar_id = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                bars.push(BarInfo {
                    id: bar_id,
                    name: manifest.name,
                    description: manifest.description,
                    version: manifest.version,
                    path: path.to_string_lossy().to_string(),
                    active: false,
                });
            }
            Err(e) => {
                eprintln!("Failed to load bar from {:?}: {}", manifest_path, e);
            }
        }
    }

    Ok(bars)
}

/// Load bar manifest from file
pub fn load_bar_manifest(path: &Path) -> Result<BarManifest> {
    let content = fs::read_to_string(path)?;
    let manifest: BarManifest = serde_json::from_str(&content)?;
    manifest.validate()?;
    Ok(manifest)
}

/// Get bar by ID
pub fn get_bar(bar_id: &str) -> Result<BarInfo> {
    let bars = discover_bars()?;
    bars.into_iter()
        .find(|b| b.id == bar_id)
        .ok_or_else(|| anyhow::anyhow!("Bar not found: {}", bar_id))
}

/// Get bar manifest and full path
pub fn get_bar_manifest(bar_id: &str) -> Result<(BarInfo, BarManifest)> {
    let bar_info = get_bar(bar_id)?;
    let manifest_path = PathBuf::from(&bar_info.path).join("barpack.json");
    let manifest = load_bar_manifest(&manifest_path)?;
    Ok((bar_info, manifest))
}

/// Get full path to bar's entry HTML
pub fn get_bar_entry_path(bar_id: &str) -> Result<PathBuf> {
    let bar_info = get_bar(bar_id)?;
    let manifest_path = PathBuf::from(&bar_info.path).join("barpack.json");
    let manifest = load_bar_manifest(&manifest_path)?;

    let entry_path = PathBuf::from(&bar_info.path).join(&manifest.entry);
    if !entry_path.exists() {
        anyhow::bail!(
            "Bar entry not found at: {}",
            entry_path.to_string_lossy()
        );
    }

    Ok(entry_path)
}

/// Create a new bar scaffold
pub fn create_bar_scaffold(bar_name: &str) -> Result<String> {
    let bars_dir = get_bars_directory()?;
    fs::create_dir_all(&bars_dir)?;

    let bar_id = bar_name.to_lowercase().replace(" ", "-");
    let bar_path = bars_dir.join(&bar_id);

    if bar_path.exists() {
        anyhow::bail!("Bar already exists: {}", bar_id);
    }

    fs::create_dir_all(&bar_path)?;

    // Create barpack.json
    let manifest = BarManifest {
        name: bar_name.to_string(),
        version: "0.1.0".to_string(),
        description: format!("Custom status bar: {}", bar_name),
        entry: "dist/index.html".to_string(),
        window: BarWindowConfig::default(),
        metadata: Default::default(),
    };

    let manifest_content = serde_json::to_string_pretty(&manifest)?;
    fs::write(bar_path.join("barpack.json"), manifest_content)?;

    // Create package.json
    let package_json = r#"{
  "name": "unkai-bar-example",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^18.0.0",
    "react-dom": "^18.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "@vitejs/plugin-react": "^4.0.0",
    "vite": "^4.0.0"
  }
}"#;
    fs::write(bar_path.join("package.json"), package_json)?;

    // Create src/main.tsx
    let src_dir = bar_path.join("src");
    fs::create_dir_all(&src_dir)?;

    let main_tsx = r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
"#;
    fs::write(src_dir.join("main.tsx"), main_tsx)?;

    // Create src/App.tsx
    let app_tsx = r#"import { useSystemProviders } from '@unkai/sdk'

export default function App() {
  const { memory, battery, network } = useSystemProviders({ autoPoll: true })

  return (
    <div className="status-bar">
      <span>💾 {memory.usagePercent?.toFixed(0)}%</span>
      <span>🔋 {battery.status?.chargePercent?.toFixed(0)}%</span>
      <span>🌐 {((network.traffic?.receivedPerSec || 0) * 8 / 1_000_000).toFixed(1)} Mbps</span>
    </div>
  )
}
"#;
    fs::write(src_dir.join("App.tsx"), app_tsx)?;

    // Create src/index.css
    let index_css = r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: transparent;
  color: #e0e0e0;
}

#root {
  width: 100%;
  height: 100%;
}

.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  padding: 8px 16px;
  background: rgba(30, 30, 30, 0.95);
  backdrop-filter: blur(10px);
  gap: 24px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 13px;
}

.status-bar span {
  display: flex;
  align-items: center;
  gap: 6px;
}
"#;
    fs::write(src_dir.join("index.css"), index_css)?;

    // Create vite.config.ts
    let vite_config = r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  }
})
"#;
    fs::write(bar_path.join("vite.config.ts"), vite_config)?;

    // Create index.html
    let index_html = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>YASB Bar</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"#;
    fs::write(bar_path.join("index.html"), index_html)?;

    Ok(bar_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_anchor_window_config() {
        let config = BarWindowConfig {
            width: 1920,
            height: 40,
            offset_x: 5,
            offset_y: 5,
            anchor: BarAnchor::TopCenter,
            always_on_top: true,
            transparent: true,
            resizable: false,
            shown_in_taskbar: false,
        };

        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 40);
    }
}
