# Configuration System Reference

Complete reference for YASB configuration including new bar management system.

---

## configuration.json (Global Config)

Located: `~/.unkai/config.json`

The global configuration file controls YASB behavior and bar selection.

### Schema

```json
{
  "selected_bar": "my-bar",
  "theme": "dark",
  "auto_start": true,
  "system_tray": {
    "show_icon": true,
    "context_menu": true
  }
}
```

### Fields

**`selected_bar`** (string, optional)
- ID of active status bar (directory name from `~/.unkai/bars/`)
- If not set, YASB uses first discovered bar
- Persists across restarts
- Set via Settings UI or QA API

```json
{
  "selected_bar": "my-status-bar"
}
```

**`theme`** (string)
- Color scheme: `"dark"` | `"light"`
- Applies to YASB UI components
- Default: `"dark"`

**`auto_start`** (boolean)
- Start YASB on Windows login
- Default: `false`

**`system_tray`** (object)
- `show_icon`: Show tray icon (default: true)
- `context_menu`: Enable right-click menu (default: true)

### Backward Compatibility

Existing configs without `selected_bar`:
- YASB detects on startup
- Automatically added with value `null`
- First bar from `~/.unkai/bars/` becomes active
- User can then select via Settings

### Manual Migration

To upgrade existing config:

```bash
# Backup
cp ~/.unkai/config.json ~/.unkai/config.json.backup

# Edit ~/.unkai/config.json, add:
{
  "selected_bar": null,
  ... existing fields ...
}
```

Or let YASB auto-upgrade on first run after update.

---

## Bar Configuration (barpack.json)

Located: `~/.unkai/bars/[bar-id]/barpack.json`

Manifest for each custom status bar.

### Schema

```json
{
  "name": "My Status Bar",
  "version": "1.0.0",
  "description": "Custom system metrics",
  "entry": "dist/index.html",
  "window": {
    "width": 1920,
    "height": 40,
    "offsetX": 0,
    "offsetY": 0,
    "anchor": "top_center",
    "alwaysOnTop": true,
    "transparent": true,
    "resizable": false,
    "shownInTaskbar": false
  },
  "metadata": {
    "author": "Your Name",
    "custom_field": "custom_value"
  }
}
```

### Required Fields

**`name`** (string, non-empty)
- Display name in YASB settings
- Example: `"Top System Bar"`

**`version`** (string)
- Semantic version `X.Y.Z`
- Example: `"1.0.0"`

**`entry`** (string, non-empty)
- Relative path to compiled HTML entry point
- Must exist: `~/.unkai/bars/[bar-id]/[entry]`
- Usually: `"dist/index.html"` (after build)

### Optional Fields

**`description`** (string)
- Human-readable description
- Shown in bar selector
- Example: `"Shows CPU, memory, and network metrics"`

**`window`** (object)
- Desktop window configuration
- All fields optional; sensible defaults provided

#### Window Configuration

```json
{
  "width": 1920,              // pixels, default: 1920
  "height": 40,               // pixels, default: 40
  "offsetX": 0,              // pixels from anchor, default: 0
  "offsetY": 0,              // pixels from anchor, default: 0
  "anchor": "top_center",    // position, default: "top_center"
  "alwaysOnTop": true,       // above other windows, default: true
  "transparent": true,        // transparent background, default: true
  "resizable": false,         // user can resize, default: false
  "shownInTaskbar": false    // show in Windows taskbar, default: false
}
```

**Anchor Positions:**
- `top_left` - ┌─────
- `top_center` - ┌──────┐  ← recommended for bars
- `top_right` - ─────┐
- `bottom_left` - └─────
- `bottom_center` - └──────┘
- `bottom_right` - ─────┘

**Size Guidelines:**
- Full-width top bar (recommended): 1920×40
- Half-width bar: 960×40
- Vertical sidebar: 50×1080
- Corner widget: 400×300

**Transparency:**
- `transparent: true` - Background color ignored, only content visible
- `transparent: false` - Background renders normally (use CSS background)

**Always On Top:**
- `alwaysOnTop: true` - Bar stays above all windows
- Often desired for always-visible status bar

**Metadata** (object, optional)
- Custom key-value data
- Ignored by YASB, can be used by bar developer
- Example:

```json
{
  "metadata": {
    "author": "John Doe",
    "repo": "https://github.com/user/bar",
    "theme": "dark",
    "features": ["cpu", "memory", "network"]
  }
}
```

---

## Config File Locations

```
Windows:
  Global config: C:\Users\[username]\.unkai\config.json
  Bar directory: C:\Users\[username]\.unkai\bars\
  Logs:          C:\Users\[username]\AppData\Local\YASB\logs\

macOS:
  Global config: ~/.unkai/config.json
  Bar directory: ~/.unkai/bars/
  Logs:          ~/.local/share/YASB/logs/

Linux:
  Global config: ~/.unkai/config.json
  Bar directory: ~/.unkai/bars/
  Logs:          ~/.local/share/YASB/logs/
```

---

## Programmatic Config Access

### From Rust (Backend)

