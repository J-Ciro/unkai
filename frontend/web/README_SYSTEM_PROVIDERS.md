# YASB System Providers SDK

Expand your YASB status bar with comprehensive system information providers for audio, battery, memory, and network. Built on the proven Komorebi integration pattern with full TypeScript support.

## Table of Contents

1. [Overview](#overview)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Audio Provider](#audio-provider)
5. [Battery Provider](#battery-provider)
6. [Memory Provider](#memory-provider)
7. [Network Provider](#network-provider)
8. [Context Integration](#context-integration)
9. [Examples](#examples)
10. [API Reference](#api-reference)
11. [Performance Tips](#performance-tips)
12. [Troubleshooting](#troubleshooting)

---

## Overview

System Providers give you access to live system metrics through typed React hooks. Inspired by Zebar's provider architecture but built for framework flexibility.

**Key Features:**
- ✅ Full TypeScript support
- ✅ Real-time polling with configurable intervals
- ✅ React Context + Hooks patterns
- ✅ Event-driven updates
- ✅ Zero-overhead singleton pattern
- ✅ Automatic cleanup on unmount

**Supported Metrics:**
- Audio: devices, volume control, default device
- Battery: charge %, state, health, time estimates
- Memory: usage %, RAM/swap statistics
- Network: traffic speed, total transferred, interfaces

---

## Installation

System Providers are included in the YASB Frontend SDK. Copy the `lib/` folder to your project:

```bash
# In your Next.js or React app
cp -r unkai/frontend/web/lib ./src/lib/yasb
```

---

## Quick Start

### Pattern 1: Direct Hooks

```tsx
import { useMemory, useBattery, useNetwork } from '@/lib/yasb/hooks/useSystemProviders';

export function SystemStatus() {
  const memory = useMemory();
  const battery = useBattery();
  const network = useNetwork();

  return (
    <div>
      <p>RAM: {memory.usagePercent.toFixed(1)}%</p>
      <p>Battery: {battery.status?.chargePercent.toFixed(0)}%</p>
      <p>Network: {((network.traffic?.receivedPerSec || 0) * 8 / 1_000_000).toFixed(2)} Mbps</p>
    </div>
  );
}
```

### Pattern 2: System Context + Hooks

```tsx
import {
  SystemProviderContext,
  useSystemContext,
} from '@/lib/yasb/providers/context';

// Wrap your app
export default function App() {
  return (
    <SystemProviderContext>
      <YourApp />
    </SystemProviderContext>
  );
}

// Use anywhere
function Dashboard() {
  const system = useSystemContext();
  
  return (
    <div>
      <p>Memory: {system.memory.usagePercent.toFixed(1)}%</p>
      <p>Battery: {system.battery.status?.chargePercent}%</p>
      <p>Network: {system.network.traffic?.receivedPerSec} bytes/s</p>
    </div>
  );
}
```

### Pattern 3: Combined System State

```tsx
import { useSystemProviders } from '@/lib/yasb/hooks/useSystemProviders';

export function ComprehensiveDashboard() {
  const {
    audio,
    battery,
    memory,
    network,
    isConnected,
    startPolling,
    stopPolling,
  } = useSystemProviders({
    autoPoll: true,
    memoryInterval: 1000,
    batteryInterval: 5000,
    networkInterval: 2000,
  });

  return (
    <div>
      {isConnected && (
        <>
          <MemoryUsageWidget stats={memory} />
          <BatteryStatusWidget status={battery} />
          <NetworkTrafficWidget traffic={network} />
        </>
      )}
    </div>
  );
}
```

---

## Audio Provider

### Interface

```typescript
interface AudioProviderState {
  playbackDevices: AudioDevice[];
  recordingDevices: AudioDevice[];
  defaultPlaybackDevice: AudioDevice | null;
  defaultRecordingDevice: AudioDevice | null;
  isLoading: boolean;
  error: string | null;
}

interface AudioDevice {
  deviceId: string;
  name: string;
  volume: number; // 0-100
  deviceType: 'playback' | 'recording';
  isDefault: boolean;
}
```

### Hook Usage

```tsx
import { useAudio } from '@/lib/yasb/hooks/useSystemProviders';

export function AudioControl() {
  const audio = useAudio();

  return (
    <div>
      <h3>Speakers</h3>
      {audio.playbackDevices.map(device => (
        <div key={device.deviceId}>
          <span>{device.name}</span>
          <input
            type="range"
            min="0"
            max="100"
            value={device.volume}
            onChange={e => audio.setVolume(device.deviceId, parseInt(e.target.value))}
          />
          <span>{device.volume.toFixed(0)}%</span>
        </div>
      ))}
    </div>
  );
}
```

### Methods

```typescript
// Get all playback/recording devices
await audio.getPlaybackDevices(): AudioDevice[]
await audio.getRecordingDevices(): AudioDevice[]

// Control volume (0-100)
await audio.setVolume(deviceId: string, volume: number): void

// State access
audio.playbackDevices: AudioDevice[]
audio.recordingDevices: AudioDevice[]
audio.defaultPlaybackDevice: AudioDevice | null
audio.defaultRecordingDevice: AudioDevice | null
```

---

## Battery Provider

### Interface

```typescript
interface BatteryProviderState {
  status: BatteryStatus | null;
  isCharging: boolean;
  isAvailable: boolean;
  isLoading: boolean;
  error: string | null;
  lastUpdated: number;
}

interface BatteryStatus {
  chargePercent: number; // 0-100
  state: 'charging' | 'discharging' | 'full' | 'empty' | 'unknown';
  healthPercent?: number; // 0-100
  timeTillEmpty?: number; // milliseconds
  timeTillFull?: number; // milliseconds
  powerConsumption?: number; // watts
}
```

### Hook Usage

```tsx
import { useBattery } from '@/lib/yasb/hooks/useSystemProviders';

export function BatteryIndicator() {
  const battery = useBattery();

  if (!battery.isAvailable) {
    return <p>No battery (AC power)</p>;
  }

  const s = battery.status!;
  const className =
    s.chargePercent < 20 ? 'critical' :
    s.chargePercent < 50 ? 'warning' :
    'normal';

  return (
    <div className={`battery ${className}`}>
      <div className="fill" style={{ width: `${s.chargePercent}%` }} />
      <span>{s.chargePercent.toFixed(0)}%</span>
      <span>{s.state}</span>
    </div>
  );
}
```

### Methods

```typescript
// Manual refresh
await battery.refresh(): void
await battery.getStatus(): void

// State access
battery.status: BatteryStatus | null
battery.isCharging: boolean
battery.isAvailable: boolean
battery.lastUpdated: number
```

---

## Memory Provider

### Interface

```typescript
interface MemoryProviderState {
  stats: MemoryStats | null;
  usagePercent: number; // 0-100
  isLoading: boolean;
  error: string | null;
  lastUpdated: number;
}

interface MemoryStats {
  total: number; // bytes
  used: number; // bytes
  free: number; // bytes
  available: number; // bytes
  buffers?: number;
  cached?: number;
  swapTotal?: number;
  swapUsed?: number;
  swapFree?: number;
}
```

### Hook Usage

```tsx
import { useMemory } from '@/lib/yasb/hooks/useSystemProviders';

export function MemoryUsage() {
  const memory = useMemory();

  if (!memory.stats) {
    return <p>Loading...</p>;
  }

  const usedGb = (memory.stats.used / 1024 / 1024 / 1024).toFixed(2);
  const totalGb = (memory.stats.total / 1024 / 1024 / 1024).toFixed(2);

  return (
    <div>
      <div className="bar">
        <div
          className="fill"
          style={{ width: `${memory.usagePercent}%` }}
        />
      </div>
      <p>
        {usedGb} GB / {totalGb} GB ({memory.usagePercent.toFixed(1)}%)
      </p>
      <p>Free: {(memory.stats.free / 1024 / 1024 / 1024).toFixed(2)} GB</p>
    </div>
  );
}
```

### Methods

```typescript
// Manual refresh
await memory.refresh(): void
await memory.getStats(): MemoryStats | null

// Getters
memory.getUsagePercent(): number

// State access
memory.stats: MemoryStats | null
memory.usagePercent: number
memory.lastUpdated: number
```

---

## Network Provider

### Interface

```typescript
interface NetworkProviderState {
  interfaces: NetworkInterface[];
  defaultInterface: NetworkInterface | null;
  traffic: NetworkTraffic | null;
  isLoading: boolean;
  error: string | null;
  lastUpdated: number;
}

interface NetworkInterface {
  name: string;
  ipAddress?: string;
  macAddress?: string;
  isUp: boolean;
  isDefault: boolean;
}

interface NetworkTraffic {
  receivedPerSec: number; // bytes
  transmittedPerSec: number; // bytes
  totalReceived: number; // bytes
  totalTransmitted: number; // bytes
}
```

### Hook Usage

```tsx
import { useNetwork } from '@/lib/yasb/hooks/useSystemProviders';

export function NetworkSpeed() {
  const network = useNetwork();

  if (!network.traffic) {
    return <p>Loading...</p>;
  }

  const dlMbps = (network.traffic.receivedPerSec * 8) / 1_000_000;
  const ulMbps = (network.traffic.transmittedPerSec * 8) / 1_000_000;

  return (
    <div>
      <p>Interface: {network.defaultInterface?.name}</p>
      <p>⬇️ {dlMbps.toFixed(2)} Mbps</p>
      <p>⬆️ {ulMbps.toFixed(2)} Mbps</p>
      <p>Total ↓: {(network.traffic.totalReceived / 1_000_000_000).toFixed(2)} GB</p>
      <p>Total ↑: {(network.traffic.totalTransmitted / 1_000_000_000).toFixed(2)} GB</p>
    </div>
  );
}
```

### Methods

```typescript
// Fetch data
await network.getTraffic(): NetworkTraffic | null
await network.getInterfaces(): NetworkInterface[]
await network.getDefaultNetworkInterface(): NetworkInterface | null

// Manual refresh
await network.refresh(): void

// State access
network.interfaces: NetworkInterface[]
network.defaultInterface: NetworkInterface | null
network.traffic: NetworkTraffic | null
network.lastUpdated: number
```

---

## Context Integration

### SystemProviderContext

Wrap your app to get automatic polling and event distribution:

```tsx
import { SystemProviderContext } from '@/lib/yasb/providers/context';

export default function App() {
  return (
    <SystemProviderContext>
      <Dashboard />
      <Sidebar />
      <Footer />
    </SystemProviderContext>
  );
}
```

**Polling defaults:**
- Battery: 5000ms
- Memory: 1000ms
- Network: 2000ms

### useSystemProvider Hook

Direct provider access:

```tsx
import { useSystemProvider } from '@/lib/yasb/providers/context';

export function AdvancedControls() {
  const provider = useSystemProvider();

  const handleClick = async () => {
    // Direct provider methods
    await provider.updateMemoryStats();
    const battery = await provider.getBatteryStatus();
    await provider.setVolume('device-id', 75);

    // Manual polling control
    provider.startPolling({ memoryInterval: 500 });
    provider.stopPolling();

    // Event handling
    const unsub = provider.on('memory:updated', (stats) => {
      console.log('Memory updated:', stats);
    });
  };

  return <button onClick={handleClick}>Update System Info</button>;
}
```

---

## Examples

### 1. Minimal Status Bar

```tsx
import { useBattery, useMemory } from '@/lib/yasb/hooks/useSystemProviders';

export function StatusBar() {
  const battery = useBattery();
  const memory = useMemory();

  return (
    <div style={{ display: 'flex', gap: '16px', padding: '8px' }}>
      <span>RAM {memory.usagePercent?.toFixed(0)}%</span>
      <span>🔋 {battery.status?.chargePercent?.toFixed(0)}%</span>
    </div>
  );
}
```

### 2. System Metrics Widget

```tsx
import { SystemStatusDashboard } from '@/lib/yasb/examples/SystemProviderExamples';

export default function Dashboard() {
  return (
    <div className="dashboard">
      <h1>System Metrics</h1>
      <SystemStatusDashboard />
    </div>
  );
}
```

### 3. Tray Widget

```tsx
import { SystemTrayWidget } from '@/lib/yasb/examples/SystemProviderExamples';

export default function Taskbar() {
  return (
    <footer>
      <SystemTrayWidget />
    </footer>
  );
}
```

### 4. Custom Alert Widget

```tsx
import { useBattery, useMemory } from '@/lib/yasb/hooks/useSystemProviders';

export function Alerts() {
  const battery = useBattery();
  const memory = useMemory();

  const memoryAlert = memory.usagePercent > 85;
  const batteryAlert = battery.status?.chargePercent && battery.status.chargePercent < 15;

  return (
    <div>
      {memoryAlert && <div className="alert warning">High memory usage!</div>}
      {batteryAlert && <div className="alert critical">Low battery!</div>}
    </div>
  );
}
```

---

## API Reference

### System Providers Hooks

#### `useAudio()`

```typescript
const audio = useAudio()

// Properties
audio.playbackDevices: AudioDevice[]
audio.recordingDevices: AudioDevice[]
audio.defaultPlaybackDevice: AudioDevice | null
audio.defaultRecordingDevice: AudioDevice | null
audio.isLoading: boolean
audio.error: string | null

// Methods
await audio.setVolume(deviceId: string, volume: number): void
await audio.getPlaybackDevices(): AudioDevice[]
await audio.getRecordingDevices(): AudioDevice[]
```

#### `useBattery()`

```typescript
const battery = useBattery()

// Properties
battery.status: BatteryStatus | null
battery.isCharging: boolean
battery.isAvailable: boolean
battery.isLoading: boolean
battery.error: string | null
battery.lastUpdated: number

// Methods
await battery.refresh(): void
```

#### `useMemory()`

```typescript
const memory = useMemory()

// Properties
memory.stats: MemoryStats | null
memory.usagePercent: number
memory.isLoading: boolean
memory.error: string | null
memory.lastUpdated: number

// Methods
await memory.refresh(): void
memory.getUsagePercent(): number
```

#### `useNetwork()`

```typescript
const network = useNetwork()

// Properties
network.interfaces: NetworkInterface[]
network.defaultInterface: NetworkInterface | null
network.traffic: NetworkTraffic | null
network.isLoading: boolean
network.error: string | null
network.lastUpdated: number

// Methods
await network.refresh(): void
await network.getTraffic(): NetworkTraffic | null
```

#### `useSystemProviders(options?)`

```typescript
const system = useSystemProviders({
  autoPoll?: boolean; // default: true
  batteryInterval?: number; // default: 5000ms
  memoryInterval?: number; // default: 1000ms
  networkInterval?: number; // default: 2000ms
})

// All individual provider states
system.audio: AudioProviderState
system.battery: BatteryProviderState
system.memory: MemoryProviderState
system.network: NetworkProviderState

// Overall state
system.isConnected: boolean
system.isLoading: boolean

// Controls
system.startPolling(): void
system.stopPolling(): void
```

---

## Performance Tips

1. **Adjust polling intervals** - Reduce memory polling to 2000ms if CPU is high:
   ```tsx
   useSystemProviders({ memoryInterval: 2000 })
   ```

2. **Memoize components** - Prevent unnecessary re-renders:
   ```tsx
   const StatusBar = React.memo(() => {
     const memory = useMemory();
     return <span>{memory.usagePercent.toFixed(1)}%</span>;
   });
   ```

3. **Selective subscriptions** - Only attach to events you need:
   ```tsx
   const memory = useMemory(); // Memory polling only
   ```

4. **Lazy load widgets** - Use React.lazy for off-screen widgets:
   ```tsx
   const NetworkWidget = React.lazy(() =>
     import('./NetworkWidget')
   );
   ```

5. **Debounce rapid updates** - For UI-intensive dashboards:
   ```tsx
   const debouncedStats = useMemo(
     () => debounce(() => memory.refresh(), 250),
     []
   );
   ```

---

## Troubleshooting

### Providers not updating

**Issue**: `useMemory()` state stays null

**Solutions**:
- Ensure wrapped in `SystemProviderContext`
- Check browser console for errors
- Verify Tauri backend commands exist
- Manually trigger: `memory.refresh()`

### High CPU usage

**Issue**: Memory spikes from constant polling

**Solutions**:
- Increase polling intervals
- Stop polling when not visible: `network.stopPolling()`
- Use `autoPoll: false` and manual refresh
- Profile with React DevTools Profiler

### Audio volume not changing

**Issue**: `setVolume()` doesn't affect system volume

**Solutions**:
- Verify device ID exists: `audio.getPlaybackDevices()`
- Check volume range (0-100)
- Ensure backend audio permission
- Try different device ID

### Battery always shows null

**Issue**: `battery.status` is always null

**Solutions**:
- Device may not have battery (desktop)
- Check `battery.isAvailable`
- Verify Windows has battery API access
- Linux may need udev rules

---

## Contributing

Found a bug? Have a feature request?

1. Check existing issues in /unkai
2. File issue with error logs
3. Reference hook/provider name
4. Include minimal reproducible example

---

## Next Steps

- Combine with Komorebi provider for complete system bar
- Add CPU provider for process-level metrics
- Create custom provider plugins
- Build marketplace of community widgets

