/**
 * React hooks for Komorebi provider
 * Simplifies integration with React/Next.js apps
 */

import { useState, useEffect, useCallback } from 'react';
import {
  WorkspaceState,
  WidgetRuntime,
  Config,
  MonitorInfo,
  ExportFormat,
  ZebarExport,
} from '../types/komorebi';
import { getKomorebiProvider } from '../providers/komorebi';

/**
 * Hook: useKomorebi
 * Get workspace state and subscribe to changes
 * 
 * @example
 * ```tsx
 * function WorkspaceIndicator() {
 *   const { workspace, isConnected } = useKomorebi();
 *   return (
 *     <div>
 *       Workspace: {workspace?.workspace_id}
 *       Layout: {workspace?.layout}
 *     </div>
 *   );
 * }
 * ```
 */
export function useKomorebi() {
  const [workspace, setWorkspace] = useState<WorkspaceState | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const provider = getKomorebiProvider();

    // Connect
    provider.connect().then((connected) => {
      setIsConnected(connected);
      setIsLoading(false);
    });

    // Subscribe to workspace changes
    const unsubscribe = provider.onWorkspaceChanged((ws) => {
      setWorkspace(ws);
    });

    // Subscribe to connection state
    const unsubConnection = provider.subscribe('connected', (connected: boolean) => {
      setIsConnected(connected);
    });

    // Get initial workspace state
    const ws = provider.getWorkspace();
    if (ws) {
      setWorkspace(ws);
    }

    return () => {
      unsubscribe();
      unsubConnection();
    };
  }, []);

  return {
    workspace,
    isConnected,
    isLoading,
    workspaceId: workspace?.workspace_id,
    layout: workspace?.layout,
    windowCount: workspace?.window_count,
    isFocused: workspace?.focused,
  };
}

/**
 * Hook: useWidgets
 * Get widgets and subscribe to updates
 * 
 * @example
 * ```tsx
 * function WidgetList() {
 *   const { widgets, isLoading } = useWidgets();
 *   return (
 *     <div>
 *       {widgets.map(w => (
 *         <Widget key={w.id} data={w.data} />
 *       ))}
 *     </div>
 *   );
 * }
 * ```
 */
export function useWidgets() {
  const [widgets, setWidgets] = useState<WidgetRuntime[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const provider = getKomorebiProvider();

    // Load all widgets
    provider.getAllWidgets().then((ws) => {
      setWidgets(ws);
      setIsLoading(false);
    });

    // Subscribe to widget updates
    const unsubscribe = provider.onWidgetUpdated((widget) => {
      setWidgets((prev) => {
        const index = prev.findIndex((w) => w.id === widget.id);
        if (index >= 0) {
          const updated = [...prev];
          updated[index] = widget;
          return updated;
        }
        return [...prev, widget];
      });
    });

    return () => {
      unsubscribe();
    };
  }, []);

  const getWidget = useCallback((id: string) => {
    return widgets.find((w) => w.id === id) || null;
  }, [widgets]);

  return {
    widgets,
    isLoading,
    error,
    getWidget,
  };
}

/**
 * Hook: useWidget
 * Get specific widget and subscribe to its updates
 * 
 * @example
 * ```tsx
 * function CPUWidget() {
 *   const { widget, isLoading } = useWidget('cpu-widget');
 *   return <div>CPU: {widget?.data.cpu_percent}%</div>;
 * }
 * ```
 */
export function useWidget(id: string) {
  const [widget, setWidget] = useState<WidgetRuntime | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const provider = getKomorebiProvider();

    // Load widget
    provider.getWidget(id).then((w) => {
      setWidget(w);
      setIsLoading(false);
    });

    // Subscribe to updates for this widget
    const unsubscribe = provider.onWidgetUpdated((w) => {
      if (w.id === id) {
        setWidget(w);
      }
    });

    return () => {
      unsubscribe();
    };
  }, [id]);

  return { widget, isLoading };
}

/**
 * Hook: useConfig
 * Get configuration and subscribe to changes
 * 
 * @example
 * ```tsx
 * function ConfigPanel() {
 *   const { config, updateConfig, reload } = useConfig();
 *   return (
 *     <div>
 *       <p>Version: {config?.version}</p>
 *       <button onClick={reload}>Reload</button>
 *     </div>
 *   );
 * }
 * ```
 */
export function useConfig() {
  const [config, setConfig] = useState<Config | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    const provider = getKomorebiProvider();

    // Load config
    provider.getConfig().then((cfg) => {
      setConfig(cfg);
      setIsLoading(false);
    });

    // Subscribe to config changes
    const unsubscribe = provider.onConfigChanged((cfg) => {
      setConfig(cfg);
    });

    const unsubReload = provider.onConfigReloaded((cfg) => {
      setConfig(cfg);
    });

    return () => {
      unsubscribe();
      unsubReload();
    };
  }, []);

  const updateConfig = useCallback(async (newConfig: Config) => {
    setIsSaving(true);
    const provider = getKomorebiProvider();
    const success = await provider.setConfig(newConfig);
    setIsSaving(false);
    return success;
  }, []);

  const reload = useCallback(async () => {
    setIsLoading(true);
    const provider = getKomorebiProvider();
    const cfg = await provider.reloadConfig();
    setIsLoading(false);
    return cfg;
  }, []);

  return {
    config,
    isLoading,
    isSaving,
    updateConfig,
    reload,
  };
}

/**
 * Hook: useExport
 * Export widgets in various formats
 * 
 * @example
 * ```tsx
 * function ExportPanel() {
 *   const { exportAsZebar, exportAsJson, isLoading } = useExport();
 *   return (
 *     <button onClick={() => exportAsZebar()}>
 *       Export as Zebar
 *     </button>
 *   );
 * }
 * ```
 */
export function useExport() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const provider = getKomorebiProvider();

  const export_ = useCallback(
    async (format: ExportFormat): Promise<string | null> => {
      setIsLoading(true);
      setError(null);
      try {
        const result = await provider.exportWidgets(format);
        setIsLoading(false);
        return result;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Export failed';
        setError(message);
        setIsLoading(false);
        return null;
      }
    },
    [provider],
  );

  const exportAsJson = useCallback(() => export_('json'), [export_]);
  const exportAsHtml = useCallback(() => export_('html'), [export_]);
  const exportAsCss = useCallback(() => export_('css'), [export_]);
  const exportAsZebar = useCallback(() => export_('zebar'), [export_]);

  return {
    isLoading,
    error,
    export: export_,
    exportAsJson,
    exportAsHtml,
    exportAsCss,
    exportAsZebar,
  };
}

/**
 * Hook: useKomorebiState
 * Combined hook for all Komorebi state
 * 
 * @example
 * ```tsx
 * function Dashboard() {
 *   const state = useKomorebiState();
 *   return (
 *     <div>
 *       <p>WS {state.workspace?.workspace_id}</p>
 *       <p>{state.widgets.length} widgets</p>
 *     </div>
 *   );
 * }
 * ```
 */
export function useKomorebiState() {
  const komorebi = useKomorebi();
  const widgets = useWidgets();
  const config = useConfig();
  const exportHook = useExport();

  return {
    komorebi,
    widgets,
    config,
    export: exportHook,
  };
}
