# YASB Bar Management System

Create, discover, and manage custom React status bars - inspired by Zebar but simpler and more flexible.

## Overview

YASB's bar system allows you to:

- **Create custom React status bars** with full access to system metrics
- **Store bars in `~/.unkai/bars/`** directory for easy discovery
- **Select active bar** through YASB settings (right-click tray)
- **Use simple `barpack.json`** manifest (inspired by Zebar's zpack.json but simpler)
- **Build and test** with standard npm/Vite workflow

---

## Quick Start

### 1. Create a New Bar

**Option A: Via YASB UI**
1. Right-click YASB system tray icon → Settings
2. Go to "Create Bar" tab
3. Enter bar name (e.g., "My Status Bar")
4. Click Create

Bar created at: `~/.unkai/bars/my-status-bar/`

**Option B: Manual**
```bash
mkdir -p ~/.unkai/bars/my-bar
cd ~/.unkai/bars/my-bar

# Copy from scaffold template or create manually
```

### 2. Install Dependencies

```bash
cd ~/.unkai/bars/my-bar
npm install
```

### 3. Develop

```bash
npm run dev
```

Open `http://localhost:5173` in your browser

### 4. Build

```bash
npm run build
```

Creates `dist/index.html` with bundled assets

### 5. Activate

In YASB Settings → Select Bar → Choose your bar

---

## Bar Structure

```
~/.unkai/bars/my-bar/
├── barpack.json           # Configuration manifest
├── package.json           # npm dependencies
├── vite.config.ts         # Build configuration
├── index.html             # HTML template
├── src/
│   ├── main.tsx          # React entry point
│   ├── App.tsx           # Main component
│   └── index.css         # Styles
└── dist/                 # Compiled output (after npm run build)
```

---

## barpack.json Reference

Minimal example:
```json
{
  "name": "My Status Bar",
  "version": "1.0.0",
  "description": "Custom system status bar",
  "entry": "dist/index.html",
  "window": {
    "width": 1920,
    "height": 40,
    "anchor": "top_center",
    "alwaysOnTop": true,
    "transparent": true
  }
}
```

### Complete Configuration

```json
{
  "name": "Awesome Bar",
  "version": "0.1.0",
  "description": "Full-featured status bar with metrics",
  "entry": "dist/index.html",
  
  "window": {
    "width": 1920,              // pixels
    "height": 40,               // pixels
    "offsetX": 0,              // pixels from anchor
    "offsetY": 0,              // pixels from anchor
    "anchor": "top_center",    // positioning
    "alwaysOnTop": true,       // always on top of other windows
    "transparent": true,       // transparent background
    "resizable": false,        // can user resize?
    "shownInTaskbar": false    // show in Windows taskbar?
  },
  
  "metadata": {                // optional custom data
    "author": "Your Name",
    "theme": "dark"
  }
}
```

### Anchor Positions

- `top_left` - Top left corner
- `top_center` - Top center (recommended for bars)
- `top_right` - Top right corner
- `bottom_left` - Bottom left corner
- `bottom_center` - Bottom center
- `bottom_right` - Bottom right corner

---

## Component Examples

### Simple Status Bar

```tsx
import { useMemory, useBattery } from '@unkai/sdk'

export default function App() {
  const { usagePercent } = useMemory()
  const { status } = useBattery()

  return (
    <div style={{
      display: 'flex',
      justifyContent: 'space-between',
      alignItems: 'center',
      height: '100%',
      padding: '0 16px',
      background: '#1e1e1e',
      color: '#e0e0e0'
    }}>
      <span>💾 RAM: {usagePercent?.toFixed(0)}%</span>
      <span>🔋 Battery: {status?.chargePercent?.toFixed(0)}%</span>
      <span>⏰ {new Date().toLocaleTimeString()}</span>
    </div>
  )
}
```

### System Dashboard

```tsx
import React from 'react'
import { useMemory, useBattery, useNetwork, useAudio } from '@unkai/sdk'

export default function App() {
  const memory = useMemory()
  const battery = useBattery()
  const network = useNetwork()
  const audio = useAudio()

  return (
    <div className="bar">
      <div className="section">
        <strong>Memory</strong>
        <span>{memory.usagePercent?.toFixed(0)}%</span>
      </div>

      <div className="section">
        <strong>Battery</strong>
        <span>{battery.status?.chargePercent?.toFixed(0)}%</span>
      </div>

      <div className="section">
        <strong>Network</strong>
        <span>
          {((network.traffic?.receivedPerSec || 0) * 8 / 1_000_000).toFixed(1)} Mbps
        </span>
      </div>
    </div>
  )
}

const styles = `
  .bar {
    display: flex;
    justify-content: space-around;
    align-items: center;
    height: 100%;
    padding: 0 32px;
    background: rgba(30, 30, 30, 0.95);
    backdrop-filter: blur(10px);
    gap: 48px;
  }

  .section {
    display: flex;
    flex-direction: column;
    align-items: center;
    color: #e0e0e0;
  }

  .section strong {
    font-size: 12px;
    color: #0099ff;
    margin-bottom: 4px;
  }

  .section span {
    font-size: 14px;
    font-family: monospace;
  }
`
```

### With Komorebi Integration

```tsx
import { useKomorebi, useMemory } from '@unkai/sdk'

export default function App() {
  const workspace = useKomorebi()
  const memory = useMemory()

  return (
    <div className="bar">
      <div>
        <strong>Workspace:</strong> {workspace.workspaceId}
      </div>
      <div>
        <strong>Layout:</strong> {workspace.layout}
      </div>
      <div>
        <strong>Windows:</strong> {workspace.windowCount}
      </div>
      <div>
        <strong>RAM:</strong> {memory.usagePercent?.toFixed(0)}%
      </div>
    </div>
  )
}
```

---

## Available SDK Providers

### Memory

```tsx
const { stats, usagePercent, refresh } = useMemory()

// stats object:
stats?.total      // total RAM in bytes
stats?.used       // used RAM in bytes
stats?.free       // free RAM in bytes
usagePercent      // 0-100 usage percentage
```

### Battery

```tsx
const { status, isCharging, isAvailable } = useBattery()

// status object:
status?.chargePercent     // 0-100
status?.state             // 'charging' | 'discharging' | 'full' | 'empty'
status?.healthPercent     // battery health 0-100
status?.timeTillEmpty     // milliseconds
status?.timeTillFull      // milliseconds
isCharging                // boolean
```

### Network

```tsx
const { traffic, interfaces, defaultInterface } = useNetwork()

// traffic object:
traffic?.receivedPerSec   // bytes/second
traffic?.transmittedPerSec // bytes/second
traffic?.totalReceived    // total bytes downloaded
traffic?.totalTransmitted // total bytes uploaded

// Convert to Mbps:
const mbps = (traffic?.receivedPerSec || 0) * 8 / 1_000_000
```

### Audio

```tsx
const { playbackDevices, setVolume, defaultPlaybackDevice } = useAudio()

// Set volume (0-100):
await setVolume('device-id', 50)

// playbackDevices:
playbackDevices.map(d => ({
  deviceId: string,
  name: string,
  volume: number,  // 0-100
  isDefault: boolean
}))
```

### Window Manager (Komorebi)

```tsx
const { workspaceId, layout, windowCount, isFocused } = useKomorebi()

// layout is one of:
// 'bsp' | 'columns' | 'rows' | 'grid' | 'maximized'
```

### All Providers Combined

```tsx
const system = useSystemProviders({
  autoPoll: true,
  memoryInterval: 1000,
  batteryInterval: 5000,
  networkInterval: 2000
})

// Access all states:
system.memory.usagePercent
system.battery.status?.chargePercent
system.network.traffic?.receivedPerSec
system.audio.playbackDevices
```

---

## Styling Best Practices

### Full-Height Layout

```css
html, body, #root {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}

.bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  width: 100%;
}
```

### Dark Theme

```css
body {
  background: rgba(30, 30, 30, 0.95);
  color: #e0e0e0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, monospace;
  backdrop-filter: blur(10px);
}
```

### Progress Bars

```tsx
<div style={{
  width: '100px',
  height: '6px',
  background: '#2a2a2a',
  borderRadius: '3px',
  overflow: 'hidden'
}}>
  <div style={{
    width: `${percent}%`,
    height: '100%',
    background: 'linear-gradient(90deg, #ff0000, #00ff00)',
    transition: 'width 0.3s ease'
  }} />
</div>
```

---

## File Structure Best Practices

```
my-bar/
├── src/
│   ├── main.tsx           # ← Keep minimal
│   ├── App.tsx            # ← Main logic here
│   ├── components/        # ← Reusable components
│   │   ├── MemoryWidget.tsx
│   │   ├── BatteryWidget.tsx
│   │   └── Clock.tsx
│   ├── hooks/             # ← Custom hooks
│   │   └── useUpdateInterval.ts
│   ├── utils/             # ← Utilities
│   │   └── format.ts
│   └── index.css          # ← Styles
├── public/                # ← Assets
├── barpack.json
├── package.json
├── tsconfig.json
├── vite.config.ts
└── index.html
```

---

## Building & Deployment

### Production Build

```bash
npm run build
```

Outputs:
- `dist/index.html` - Main HTML file
- `dist/assets/` - JavaScript, CSS, and other assets

### Testing Build Locally

```bash
# Terminal 1: Development mode
npm run dev

# Terminal 2: Preview production build
npm run preview
```

Visit `http://localhost:4173` to test production build

### Sharing Your Bar

1. Build the bar: `npm run build`
2. Zip the entire `~/.unkai/bars/your-bar/` directory
3. Share with others
4. Recipients extract to their `~/.unkai/bars/` directory
5. Bar appears automatically in YASB settings

---

## Troubleshooting

### Bar not appearing in YASB

1. **Verify barpack.json exists**
   ```bash
   ls ~/.unkai/bars/your-bar/barpack.json
   ```

2. **Check entry path**
   - Verify `entry` in barpack.json matches real file
   - Usually: `"entry": "dist/index.html"`

3. **Build the bar**
   ```bash
   cd ~/.unkai/bars/your-bar
   npm run build
   ```

4. **Refresh YASB**
   - Right-click YASB tray icon → Settings → Refresh

### Bar shows but updates seem frozen

1. Check browser console for errors:
   - Press F12 in bar window
   - Look for red errors

2. Verify providers are updating:
   ```tsx
   const { stats } = useMemory()
   useEffect(() => {
     console.log('Memory updated:', stats)
   }, [stats])
   ```

3. Check polling intervals not conflicting

### High CPU usage

1. **Reduce update frequency**
   ```tsx
   useSystemProviders({ 
     memoryInterval: 2000,   // ← increase
     networkInterval: 5000   // ← increase
   })
   ```

2. **Memoize components**
   ```tsx
   const MemoryWidget = React.memo(() => {
     const { usagePercent } = useMemory()
     return <span>{usagePercent}%</span>
   })
   ```

3. **Use conditional rendering**
   ```tsx
   {memory.isLoading && <span>Loading...</span>}
   {!memory.isLoading && <span>{memory.usagePercent}%</span>}
   ```

---

## Architecture Comparison: YASB vs Zebar

| Feature | Zebar | YASB |
|---------|-------|------|
| **Widget Format** | Widget packs + HTML | React + TypeScript |
| **Discovery** | GUI marketplace | Filesystem + UI |
| **Development** | HTML/CSS/JS | npm + Vite + React |
| **Extensibility** | Limited to Zebar | Any React library |
| **Type Safety** | No | Full TypeScript |
| **State Management** | Custom reactivity | React Hooks |
| **Community** | Marketplace | Any React pattern |

**Key Advantage:** YASB bars are just React apps - use any npm package, styling system, or library you want!

---

## Next Steps

1. **[Bar Creation Guide](BAR_CREATION_GUIDE.md)** - Detailed examples
2. **[System Providers Reference](SYSTEM_PROVIDERS_QUICK_REF.md)** - All available metrics
3. **[Komorebi Integration](README_KOMOREBI_SDK.md)** - Window manager features
4. **[Full SDK Docs](lib/index.ts)** - Complete TypeScript API

---

## Example Bars to Try

Check `examples/` directory for:
- `BarManager.tsx` - Bar management UI components
- `SystemProviderExamples.tsx` - System metric widgets
- `KomorebiExamples.tsx` - Window manager examples

Create something awesome! 🚀

