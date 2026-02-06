# Frontend UI Integration Guide

Add bar management to your YASB UI components.

## Quick Integration

### In Your Settings Component

```tsx
import { BarSettingsPanel } from '@unkai/sdk'

export function Settings() {
  return (
    <div className="settings">
      <h2>System Tray Settings</h2>
      
      {/* Existing settings */}
      
      {/* Add bar management */}
      <section className="settings-section">
        <h3>Status Bars</h3>
        <BarSettingsPanel />
      </section>
    </div>
  )
}
```

---

## Component Usage Patterns

### Pattern 1: Full Settings (Recommended)

Use `BarSettingsPanel` for complete UI with tabs:

```tsx
import { BarSettingsPanel } from '@unkai/sdk'

export function SettingsUI() {
  return (
    <main className="settings-container">
      <h1>YASB Settings</h1>
      
      <section>
        <h2>Status Bar Management</h2>
        <BarSettingsPanel />
      </section>
    </main>
  )
}
```

**Includes:**
- Bar selector dropdown
- All available bars grid
- Create bar form
- All styling included

---

### Pattern 2: Compact Dropdown (Minimal)

Use `BarSelector` for dropdown only:

```tsx
import { BarSelector } from '@unkai/sdk'

export function MinimalSettings() {
  return (
    <div>
      <label>Status Bar</label>
      <BarSelector />
    </div>
  )
}
```

---

### Pattern 3: In System Tray Menu

Use `QuickBarSelector` for right-click context menu:

```tsx
import { QuickBarSelector } from '@unkai/sdk'

export function TrayMenu() {
  return (
    <div className="tray-menu">
      <button>Settings</button>
      <button>Exit</button>
      <hr />
      
      <strong>Status Bars</strong>
      <QuickBarSelector />
    </div>
  )
}
```

---

### Pattern 4: Custom Implementation

Use hooks for complete control:

```tsx
import { useAvailableBars, useSelectedBar, useBarManager } from '@unkai/sdk'

export function CustomBarManager() {
  const { bars, isLoading, error, refresh } = useAvailableBars()
  const { bar: selected, selectBar } = useSelectedBar()
  const { createBar } = useBarManager()
  
  const [newBarName, setNewBarName] = useState('')

  return (
    <div>
      {/* Show error if any */}
      {error && <div className="error">{error}</div>}

      {/* Current selection */}
      {selected && (
        <div>
          <h3>Active: {selected.name}</h3>
          <p>{selected.description}</p>
        </div>
      )}

      {/* Available bars */}
      <h4>Available Bars</h4>
      {isLoading ? (
        <p>Loading...</p>
      ) : (
        <ul>
          {bars.map(bar => (
            <li key={bar.id}>
              <button onClick={() => selectBar(bar.id)}>
                {bar.name}
                {bar.active && ' ✓'}
              </button>
            </li>
          ))}
        </ul>
      )}

      {/* Refresh */}
      <button onClick={refresh}>Refresh Bars</button>

      {/* Create new */}
      <div>
        <input
          type="text"
          placeholder="New bar name"
          value={newBarName}
          onChange={e => setNewBarName(e.target.value)}
        />
        <button onClick={() => createBar(newBarName)}>
          Create Bar
        </button>
      </div>
    </div>
  )
}
```

---

## State Management Patterns

### Using Context Provider

Wrap your app for access throughout:

```tsx
import { BarManager, useBarManager } from '@unkai/sdk'

// Create context
const BarContext = createContext()

export function App() {
  return (
    <BarProvider>
      <Settings />
    </BarProvider>
  )
}

// In child components:
export function SettingsPage() {
  const bar = useContext(BarContext)
  return <BarSettingsPanel />
}
```

### Subscribing to Changes

React to bar changes automatically:

```tsx
import { useEffect } from 'react'
import { useSelectedBar } from '@unkai/sdk'

export function BarStatusDisplay() {
  const { bar } = useSelectedBar()

  useEffect(() => {
    if (bar) {
      console.log(`Bar updated to: ${bar.name}`)
      // Trigger UI refresh, logging, etc.
    }
  }, [bar?.id]) // Re-run only when ID changes
}
```

---

## Styling Integration

### Default Dark Theme

All components include built-in styling:

```tsx
import { BarSettingsPanel } from '@unkai/sdk'

export function App() {
  return (
    <div className="app" style={{ background: '#1e1e1e', color: '#e0e0e0' }}>
      <BarSettingsPanel /> {/* ← All CSS included */}
    </div>
  )
}
```

### Custom Theme

Override CSS with your own:

```css
/* Override button colors */
.bar-selector button {
  background: #your-color !important;
  color: #your-text !important;
}

/* Override card styling */
.bar-card {
  border-color: #your-accent !important;
}

/* Override tab styling */
.bar-settings-tab.active {
  border-bottom-color: #your-accent !important;
}
```

### Theme Variables

Available CSS variables:

```css
:root {
  --bar-bg-primary: #1e1e1e;
  --bar-bg-secondary: #2a2a2a;
  --bar-bg-hover: #3a3a3a;
  --bar-text-primary: #e0e0e0;
  --bar-text-secondary: #999999;
  --bar-accent: #0099ff;
  --bar-border: #444444;
  --bar-error: #ff4444;
  --bar-success: #44ff44;
}
```

---

## Error Handling

Components include built-in error handling:

```tsx
import { BarSettingsPanel } from '@unkai/sdk'

// Component shows errors automatically
<BarSettingsPanel />

// Or handle manually:
import { useAvailableBars } from '@unkai/sdk'

export function MyComponent() {
  const { error } = useAvailableBars()
  
  return (
    <>
      {error && (
        <div className="alert alert-error">
          Failed to load bars: {error}
        </div>
      )}
    </>
  )
}
```

