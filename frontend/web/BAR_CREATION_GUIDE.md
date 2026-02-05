# Creating Custom Status Bars for YASB

YASB allows you to create completely custom status bars in React. Each bar is a standalone web app that gets embedded in a Tauri window.

## Directory Structure

Custom bars are stored in `~/.unkai/bars/` on Windows:

```
C:\Users\[username]\.unkai\
├── bars/
│   ├── my-bar/
│   │   ├── barpack.json        # Bar configuration
│   │   ├── package.json        # npm dependencies
│   │   ├── vite.config.ts      # Build configuration
│   │   ├── index.html          # HTML template
│   │   ├── src/
│   │   │   ├── main.tsx        # React entrypoint
│   │   │   ├── App.tsx         # Main component
│   │   │   └── index.css       # Styles
│   │   └── dist/               # Compiled output (created after build)
│   └── another-bar/
└── config.json
```

## Quick Start

### 1. Create Bar Scaffold

Via YASB settings (right-click system tray → Settings → "Create Bar"):

```
Name: My Cool Bar
```

Or manually create directory and files.

### 2. Install Dependencies

```bash
cd ~/.unkai/bars/my-cool-bar
npm install
```

### 3. Develop

```bash
npm run dev
```

Vite will start dev server at `http://localhost:5173`

### 4. Build

```bash
npm run build
```

Creates `dist/index.html` with compiled assets

### 5. Activate

In YASB settings, select your bar from the list. It will appear immediately.

## barpack.json Reference

```json
{
  "name": "My Status Bar",
  "version": "1.0.0",
  "description": "Beautiful system status bar",
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
    "theme": "dark"
  }
}
```

### Window Configuration

- **width/height**: Window dimensions in pixels
- **offsetX/offsetY**: Offset from anchor point
- **anchor**: Where to position on screen
  - `top_left`, `top_center`, `top_right`
  - `bottom_left`, `bottom_center`, `bottom_right`
- **alwaysOnTop**: Keep window above others
- **transparent**: Enable window transparency
- **resizable**: Allow user to resize
- **shownInTaskbar**: Show in Windows taskbar

## React Component Example

```tsx
import { useMemory, useBattery, useNetwork } from '@unkai/sdk'
import './App.css'

export default function App() {
  const { usagePercent, stats } = useMemory()
  const { status } = useBattery()
  const { traffic } = useNetwork()

  return (
    <div className="status-bar">
      <div className="section">
        <span>💾</span>
        <span>{usagePercent?.toFixed(0)}%</span>
        <div className="bar">
          <div className="fill" style={{ width: `${usagePercent}%` }} />
        </div>
      </div>

      <div className="section">
        <span>🔋</span>
        <span>{status?.chargePercent?.toFixed(0)}%</span>
      </div>

      <div className="section">
        <span>🌐</span>
        <span>{((traffic?.receivedPerSec || 0) * 8 / 1_000_000).toFixed(1)} Mbps</span>
      </div>
    </div>
  )
}
```

## Using YASB SDK

Your bar has access to all YASB providers:

```tsx
// System metrics
import { useMemory, useBattery, useAudio, useNetwork } from '@unkai/sdk'

// Window management (Komorebi)
import { useKomorebi, useWidgets } from '@unkai/sdk'

// Full combined state
import { useSystemProviders } from '@unkai/sdk'

// Direct provider access
import { getSystemProvider, getBarManager } from '@unkai/sdk'
```

## Styling Tips

### Responsive Layout

```css
.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  padding: 0 16px;
  gap: 24px;
}

.section {
  display: flex;
  align-items: center;
  gap: 8px;
}
```

### Dark Mode

```css
body {
  background: rgba(30, 30, 30, 0.95);
  color: #e0e0e0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  backdrop-filter: blur(10px);
}
```

### Visual Bars

```tsx
<div className="progress-bar">
  <div className="fill" style={{ width: `${percent}%` }}>
    <style>{`
      .progress-bar {
        width: 100px;
        height: 6px;
        background: #2a2a2a;
        border-radius: 3px;
        overflow: hidden;
      }
      .fill {
        height: 100%;
        background: linear-gradient(90deg, #ff0000, #00ff00);
        transition: width 0.3s ease;
      }
    `}</style>
  </div>
</div>
```

## Available YASB SDK Features

### Memory Metrics

```tsx
const { stats, usagePercent } = useMemory()

console.log(stats?.used)      // bytes
console.log(stats?.total)     // bytes
console.log(usagePercent)     // 0-100
```

### Battery

```tsx
const battery = useBattery()

console.log(battery.status?.chargePercent)    // 0-100
console.log(battery.status?.state)             // 'charging' | 'discharging' | etc
console.log(battery.isCharging)                // boolean
```

### Network

```tsx
const { traffic } = useNetwork()

const mbps = (traffic?.receivedPerSec || 0) * 8 / 1_000_000
console.log(traffic?.totalReceived)   // total bytes downloaded
```

### Audio

```tsx
const audio = useAudio()

audio.playbackDevices.map(device => (
  <button onClick={() => audio.setVolume(device.deviceId, 50)}>
    {device.name}: {device.volume}%
  </button>
))
```

### Window Manager (Komorebi)

```tsx
const workspace = useKomorebi()

console.log(workspace.workspaceId)     // Current workspace
console.log(workspace.layout)          // 'bsp' | 'columns' | 'rows' | etc
console.log(workspace.windowCount)     // Windows in workspace
```

## Building for Production

1. Make sure bar is built: `npm run build`
2. Test locally in YASB
3. Package bar directory (zip file optional)
4. Share with others

## Example Bars

### Minimal Top Bar

```tsx
export default function App() {
  const { usagePercent } = useMemory()
  const { status } = useBattery()

  return (
    <div style={{
      display: 'flex',
      justifyContent: 'space-between',
      padding: '8px 16px',
      background: '#1e1e1e',
      height: '100%'
    }}>
      <span>RAM {usagePercent?.toFixed(0)}%</span>
      <span>🔋 {status?.chargePercent?.toFixed(0)}%</span>
      <span>{new Date().toLocaleTimeString()}</span>
    </div>
  )
}
```

### Workspace Bar

```tsx
export default function App() {
  const workspace = useKomorebi()
  const { usagePercent } = useMemory()

  return (
    <div style={{ padding: '8px 16px' }}>
      <span>WS {workspace.workspaceId}</span>
      <span>{workspace.layout}</span>
      <span>Windows: {workspace.windowCount}</span>
      <span>RAM: {usagePercent?.toFixed(0)}%</span>
    </div>
  )
}
```

### Full-Featured Dashboard

See `examples/BarExamples.tsx` for complete examples with:
- System metrics display
- Status indicators
- Graph visualizations
- Color themes
- Animations

## Troubleshooting

### Bar not showing up

1. Verify `barpack.json` exists in bar directory
2. Check `entry` path in manifest matches compiled output
3. Ensure `npm run build` was successful
4. Check YASB logs for errors

### Bar seems frozen

1. Check browser console for errors
2. Verify SDK imports are correct
3. Make sure bar is getting events from providers

### High CPU usage

1. Reduce update frequency with polling intervals:
   ```tsx
   useSystemProviders({ 
     memoryInterval: 2000,  // Less frequent
     networkInterval: 5000
   })
   ```
2. Use `React.memo` to prevent unnecessary re-renders
3. Only subscribe to needed events

---

For complete SDK documentation, see:
- [System Providers API](SYSTEM_PROVIDERS_QUICK_REF.md)
- [Komorebi Integration](README_KOMOREBI_SDK.md)
- [Frontend SDK Index](lib/index.ts)

