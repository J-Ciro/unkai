# System Providers Quick Reference

## Installation

```bash
# Copy SDK to your project
cp -r unkai/frontend/web/lib ./src/lib/yasb
```

## Import Everything

```tsx
// Barrel import - get everything
import {
  // Types
  MemoryStats,
  BatteryStatus,
  AudioDevice,
  NetworkTraffic,
  
  // Hooks
  useMemory,
  useBattery,
  useAudio,
  useNetwork,
  useSystemProviders,
  
  // Context
  SystemProviderContext,
  useSystemContext,
  useSystemProvider,
  
  // Examples
  MemoryUsageWidget,
  BatteryStatusWidget,
  SystemStatusDashboard,
} from '@/lib/yasb';
```

## 30-Second Setup

### 1. Wrap Your App
```tsx
import { SystemProviderContext } from '@/lib/yasb';

export default function App() {
  return (
    <SystemProviderContext>
      <YourApp />
    </SystemProviderContext>
  );
}
```

### 2. Use Anywhere
```tsx
import { useMemory, useBattery } from '@/lib/yasb';

function StatusBar() {
  const memory = useMemory();
  const battery = useBattery();

  return (
    <div>
      RAM: {memory.usagePercent.toFixed(0)}% | 
      Battery: {battery.status?.chargePercent.toFixed(0)}%
    </div>
  );
}
```

## Common Use Cases

### Memory Monitor
```tsx
const { stats, usagePercent } = useMemory();
// stats.used, stats.total, usagePercent
```

### Battery Alert
```tsx
const battery = useBattery();

if (battery.status?.chargePercent < 20) {
  return <Alert>Low battery!</Alert>;
}
```

### Network Speed
```tsx
const { traffic } = useNetwork();
const dlMbps = (traffic.receivedPerSec * 8) / 1_000_000;
// Display dlMbps
```

### Audio Volume Control
```tsx
const audio = useAudio();

audio.playbackDevices.forEach(device => {
  <input
    type="range"
    value={device.volume}
    onChange={e => audio.setVolume(device.deviceId, e.target.value)}
  />
});
```

## Polling Control

```tsx
// Auto-poll (enabled by default)
const system = useSystemProviders({ autoPoll: true });

// Adjust intervals
const system = useSystemProviders({
  memoryInterval: 500,    // 500ms - very frequent
  batteryInterval: 10000, // 10s - less frequent
  networkInterval: 5000,  // 5s - moderate
});

// Manual control
system.startPolling();
system.stopPolling();
```

## Event Listening

```tsx
import { useSystemProvider, useSystemProviderEvents } from '@/lib/yasb';

const provider = useSystemProvider();

// Subscribe to specific events
provider.on('memory:updated', (stats) => {
  console.log('Memory changed:', stats);
});

provider.on('battery:status-changed', (status) => {
  console.log('Battery changed:', status);
});

// Or use hook
const { on } = useSystemProviderEvents();
on('network:traffic-updated', (traffic) => {
  console.log('Network changed:', traffic);
});
```

## TypeScript Types

```typescript
// Memory
MemoryStats {
  total: number;
  used: number;
  free: number;
  available: number;
  buffers?: number;
  cached?: number;
  swapTotal?: number;
  swapUsed?: number;
  swapFree?: number;
}

// Battery
BatteryStatus {
  chargePercent: number;
  state: 'charging' | 'discharging' | 'full' | 'empty' | 'unknown';
  healthPercent?: number;
  timeTillEmpty?: number;
  timeTillFull?: number;
  powerConsumption?: number;
}

// Audio
AudioDevice {
  deviceId: string;
  name: string;
  volume: number;
  deviceType: 'playback' | 'recording';
  isDefault: boolean;
}

// Network
NetworkTraffic {
  receivedPerSec: number;
  transmittedPerSec: number;
  totalReceived: number;
  totalTransmitted: number;
}

NetworkInterface {
  name: string;
  ipAddress?: string;
  macAddress?: string;
  isUp: boolean;
  isDefault: boolean;
}
```

## Complete Example

```tsx
import React from 'react';
import {
  SystemProviderContext,
  useMemory,
  useBattery,
  useNetwork,
  BatteryStatusWidget,
  MemoryUsageWidget,
  NetworkTrafficWidget,
  SystemTrayWidget,
} from '@/lib/yasb';

export default function App() {
  return (
    <SystemProviderContext>
      <StatusBar />
    </SystemProviderContext>
  );
}

function StatusBar() {
  const memory = useMemory();
  const battery = useBattery();
  const network = useNetwork();

  return (
    <div className="status-bar">
      <div className="section">
        {memory.stats && (
          <span>
            💾 {memory.usagePercent.toFixed(0)}% 
            ({(memory.stats.used / 1024 / 1024 / 1024).toFixed(1)} GB)
          </span>
        )}
      </div>

      <div className="section">
        {battery.status && (
          <span>
            🔋 {battery.status.chargePercent.toFixed(0)}% 
            ({battery.status.state})
          </span>
        )}
      </div>

      <div className="section">
        {network.traffic && (
          <span>
            🌐 {((network.traffic.receivedPerSec * 8) / 1_000_000).toFixed(2)} Mbps
          </span>
        )}
      </div>
    </div>
  );
}

// CSS
const styles = `
  .status-bar {
    display: flex;
    justify-content: space-between;
    padding: 12px 16px;
    background: #1e1e1e;
    color: #e0e0e0;
    font-family: monospace;
    gap: 24px;
  }

  .section {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .section span {
    white-space: nowrap;
  }
`;
```

## Combining with Komorebi

```tsx
import {
  KomorebiContextProvider,
  SystemProviderContext,
  useKomorebi,
  useMemory,
  useBattery,
} from '@/lib/yasb';

export default function App() {
  return (
    <KomorebiContextProvider>
      <SystemProviderContext>
        <YourApp />
      </SystemProviderContext>
    </KomorebiContextProvider>
  );
}

// Now both are available everywhere
function Widgets() {
  const workspace = useKomorebi();
  const memory = useMemory();
  const battery = useBattery();

  return (
    <div>
      Workspace: {workspace.workspaceId}
      Memory: {memory.usagePercent}%
      Battery: {battery.status?.chargePercent}%
    </div>
  );
}
```

## Hooks Cheat Sheet

| Hook | Returns | Purpose |
|------|---------|---------|
| `useMemory()` | MemoryProviderState + methods | RAM/swap metrics |
| `useBattery()` | BatteryProviderState + methods | Battery status |
| `useAudio()` | AudioProviderState + methods | Audio devices |
| `useNetwork()` | NetworkProviderState + methods | Network traffic |
| `useSystemProviders()` | SystemProviderState + controls | All providers |
| `useSystemContext()` | SystemProviderState | Access app context |
| `useSystemProvider()` | SystemProvider | Direct provider |

## Troubleshooting

**Q: State is null**
- Ensure wrapped in `SystemProviderContext`
- Check browser console for Tauri errors
- Verify backend IPC commands are registered

**Q: High CPU usage**
- Reduce polling intervals
- Stop polling when off-screen
- Use manual refresh instead of auto-polling

**Q: Battery always null**
- Desktop/AC power - battery feature not available
- Check `battery.isAvailable` before use

**Q: Audio commands fail**
- Verify device ID from `getPlaybackDevices()`
- Check volume is 0-100
- Platform permissions may be needed

---

For complete API reference, see [README_SYSTEM_PROVIDERS.md](README_SYSTEM_PROVIDERS.md)

To combine with Komorebi, see [README_KOMOREBI_SDK.md](README_KOMOREBI_SDK.md)
