# YASB Komorebi Frontend SDK

Frontend integration for YASB with Komorebi workspace manager support.

## Overview

The YASB Frontend SDK provides TypeScript/React tools for building custom status bars that integrate with:
- **Komorebi** - Tiling window manager state (workspaces, layouts, window counts)
- **YASB Backend** - Real-time widget data via Tauri IPC
- **Zebar Compatible** - Export widgets in Zebar-compatible format

## Installation

1. Copy the `lib/` folder to your React/Next.js project
2. Install peer dependencies:

```bash
npm install react react-dom
```

## Quick Start

### Option 1: Using Context Provider (Recommended)

Wrap your app with the `KomorebiContextProvider`:

```tsx
// app.tsx or _app.tsx
import { KomorebiContextProvider } from '@/lib/providers/context';
import { MyStatusBar } from './components/StatusBar';

export default function App() {
  return (
    <KomorebiContextProvider>
      <MyStatusBar />
    </KomorebiContextProvider>
  );
}

// components/StatusBar.tsx
import { useKomorebiContext, useWidgetsContext } from '@/lib/providers/context';

export function MyStatusBar() {
  const { workspace } = useKomorebiContext();
  const { widgets } = useWidgetsContext();

  return (
    <div>
      <p>Workspace {workspace?.workspace_id}</p>
      <p>Widgets: {widgets.length}</p>
    </div>
  );
}
```

### Option 2: Using Hooks Directly

```tsx
import { useKomorebi, useWidgets } from '@/lib/hooks/useKomorebi';

export function StatusBar() {
  const { workspace, isConnected } = useKomorebi();
  const { widgets } = useWidgets();

  if (!isConnected) return <div>Connecting...</div>;

  return (
    <div>
      Workspace {workspace?.workspace_id} • {widgets.length} widgets
    </div>
  );
}
```

## API Reference

### Contexts

#### `KomorebiContextProvider`

Wrap your app to provide Komorebi state to all components.

```tsx
<KomorebiContextProvider>
  <YourApp />
</KomorebiContextProvider>
```

### Hooks

#### `useKomorebi()`

Access workspace state.

```tsx
const { workspace, isConnected, workspaceId, layout, windowCount } = useKomorebi();
```

**Returns:**
- `workspace: WorkspaceState | null` - Current workspace state
- `isConnected: boolean` - Backend connection status
- `isLoading: boolean` - Initial load state
- `workspaceId: number` - Workspace ID
- `layout: string` - Layout mode (bsp, columns, rows, grid, maximized)
- `windowCount: number` - Number of windows in workspace
- `isFocused: boolean` - Is workspace focused

#### `useWidgets()`

Get all widgets.

```tsx
const { widgets, isLoading, getWidget } = useWidgets();

const cpu = getWidget('cpu-widget');
```

**Returns:**
- `widgets: WidgetRuntime[]` - All widgets
- `isLoading: boolean` - Loading state
- `error: string | null` - Error message
- `getWidget(id): WidgetRuntime | null` - Get specific widget

#### `useWidget(id: string)`

Subscribe to specific widget updates.

```tsx
const { widget, isLoading } = useWidget('cpu-widget');

if (widget) {
  console.log(widget.data.cpu_percent); // 45.5
}
```

#### `useConfig()`

Access and update configuration.

```tsx
const { config, updateConfig, reload } = useConfig();

// Reload from disk
await reload();

// Update in memory
await updateConfig(newConfig);
```

#### `useExport()`

Export widgets in various formats.

```tsx
const { exportAsZebar, exportAsJson, isLoading } = useExport();

const zebarJson = await exportAsZebar();
const jsonStr = await exportAsJson();
```

### Context Hooks

#### `useKomorebiProvider()`

Access the provider instance directly.

```tsx
const provider = useKomorebiProvider();

// Manual operations
await provider.getConfig();
await provider.reloadConfig();
```

#### `useKomorebiContext()`

Access Komorebi state from context.

```tsx
const { workspace, isConnected, isListening, lastUpdate } = useKomorebiContext();
```

#### `useWidgetsContext()`

Access widgets from context.

```tsx
const { widgets, config, isConnected, isLoading } = useWidgetsContext();
```

#### `useWorkspace()`

Shortcut to get workspace only.

```tsx
const workspace = useWorkspace(); // WorkspaceState | null
```

#### `useWidget(id: string)`

Get widget from context.

```tsx
const widget = useWidget('cpu-widget'); // WidgetRuntime | null
```

## Types

### `WorkspaceState`

```typescript
interface WorkspaceState {
  workspace_id: number;        // Current workspace ID
  monitor_index: number;       // Monitor index
  layout: LayoutMode;          // 'bsp' | 'columns' | 'rows' | 'grid' | 'maximized'
  window_count: number;        // Windows in this workspace
  focused: boolean;            // Is workspace focused
}
```

### `WidgetRuntime`

```typescript
interface WidgetRuntime {
  id: string;                  // Widget ID
  type: WidgetType;            // 'System' | 'Process' | 'Command' | 'Http' | 'Custom'
  data: Record<string, any>;   // Current widget data
  last_update: string;         // ISO datetime of last update
  error?: string;              // Error message if any
}
```

### `StateEvent`

```typescript
type StateEvent =
  | { type: 'widget:updated'; widget_id: string; data: any; timestamp: string }
  | { type: 'monitors:changed'; monitors: MonitorInfo[] }
  | { type: 'config:reloaded'; config: Config }
  | { type: 'widget:error'; widget_id: string; error: string; timestamp: string }
  | { type: 'workspace:changed'; workspace: WorkspaceState; timestamp: string };
```

