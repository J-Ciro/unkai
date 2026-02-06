# Bar System Architecture & Developer Guide

Complete technical guide to YASB's bar management system.

---

## System Overview

```
┌─────────────────────────────────────────────────┐
│ User's statusbar apps (~/.unkai/bars/)          │
│  - bar1/ (React app)                            │
│  - bar2/ (React app)                            │
│  - bar3/ (React app)                            │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────┐
│ Bar Discovery Service (Rust backend)             │
│  - Scans ~/.unkai/bars/                          │
│  - Loads barpack.json manifests                  │
│  - Validates configurations                      │
│  - Creates new bar scaffolds                     │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────┐
│ Global Config (config.json)                      │
│  - selected_bar: ID of active bar               │
│  - Persisted across restarts                     │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────┐
│ IPC Layer (Tauri commands)                       │
│  - discover_available_bars()                     │
│  - select_bar(id)                                │
│  - get_selected_bar()                            │
│  - create_bar_scaffold(name)                     │
│  - get_bar_manifest(id)                          │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────┐
│ Frontend Provider (BarManager)                   │
│  - Wraps IPC commands                            │
│  - Manages event system                          │
│  - Caches state                                  │
│  - Type-safe API                                 │
└──────────────────┬──────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────┐
│ React Hooks & Components                         │
│  - useBarManager()                               │
│  - useAvailableBars()                            │
│  - useSelectedBar()                              │
│  - BarSettingsPanel (UI)                         │
│  - BarSelector, BarsList, CreateBarForm          │
└──────────────────────────────────────────────────┘
```

---

## Component Deep Dive

### 1. Backend Discovery (Rust)

**Location:** `crates/engine-core/src/bar_discovery.rs`

#### Core Functions

```rust
pub fn discover_bars() -> Result<Vec<BarInfo>>
```
- Scans `~/.unkai/bars/` directory
- Loads `barpack.json` from each subdirectory
- Returns list of available bars with metadata
- Auto-creates directory if missing

```rust
pub fn load_bar_manifest(path: &Path) -> Result<BarManifest>
```
- Parses JSON manifest file
- Validates required fields (name, entry, window)
- Returns structured manifest
- Errors on invalid JSON or missing fields

```rust
pub fn create_bar_scaffold(bar_name: &str) -> Result<String>
```
- Creates complete bar structure:
  - `barpack.json` with defaults
  - `package.json` with dependencies
  - `vite.config.ts` build configuration
  - `src/main.tsx` React entry
  - `src/App.tsx` example component
  - `src/index.css` styling
- Returns bar ID (directory name)
- Used by "Create Bar" feature

**Example:**
```rust
// User creates bar named "My Status Bar"
let bar_id = create_bar_scaffold("My Status Bar")?;
// Creates: ~/.unkai/bars/my-status-bar/ with all files
```

#### Type System

```rust
pub struct BarManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub entry: String,
    pub window: BarWindowConfig,
    pub metadata: Option<HashMap<String, String>>,
}

pub struct BarInfo {
    pub id: String,              // directory name
    pub name: String,            // from manifest
    pub description: Option<String>,
    pub version: String,
    pub path: PathBuf,           // full filesystem path
    pub active: bool,            // = config.selected_bar
}

pub struct BarWindowConfig {
    pub width: u32,              // pixels
    pub height: u32,             // pixels
    pub offset_x: i32,           // from anchor
    pub offset_y: i32,
    pub anchor: BarAnchor,       // enum
    pub always_on_top: bool,
    pub transparent: bool,
    pub resizable: bool,
    pub shown_in_taskbar: bool,
}

pub enum BarAnchor {
    TopLeft, TopCenter, TopRight,
    BottomLeft, BottomCenter, BottomRight,
}
```

### 2. IPC Commands (Backend-Frontend Bridge)

**Location:** `crates/tauri-backend/src/ipc.rs`

#### Command Implementations

