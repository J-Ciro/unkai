/**
 * Example React Components using Komorebi Providers
 * 
 * These examples show how to build a custom status bar with YASB
 * You can use these with React, Next.js, or any React-based Tauri app
 */

'use client'; // For Next.js 13+

import React, { FC } from 'react';
import {
  useKomorebiContext,
  useWidgetsContext,
  useKomorebiProvider,
  useWidget,
  useWorkspace,
} from '../lib/providers/context';

/**
 * Example 1: Workspace Indicator
 * Shows current workspace, layout, and window count
 */
export const WorkspaceIndicator: FC = () => {
  const { workspace, isConnected } = useKomorebiContext();

  if (!workspace) {
    return (
      <div className="workspace-indicator loading">
        {isConnected ? 'Loading...' : 'Not connected'}
      </div>
    );
  }

  return (
    <div className="workspace-indicator">
      <span className="label">Workspace:</span>
      <span className="value">{workspace.workspace_id}</span>
      <span className="label">Layout:</span>
      <span className="value">{workspace.layout.toUpperCase()}</span>
      <span className="label">Windows:</span>
      <span className="value">{workspace.window_count}</span>
    </div>
  );
};

/**
 * Example 2: System Metrics Widget
 * Displays CPU and memory metrics
 */
export const SystemMetricsWidget: FC<{ widgetId: string }> = ({ widgetId }) => {
  const widget = useWidget(widgetId);

  if (!widget) {
    return <div className="widget loading">Loading {widgetId}...</div>;
  }

  const { cpu_percent, memory_percent, memory_mb, total_memory_mb } = widget.data as any;

  return (
    <div className="widget system-metrics">
      <div className="widget-title">{widgetId}</div>
      <div className="metrics">
        {cpu_percent !== undefined && (
          <div className="metric">
            <span className="label">CPU</span>
            <span className="value">{cpu_percent.toFixed(1)}%</span>
            <div className="bar">
              <div
                className="fill"
                style={{ width: `${Math.min(cpu_percent, 100)}%` }}
              />
            </div>
          </div>
        )}
        {memory_percent !== undefined && (
          <div className="metric">
            <span className="label">Memory</span>
            <span className="value">{memory_percent.toFixed(1)}%</span>
            <div className="bar">
              <div
                className="fill"
                style={{ width: `${Math.min(memory_percent, 100)}%` }}
              />
            </div>
          </div>
        )}
        {memory_mb !== undefined && total_memory_mb !== undefined && (
          <div className="metric">
            <span className="label">Used</span>
            <span className="value">
              {Math.round(memory_mb)} / {Math.round(total_memory_mb)} MB
            </span>
          </div>
        )}
      </div>
    </div>
  );
};

/**
 * Example 3: Widget List
 * Shows all available widgets
 */
export const WidgetList: FC = () => {
  const { widgets, isLoading } = useWidgetsContext();

  if (isLoading) {
    return <div className="widget-list loading">Loading widgets...</div>;
  }

  if (widgets.length === 0) {
    return <div className="widget-list empty">No widgets configured</div>;
  }

  return (
    <div className="widget-list">
      {widgets.map((widget) => (
        <div key={widget.id} className="widget-item">
          <div className="widget-id">{widget.id}</div>
          <div className="widget-type">{widget.type}</div>
          {widget.error && <div className="widget-error">{widget.error}</div>}
          {!widget.error && (
            <div className="widget-data">
              {Object.entries(widget.data).map(([key, value]) => (
                <div key={key} className="data-item">
                  <span className="key">{key}:</span>
                  <span className="value">
                    {typeof value === 'number' ? value.toFixed(2) : String(value)}
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
};

/**
 * Example 4: Custom Status Bar
 * A complete status bar showing workspace + widgets
 */
export const StatusBar: FC = () => {
  const workspace = useWorkspace();
  const { widgets } = useWidgetsContext();

  return (
    <div className="status-bar">
      <div className="left">
        {workspace && (
          <div className="workspace-badge">
            WS{workspace.workspace_id} <span className="layout">[{workspace.layout}]</span>
          </div>
        )}
      </div>

      <div className="center">
        <div className="widgets-mini">
          {widgets.slice(0, 3).map((w) => (
            <div key={w.id} className="widget-mini">
              {w.data.cpu_percent && (
                <span>CPU: {(w.data.cpu_percent as number).toFixed(0)}%</span>
              )}
              {w.data.memory_percent && (
                <span>MEM: {(w.data.memory_percent as number).toFixed(0)}%</span>
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="right">
        <div className="timestamp">{new Date().toLocaleTimeString()}</div>
      </div>
    </div>
  );
};

/**
 * Example 5: Configuration Panel
 * Reload configuration and export widgets
 */
export const ConfigPanel: FC = () => {
  const provider = useKomorebiProvider();
  const { config } = useWidgetsContext();
  const [exporting, setExporting] = React.useState(false);

  const handleReload = async () => {
    await provider.reloadConfig();
  };

  const handleExportZebar = async () => {
    setExporting(true);
    try {
      const zebar = await provider.exportZebar();
      if (zebar) {
        console.log('Zebar export:', JSON.stringify(zebar, null, 2));
        // You could download or send to Zebar here
      }
    } finally {
      setExporting(false);
    }
  };

  return (
    <div className="config-panel">
      <h2>Configuration</h2>
      <p>Version: {config?.version}</p>
      <p>Widgets: {config?.widgets.length || 0}</p>

      <div className="actions">
        <button onClick={handleReload}>Reload Config</button>
        <button onClick={handleExportZebar} disabled={exporting}>
          {exporting ? 'Exporting...' : 'Export as Zebar'}
        </button>
      </div>
    </div>
  );
};

/**
 * Example 6: Connection Status
 * Shows backend connection status
 */
export const ConnectionStatus: FC = () => {
  const { isConnected, lastUpdate } = useKomorebiContext();

  return (
    <div className={`connection-status ${isConnected ? 'connected' : 'disconnected'}`}>
      <div className="indicator" />
      <span>{isConnected ? 'Connected' : 'Disconnected'}</span>
      {lastUpdate && (
        <span className="timestamp">
          Last update: {new Date(lastUpdate).toLocaleTimeString()}
        </span>
      )}
    </div>
  );
};

/**
 * Example 7: Complete App Layout
 * Shows how to structure a full Tauri app with providers
 */
export const CompleteApp: FC = () => {
  return (
    <div className="app-layout">
      <header className="header">
        <h1>YASB Status Bar</h1>
        <ConnectionStatus />
      </header>

      <main className="main">
        <aside className="sidebar">
          <div className="section">
            <h3>Workspace</h3>
            <WorkspaceIndicator />
          </div>

          <div className="section">
            <h3>System</h3>
            <SystemMetricsWidget widgetId="cpu-memory" />
          </div>

          <div className="section">
            <h3>Configuration</h3>
            <ConfigPanel />
          </div>
        </aside>

        <section className="content">
          <h2>All Widgets</h2>
          <WidgetList />
        </section>
      </main>

      <footer className="footer">
        <StatusBar />
      </footer>
    </div>
  );
};