## Examples

### Complete Status Bar

```tsx
import { useKomorebiContext, useWidgetsContext } from '@/lib/providers/context';

export function StatusBar() {
  const { workspace, isConnected } = useKomorebiContext();
  const { widgets, config } = useWidgetsContext();

  if (!isConnected) return <div>Offline</div>;

  const cpuWidget = widgets.find(w => w.type === 'System');

  return (
    <div className="status-bar">
      <div className="left">
        WS{workspace?.workspace_id} [{workspace?.layout}]
      </div>
      <div className="center">
        {cpuWidget && (
          <span>CPU: {(cpuWidget.data.cpu_percent as number).toFixed(0)}%</span>
        )}
      </div>
      <div className="right">
        {config?.widgets.length} widgets
      </div>
    </div>
  );
}
```

### Real-time CPU Monitor

```tsx
import { useWidget } from '@/lib/providers/context';

export function CPUMonitor() {
  const widget = useWidget('system-metrics');

  if (!widget) return <div>Loading...</div>;

  const cpu = widget.data.cpu_percent as number;
  const isBusy = cpu > 80;

  return (
    <div className={`monitor ${isBusy ? 'busy' : 'idle'}`}>
      <div className="label">CPU</div>
      <div className="value">{cpu.toFixed(1)}%</div>
      <div className="bar">
        <div className="fill" style={{ width: `${cpu}%` }} />
      </div>
    </div>
  );
}
```

### Configuration Manager

```tsx
import { useKomorebiProvider, useConfig } from '@/lib/providers/context';

export function ConfigManager() {
  const provider = useKomorebiProvider();
  const { config } = useConfig();

  const handleReload = async () => {
    await provider.reloadConfig();
  };

  const handleExport = async () => {
    const zebar = await provider.exportZebar();
    console.log('Zebar config:', zebar);
  };

  return (
    <div className="config-manager">
      <h2>Configuration</h2>
      <p>Widgets: {config?.widgets.length}</p>
      <button onClick={handleReload}>Reload</button>
      <button onClick={handleExport}>Export as Zebar</button>
    </div>
  );
}
```

## Event Subscriptions

Subscribe to specific events:

```tsx
import { getKomorebiProvider } from '@/lib/providers/komorebi';

const provider = getKomorebiProvider();

// Workspace changes
const unsub = provider.onWorkspaceChanged((workspace) => {
  console.log('Workspace changed:', workspace);
});

// Widget updates
const unsub2 = provider.onWidgetUpdated((widget) => {
  console.log('Widget updated:', widget.id, widget.data);
});

// Config changes
const unsub3 = provider.onConfigChanged((config) => {
  console.log('Config changed');
});

// Cleanup
unsub();
unsub2();
unsub3();
```

## Tauri Integration

The SDK uses Tauri's IPC system. Make sure your Tauri app has these commands registered:

- `get_config` - Get current configuration
- `set_config` - Set configuration
- `reload_config` - Reload from disk
- `get_all_widgets` - Get all widgets
- `get_widget_state` - Get specific widget
- `run_widget_action` - Run action on widget
- `export_widgets` - Export in format

And these events emitted:

- `app:ready` - Backend ready
- `workspace:changed` - Workspace changed
- `widget:updated` - Widget data updated
- `monitors:changed` - Monitors changed
- `widget:error` - Widget error
- `config:reloaded` - Config reloaded

## CSS Classes & Styling

The examples use CSS classes like:

- `.status-bar` - Main status bar
- `.workspace-indicator` - Workspace display
- `.widget` - Widget container
- `.metrics` - Metrics display
- `.metric` - Individual metric
- `.bar` - Progress bar
- `.connection-status` - Connection status indicator

You can style these with your own CSS or CSS-in-JS solution.

## Performance Tips

1. **Memoize components** to prevent unnecessary re-renders
2. **Use selective subscriptions** - only listen to events you need
3. **Batch updates** - avoid rapid re-renders
4. **Lazy load widgets** - only render visible widgets

```tsx
import { memo } from 'react';

const WidgetCard = memo(({ widget }: { widget: WidgetRuntime }) => {
  // Component only re-renders if widget prop changes
  return <div>{widget.id}: {JSON.stringify(widget.data)}</div>;
});
```

## Troubleshooting

### "Not connected"

Make sure:
- Tauri backend is running
- `invoke` function is available via `window.__TAURI__.tauri`
- Commands are registered in Tauri backend

### Widgets not updating

Check:
- Backend scheduler is running
- Widget's `update_interval_ms` is not too high
- No errors in widget's data sources
- Event listeners are subscribed

### Komorebi integration not working

Ensure:
- Komorebi window manager is running
- Socket at `~/.komorebi.sock` exists
- Backend has `komorebi` feature enabled

## Next Steps

1. See [examples/KomorebiExamples.tsx](./examples/KomorebiExamples.tsx) for component examples
2. Check [lib/types/komorebi.ts](./lib/types/komorebi.ts) for all type definitions
3. Read the [../../plan/phase-5](../../plan/phase-5) for backend architecture

## Contributing

To extend the SDK:

1. Add types to `lib/types/komorebi.ts`
2. Add provider methods to `lib/providers/komorebi.ts`
3. Add hooks to `lib/hooks/useKomorebi.ts`
4. Add React components to `examples/`