```rust
pub async fn discover_available_bars(state: State<'_, AppState>) 
    -> CommandResponse<Vec<BarInfo>>
```
- Calls `discover_bars()`
- Marks active bar (compares ID to `state.config.global.selected_bar`)
- Wraps in CommandResponse for error handling
- Returns JSON with bars list

```rust
pub async fn select_bar(
    bar_id: String,
    state: State<'_, AppState>,
) -> CommandResponse<String>
```
- Validates bar exists
- Updates `state.config.global.selected_bar`
- Saves config to disk
- Emits `StateEvent::ConfigReloaded`
- Returns success message

```rust
pub async fn create_bar_scaffold(bar_name: String) 
    -> CommandResponse<String>
```
- Calls `create_bar_scaffold()`
- Returns path to created bar
- Auto-refreshes discovery

#### Error Handling

All commands wrap responses:

```rust
pub struct CommandResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub code: u32,
}
```

Errors bubble through IPC with context.

### 3. Frontend Provider (Type-Safe API Wrapper)

**Location:** `frontend/web/lib/providers/bar.ts`

#### BarManager Class

```typescript
class BarManager {
  // State
  private availableBars: BarInfo[] = []
  private selectedBar: BarInfo | null = null
  private isLoading = false
  private error: string | null = null
  private eventListeners = new Map()

  // Methods
  async discoverBars(): Promise<BarInfo[]>
  async selectBar(barId: string): Promise<void>
  async getSelectedBar(): Promise<BarInfo | null>
  async getBarManifest(barId: string): Promise<BarManifest | null>
  async createBarScaffold(barName: string): Promise<string>

  // Events
  on(event: 'bars:discovered' | 'bar:selected', listener: Function): () => void
  private emit(event: string, data: any): void

  // State
  getState(): BarManagerState
}
```

**Singleton Pattern:**
```typescript
// Get or create global instance
const barManager = getBarManager()

// Or create new instance
const barManager = createBarManager()
```

**Event Flow Example:**
```typescript
// User clicks "Refresh Bars"
await barManager.discoverBars()
  ↓
// Invokes IPC command
invoke('discover_available_bars')
  ↓
// Backend scans filesystem
// Backend returns Vec<BarInfo>
  ↓
// Frontend updates state.availableBars
// Frontend emits 'bars:discovered' event
  ↓
// React hooks catch event
// React re-renders components
// UI shows updated bars
```

### 4. React Hooks (State Management)

**Location:** `frontend/web/lib/hooks/useBarManager.ts`

#### Hook 1: useBarManager()
```typescript
function useBarManager() {
  // Returns current manager state + methods
  return {
    availableBars,
    selectedBar,
    isLoading,
    error,
    discoverBars,
    selectBar,
    getSelectedBar,
    createBar,
  }
}
```

#### Hook 2: useAvailableBars()
```typescript
function useAvailableBars() {
  // Returns bars list with helper methods
  return {
    bars,               // BarInfo[]
    isLoading,
    error,
    refresh,            // () => Promise<void>
  }
}
```

**Usage:**
```tsx
const { bars, refresh } = useAvailableBars()

useEffect(() => {
  refresh() // Load bars on component mount
}, [])

return (
  <button onClick={refresh}>Refresh</button>
)
```

#### Hook 3: useSelectedBar()
```typescript
function useSelectedBar() {
  // Returns currently selected bar + selection method
  return {
    bar,                // BarInfo | null
    isLoading,
    selectBar,          // (id: string) => Promise<void>
  }
}
```

**Usage:**
```tsx
const { bar, selectBar } = useSelectedBar()

return (
  <div>
    Active: {bar?.name}
    <button onClick={() => selectBar('other-bar')}>
      Switch Bar
    </button>
  </div>
)
```

#### Auto-Subscription

Hooks automatically:
1. Subscribe to provider events on mount
2. Update state when events fire
3. Unsubscribe on unmount (cleanup)
4. Handle loading/error states

### 5. React Components (UI)

**Location:** `frontend/web/components/BarManager.tsx`