```rust
use unkai_core::{load_config_file, save_config_file, GlobalConfig};

// Load config
let config = load_config_file()?;
let global = &config.global;
let selected_bar_id = &global.selected_bar;

// Modify and save
let mut config = load_config_file()?;
config.global.selected_bar = Some("my-bar".to_string());
save_config_file(&config)?;
```

### From TypeScript (Frontend)

```tsx
import { invoke } from '@tauri-apps/api/tauri'

// Get current config
const selected = await invoke('get_selected_bar')

// Update config
await invoke('select_bar', { barId: 'my-bar' })

// In React hook:
const { bar: selectedBar, selectBar } = useSelectedBar()
await selectBar('my-bar')
```

---

## Config Validation

### Global Config

YASB validates on load:
- `selected_bar` (optional): must be valid bar ID if set
- Other fields: type-checked at load time

Invalid configs:
- Missing `~/.unkai/` directory → auto-created
- Malformed JSON → error logged, defaults used
- Missing required fields → filled with defaults

### Bar Config (barpack.json)

YASB validates on discovery:
- `name`: must not be empty
- `entry`: must not be empty, file must exist
- Optional fields: ignored if invalid

Invalid bar manifests:
- Skipped during discovery
- Error logged: `Failed to load bar: [bar-id]`
- Cannot be activated until fixed

---

## Examples

### Minimal Config

```json
{
  "selected_bar": null
}
```

YASB uses all defaults.

### With Bar Selection

```json
{
  "selected_bar": "system-monitor"
}
```

Loads bar at `~/.unkai/bars/system-monitor/`

### Complete Config

```json
{
  "selected_bar": "top-bar",
  "theme": "dark",
  "auto_start": true,
  "system_tray": {
    "show_icon": true,
    "context_menu": true
  }
}
```

### Minimal Bar Manifest

```json
{
  "name": "Simple Bar",
  "version": "0.1.0",
  "entry": "dist/index.html"
}
```

Window uses all defaults (top-center, 1920×40, transparent, etc.)

### Complete Bar Manifest

```json
{
  "name": "Advanced Status Bar",
  "version": "2.1.3",
  "description": "Full-featured bar with system metrics, time, and app launcher",
  "entry": "dist/index.html",
  "window": {
    "width": 1920,
    "height": 48,
    "offsetX": 0,
    "offsetY": 0,
    "anchor": "top_center",
    "alwaysOnTop": true,
    "transparent": true,
    "resizable": false,
    "shownInTaskbar": false
  },
  "metadata": {
    "author": "Jane Developer",
    "repo": "https://github.com/jane/advanced-bar",
    "license": "MIT",
    "features": [
      "cpu_monitoring",
      "memory_usage",
      "network_speed",
      "battery_status",
      "clock",
      "workspace_indicator"
    ]
  }
}
```

---

## Migration Guide: Adding Bar Support

If you have YASB version < 1.0 without bar management:

### Step 1: Backup

```bash
cp ~/.unkai/config.json ~/.unkai/config.json.v0
```

### Step 2: Create Bars Directory

```bash
mkdir -p ~/.unkai/bars
```

### Step 3: Upgrade

When you run updated YASB:
- Detects missing `selected_bar` field
- Creates default value `null`
- Discovers bars in `~/.unkai/bars/`
- On next startup, uses first discovered bar

### Step 4: (Optional) Configure

Right-click YASB tray → Settings → Select Bar

---

## Troubleshooting

### Config not loading

**Error:** "Failed to load config file"

**Solutions:**
1. Check file exists: `ls ~/.unkai/config.json`
2. Check permissions: `chmod 644 ~/.unkai/config.json`
3. Verify JSON syntax: `cat ~/.unkai/config.json | jq`

### Bar not activating

**Error:** "Selected bar not found"

**Solutions:**
1. Verify bar exists: `ls -la ~/.unkai/bars/`
2. Check bar manifest: `cat ~/.unkai/bars/[bar-id]/barpack.json`
3. Verify entry file exists: `ls ~/.unkai/bars/[bar-id]/dist/index.html`

### Config keeps reverting

**Cause:** Permission or corruption

**Solutions:**
1. Check YASB process not running: `ps aux | grep yasb`
2. Verify file not read-only: `chmod u+w ~/.unkai/config.json`
3. Reset to defaults: `rm ~/.unkai/config.json` (restart YASB to recreate)

---

## API Reference

### IPC Commands

**`get_selected_bar()`**
```typescript
const bar: BarInfo | null = await invoke('get_selected_bar')
```

**`select_bar(bar_id: string)`**
```typescript
await invoke('select_bar', { barId: 'my-bar' })
```

**`discover_available_bars()`**
```typescript
const bars: BarInfo[] = await invoke('discover_available_bars')
```

**`get_bar_manifest(bar_id: string)`**
```typescript
const manifest: BarManifest = await invoke('get_bar_manifest', { barId: 'my-bar' })
```

**`create_bar_scaffold(bar_name: string)`**
```typescript
const path: string = await invoke('create_bar_scaffold', { barName: 'My Bar' })
```

---

See also:
- [Bar Creation Guide](BAR_CREATION_GUIDE.md)
- [Bar Management System](BAR_MANAGEMENT_SYSTEM.md)
- [UI Integration Guide](BAR_UI_INTEGRATION.md)