---

## Loading States

Components handle loading gracefully:

```tsx
import { useAvailableBars } from '@unkai/sdk'

export function MyComponent() {
  const { isLoading } = useAvailableBars()
  
  return (
    <>
      {isLoading ? (
        <div className="spinner" />
      ) : (
        <BarSettingsPanel />
      )}
    </>
  )
}
```

---

## Event Listening

Subscribe to bar events:

```tsx
import { useEffect } from 'react'
import { useBarManager } from '@unkai/sdk'

export function BarEventMonitor() {
  const { getState } = useBarManager()

  useEffect(() => {
    const manager = getState()
    
    // Listen for discovery
    manager.on?.('bars:discovered', (bars) => {
      console.log('Discovered bars:', bars)
    })

    // Listen for selection
    manager.on?.('bar:selected', (bar) => {
      console.log('Selected bar:', bar.name)
    })

    return () => {
      // Cleanup listeners
      manager.on?.('bars:discovered', null)
      manager.on?.('bar:selected', null)
    }
  }, [])
}
```

---

## Real-World Examples

### Example 1: Settings Dashboard

```tsx
import React, { useState } from 'react'
import { BarSettingsPanel, useAvailableBars } from '@unkai/sdk'

export function SettingsDashboard() {
  const { bars } = useAvailableBars()
  const [activeTab, setActiveTab] = useState('bars')

  return (
    <div className="settings-dashboard">
      <header>
        <h1>YASB Settings</h1>
        <p>{bars.length} bars available</p>
      </header>

      <main>
        <BarSettingsPanel />
      </main>

      <footer>
        <button onClick={() => {/* restart */}}>Restart YASB</button>
      </footer>
    </div>
  )
}
```

### Example 2: Tray Integration

```tsx
import { QuickBarSelector } from '@unkai/sdk'

export function TrayContextMenu() {
  return (
    <div className="context-menu">
      <div className="menu-item">Settings</div>
      <div className="menu-item">Documentation</div>
      <div className="menu-divider" />
      
      <div className="menu-section">
        <strong>Active Status Bar</strong>
        <QuickBarSelector />
      </div>

      <div className="menu-divider" />
      <div className="menu-item danger">Exit</div>
    </div>
  )
}
```

### Example 3: Bar Discovery Visualization

```tsx
import { useAvailableBars, useSelectedBar } from '@unkai/sdk'

export function BarVisualization() {
  const { bars } = useAvailableBars()
  const { bar: selected } = useSelectedBar()

  return (
    <div className="bar-grid">
      <h2>Available Status Bars ({bars.length})</h2>
      
      <div className="grid">
        {bars.map(bar => (
          <div
            key={bar.id}
            className={`card ${bar.id === selected?.id ? 'active' : ''}`}
          >
            <h3>{bar.name}</h3>
            <p>{bar.description}</p>
            <small>v{bar.version}</small>
            {bar.active && <span className="badge">Active</span>}
          </div>
        ))}
      </div>
    </div>
  )
}
```

---

## Testing Components Locally

### Standalone Test

```tsx
import { BarSettingsPanel } from '@unkai/sdk'

export default function TestPage() {
  return (
    <div style={{
      padding: '20px',
      background: '#1e1e1e',
      color: '#e0e0e0',
      minHeight: '100vh'
    }}>
      <h1>Bar Settings Test</h1>
      <BarSettingsPanel />
    </div>
  )
}
```

### Mock Data

Test without IPC:

```tsx
import { BarSettingsPanel } from '@unkai/sdk'

// Mock bars
const mockBars = [
  { id: 'bar1', name: 'Top Bar', description: 'System metrics', version: '1.0.0', active: true },
  { id: 'bar2', name: 'Bottom Bar', description: 'Clock and date', version: '1.0.1', active: false }
]

export function TestWithMocks() {
  return <BarSettingsPanel />
}
```

---

## Performance Optimization

### Memoization

```tsx
import React from 'react'
import { BarSelector } from '@unkai/sdk'

const MemoizedSelector = React.memo(BarSelector)

export function App() {
  return <MemoizedSelector />
}
```

### Lazy Loading

```tsx
import { lazy, Suspense } from 'react'

const BarManager = lazy(() => import('@unkai/sdk').then(m => ({ default: m.BarSettingsPanel })))

export function App() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <BarManager />
    </Suspense>
  )
}
```

---

## Troubleshooting Integration

### Components not rendering

1. Check import path:
   ```tsx
   import { BarSettingsPanel } from '@unkai/sdk'  // ✓ correct
   import { BarSettingsPanel } from './components' // ✗ wrong
   ```

2. Verify SDK exported:
   ```bash
   grep "export.*BarSettingsPanel" /path/to/sdk
   ```

3. Check context setup

### IPC commands failing

1. Verify Tauri backend commands registered:
   ```rust
   generate_handler![
     discover_available_bars,
     select_bar,
     get_selected_bar,
     create_bar_scaffold,
     get_bar_manifest
   ]
   ```

2. Check browser console for CORS/security errors

3. Inspect Tauri logs: `%APPDATA%\YASB\logs\`

### Styling not applying

1. Check CSS not being overridden
2. Increase specificity if needed
3. Use `!important` as last resort

---

## Next Steps

1. Choose integration pattern (Pattern 1-4 above)
2. Add BarSettingsPanel or component variant to your UI
3. Test in development with `npm run dev`
4. Build and verify in releases

See [BAR_CREATION_GUIDE.md](BAR_CREATION_GUIDE.md) for user-facing documentation.