#### Component 1: BarSelector
```tsx
<select>
  <option>Choose a bar...</option>
  {bars.map(bar => <option>{bar.name}</option>)}
</select>
```
- Dropdown list of bars
- Shows current selection
- Change handler calls selectBar()
- Displays bar metadata

#### Component 2: BarsList
```tsx
<div className="grid">
  {bars.map(bar => <BarCard bar={bar} />)}
</div>
```
- Grid of bar cards (auto-layout)
- Shows: name, version, description, id
- "Active" badge on selected bar
- Click to select
- Refresh button

#### Component 3: CreateBarForm
```tsx
<input placeholder="Bar name" />
<button>Create</button>
```
- Text input for bar name
- Submit button
- Validation
- Status messages
- Auto-refresh on creation

#### Component 4: BarSettingsPanel
```tsx
<tabs>
  <tab>Select Bar → <BarSelector /></tab>
  <tab>All Bars → <BarsList /></tab>
  <tab>Create Bar → <CreateBarForm /></tab>
</tabs>
```
- Tabbed interface combining all above
- Latest tab styling
- Complete CSS included
- Dark theme by default

#### Component 5: QuickBarSelector
```tsx
<div>
  {bars.map(bar => (
    <button onClick={() => selectBar(bar.id)}>
      {bar.name} {bar.active ? '✓' : ''}
    </button>
  ))}
</div>
```
- Compact button list
- For tray menu integration
- Minimal UI

---

## Data Flow: Complete Example

### Scenario: User Creates a Bar

```
1. User clicks "Create Bar" in YASB Settings UI

2. Frontend: CreateBarForm
   └─ Gets bar name from input
   └─ Calls useBarManager().createBar('my-bar')

3. React Hook: useBarManager
   └─ Calls BarManager.createBarScaffold('my-bar')

4. Provider: BarManager
   └─ Invokes IPC: invoke('create_bar_scaffold', { barName: 'my-bar' })

5. IPC Layer: Tauri
   └─ Routes to create_bar_scaffold command

6. Backend: crates/tauri-backend/ipc.rs
   └─ Calls engine_core::create_bar_scaffold()

7. Backend: crates/engine-core/bar_discovery.rs
   └─ Creates ~/.unkai/bars/my-bar/
   └─ Generates barpack.json ✓
   └─ Generates package.json ✓
   └─ Generates index.html ✓
   └─ Generates src/main.tsx ✓
   └─ Generates src/App.tsx ✓
   └─ Generates vite.config.ts ✓
   └─ Returns path string

8. IPC Response
   └─ Returns path to frontend

9. Provider: BarManager
   └─ Receives response
   └─ Auto-calls discoverBars()
   └─ Emits 'bars:discovered' event

10. React Hook: useAvailableBars
    └─ Listens for 'bars:discovered'
    └─ Updates bars state
    └─ Triggers re-render

11. Component: BarsList
    └─ Re-renders with new bar
    └─ Shows "my-bar" in grid

12. User sees new bar in UI ✓
    └─ Click to activate
    └─ Bar becomes active
```

---

## Filesystem Layout

```
~/.unkai/
├── config.json
│   └─ { "selected_bar": "top-bar", ... }
│
├── bars/
│   ├── bar-1/
│   │   ├── barpack.json
│   │   ├── package.json
│   │   ├── vite.config.ts
│   │   ├── index.html
│   │   ├── src/
│   │   │   ├── main.tsx
│   │   │   ├── App.tsx
│   │   │   └── index.css
│   │   └── dist/              (after npm run build)
│   │       ├── index.html
│   │       └── assets/
│   │
│   └── bar-2/
│       ├── barpack.json
│       ├── dist/
│       │   └── index.html     (active bar)
│       └── ...
```

**Key Pattern:**
- Each bar is self-contained directory
- Contains source + build output (dist/)
- Uses separate barpack.json manifest
- No dependencies between bars

---

## Event System

### Events Emitted

**'bars:discovered'**
```typescript
manager.on('bars:discovered', (bars: BarInfo[]) => {
  console.log('Found bars:', bars)
  // Update UI with new bars list
})
```
- Fired after successful discovery
- Contains full bars array
- Used by useAvailableBars() to update

**'bar:selected'**
```typescript
manager.on('bar:selected', (bar: BarInfo) => {
  console.log('Selected:', bar.name)
  // Update UI to show new selection
})
```
- Fired after successful selection
- Contains selected bar info
- Used by useSelectedBar() to update

### Subscription Pattern

```tsx
useEffect(() => {
  // Subscribe
  const unsubscribe = manager.on('bars:discovered', handleBarsUpdated)
  
  // Cleanup on unmount
  return () => unsubscribe()
}, [])
```

---

## Error Handling Strategy

### Levels

**1. Backend (Rust)**
```rust
pub fn discover_bars() -> Result<Vec<BarInfo>> {
  // Returns Err if:
  // - bars directory doesn't exist (auto-created)
  // - invalid barpack.json (skipped with log)
  // - manifest validation fails (skipped)
}
```

**2. IPC (Tauri)**
```rust
pub async fn discover_available_bars(state: ...) 
    -> CommandResponse<Vec<BarInfo>> {
  // Wraps errors in CommandResponse
  // Returns { data: None, error: "message", code: 1 }
}
```

**3. Provider (Frontend)**
```typescript
async discoverBars(): Promise<BarInfo[]> {
  try {
    const result = await invoke('discover_available_bars')
    if (result.error) {
      this.error = result.error
      return []
    }
    return result.data
  } catch (e) {
    this.error = e.message
    return []
  }
}
```

**4. Hooks (React)**
```typescript
const { bars, error } = useAvailableBars()

if (error) {
  return <div className="error">{error}</div>
}
```

**5. Components (UI)**
```tsx
export function BarsList() {
  const { bars, error } = useAvailableBars()
  
  return (
    <>
      {error && <ErrorBanner message={error} />}
      {bars.map(bar => <BarCard key={bar.id} bar={bar} />)}
    </>
  )
}
```

### Error Recovery

- Discoverable errors auto-skipped (bar still shows in list)
- Selection errors notify user via error message
- Config save errors prevent bar switch
- IPC timeouts return error in response

---

## Testing Strategy

### Unit Tests (Backend)

**bar.rs**
```rust
#[test]
fn test_bar_anchor_display() { ... }

#[test]
fn test_bar_manifest_validation() { ... }

#[test]
fn test_bar_window_config_defaults() { ... }
```

**bar_discovery.rs**
```rust
#[test]
fn test_discover_empty_directory() { ... }

#[test]
fn test_load_valid_manifest() { ... }

#[test]
fn test_create_bar_scaffold() { ... }
```

### Integration Tests (IPC)

```typescript
describe('Bar Management IPC', () => {
  test('discover_available_bars returns list', async () => {
    const result = await invoke('discover_available_bars')
    expect(result.data).toBeInstanceOf(Array)
  })

  test('select_bar persists to config', async () => {
    await invoke('select_bar', { barId: 'test-bar' })
    const selected = await invoke('get_selected_bar')
    expect(selected.data.id).toBe('test-bar')
  })
})
```

### Component Tests (React)

```tsx
import { render, screen } from '@testing-library/react'
import { BarsList } from '@/components/BarManager'

test('renders bar list', () => {
  render(<BarsList />)
  expect(screen.getByText('Available Bars')).toBeInTheDocument()
})
```

---

## Performance Considerations

### Caching

```typescript
// BarManager caches results
private availableBars: BarInfo[] = []
private selectedBar: BarInfo | null = null

// Only refresh on explicit request or event
discoverBars() // ← explicit refresh
// or
'bars:discovered' event // ← from backend
```

### Polling vs Events

```typescript
// ❌ Bad: Poll every second
setInterval(() => discoverBars(), 1000)

// ✅ Good: Listen for changes
manager.on('bars:discovered', refresh)
```

### Memoization

```tsx
// ✅ Prevent unnecessary re-renders
const MemoizedBarCard = React.memo(BarCard)

export function BarsList() {
  return (
    <div>
      {bars.map(bar => (
        <MemoizedBarCard key={bar.id} bar={bar} />
      ))}
    </div>
  )
}
```

---

## Security Considerations

### Path Traversal Protection

```rust
// ✓ Safe: Only reads from bars directory
fn discover_bars() -> Result<Vec<BarInfo>> {
  let bars_dir = get_bars_directory()?  // ~/.unkai/bars only
  // Only reads immediate subdirectories
}

// ✓ Safe: Validates bar ID before use
fn get_bar_entry_path(bar_id: &str) -> Result<PathBuf> {
  let path = get_bars_directory()?.join(bar_id)
  // Checks path is canonical (no ..)
  path.canonicalize()?
}
```

### Input Validation

```rust
// ✓ Safe: Validates bar name
pub fn create_bar_scaffold(bar_name: &str) -> Result<String> {
  if bar_name.is_empty() {
    return Err("Bar name cannot be empty");
  }
  if bar_name.contains('/') || bar_name.contains('\\') {
    return Err("Invalid characters in bar name");
  }
  // Only creates in bars directory
}
```

### Sandboxing

- Bar processes run as separate child windows
- Each bar isolated by Tauri
- Bars cannot access other bars' data
- File access scoped to bar directory

---

## Migration Guide: Phase 5b to Phase 3

If upgrading existing YASB:

1. **Backup Config**
   ```bash
   cp ~/.unkai/config.json ~/.unkai/config.json.backup
   ```

2. **Auto-Upgrade**
   - YASB detects missing `selected_bar` field
   - Adds field with value `null` on first run
   - No manual action needed

3. **Create Bars Directory**
   ```bash
   mkdir -p ~/.unkai/bars
   ```

4. **Build New Backend**
   ```bash
   cargo build --release
   ```

5. **Deploy Frontend**
   - Components automatically included
   - No code changes needed in existing UI
   - Add `<BarSettingsPanel />` where desired

---

## API Reference Summary

### Rust Backend
- `discover_bars() -> Result<Vec<BarInfo>>`
- `load_bar_manifest(&Path) -> Result<BarManifest>`
- `create_bar_scaffold(&str) -> Result<String>`
- `get_bar(&str) -> Result<BarInfo>`
- `get_bar_entry_path(&str) -> Result<PathBuf>`

### Tauri IPC Commands
- `discover_available_bars() -> CommandResponse<Vec<BarInfo>>`
- `select_bar(bar_id: String) -> CommandResponse<String>`
- `get_selected_bar() -> CommandResponse<Option<BarInfo>>`
- `create_bar_scaffold(bar_name: String) -> CommandResponse<String>`
- `get_bar_manifest(bar_id: String) -> CommandResponse<BarManifest>`

### TypeScript Provider
- `BarManager.discoverBars() -> Promise<BarInfo[]>`
- `BarManager.selectBar(barId: string) -> Promise<void>`
- `BarManager.getSelectedBar() -> Promise<BarInfo | null>`
- `BarManager.getBarManifest(barId: string) -> Promise<BarManifest | null>`
- `BarManager.createBarScaffold(barName: string) -> Promise<string>`

### React Hooks
- `useBarManager() -> BarManagerState & Methods`
- `useAvailableBars() -> { bars, isLoading, error, refresh }`
- `useSelectedBar() -> { bar, isLoading, selectBar }`

### React Components
- `<BarSettingsPanel />` - Complete tabbed UI
- `<BarSelector />` - Dropdown selector
- `<BarsList />` - Grid view
- `<CreateBarForm />` - Creation form
- `<QuickBarSelector />` - Compact button list

---

See also:
- [Bar Management System](BAR_MANAGEMENT_SYSTEM.md)
- [Bar Creation Guide](BAR_CREATION_GUIDE.md)
- [Config Reference](CONFIG_REFERENCE.md)
- [UI Integration Guide](BAR_UI_INTEGRATION.md)

