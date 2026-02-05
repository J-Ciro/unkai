/**
 * React Context Providers for Komorebi
 * Wraps the entire app to provide Komorebi state
 */

'use client'; // For Next.js 13+ server components

import React, { createContext, useContext, ReactNode, useState, useEffect } from 'react';
import {
  WorkspaceState,
  WidgetRuntime,
  Config,
  MonitorInfo,
  WidgetProviderState,
  KomorebiProviderState,
} from '../types/komorebi';
import { getKomorebiProvider, KomorebiProvider } from '../providers/komorebi';

/**
 * Komorebi Context
 */
const KomorebiContext = createContext<KomorebiProviderState | null>(null);
const WidgetContext = createContext<WidgetProviderState | null>(null);
const ProviderContext = createContext<KomorebiProvider | null>(null);

/**
 * Komorebi Provider Component
 * Wrap your app with this to provide Komorebi state
 * 
 * @example
 * ```tsx
 * // In your main app component
 * import { KomorebiContextProvider } from '@/lib/providers/context';
 * 
 * export default function App() {
 *   return (
 *     <KomorebiContextProvider>
 *       <YourApp />
 *     </KomorebiContextProvider>
 *   );
 * }
 * ```
 */
export function KomorebiContextProvider({ children }: { children: ReactNode }) {
  const [komorebi, setKomorebi] = useState<KomorebiProviderState>({
    workspace: null,
    isConnected: false,
    isListening: false,
    lastUpdate: null,
  });

  const [widgets, setWidgets] = useState<WidgetProviderState>({
    widgets: [],
    workspace: komorebi.workspace || {
      workspace_id: 1,
      monitor_index: 0,
      layout: 'bsp',
      window_count: 0,
      focused: false,
    },
    config: null,
    monitors: [],
    isConnected: false,
    isLoading: true,
  });

  const provider = getKomorebiProvider();

  useEffect(() => {
    // Connect to backend
    let isMounted = true;

    (async () => {
      const connected = await provider.connect();

      if (!isMounted) return;

      setKomorebi((prev) => ({
        ...prev,
        isConnected: connected,
        isListening: true,
      }));

      // Load initial state
      const config = await provider.getConfig();
      const allWidgets = await provider.getAllWidgets();

      if (!isMounted) return;

      setWidgets((prev) => ({
        ...prev,
        config,
        widgets: allWidgets,
        isConnected: connected,
        isLoading: false,
      }));
    })();

    // Subscribe to events
    const unsub1 = provider.onWorkspaceChanged((workspace) => {
      if (!isMounted) return;
      setKomorebi((prev) => ({
        ...prev,
        workspace,
        lastUpdate: new Date().toISOString(),
      }));
      setWidgets((prev) => ({
        ...prev,
        workspace,
      }));
    });

    const unsub2 = provider.onWidgetUpdated((widget) => {
      if (!isMounted) return;
      setWidgets((prev) => ({
        ...prev,
        widgets: prev.widgets.map((w) => (w.id === widget.id ? widget : w)),
      }));
    });

    const unsub3 = provider.onConfigChanged((config) => {
      if (!isMounted) return;
      setWidgets((prev) => ({
        ...prev,
        config,
      }));
    });

    const unsub4 = provider.subscribe('connected', (connected: boolean) => {
      if (!isMounted) return;
      setKomorebi((prev) => ({
        ...prev,
        isConnected: connected,
      }));
      setWidgets((prev) => ({
        ...prev,
        isConnected: connected,
      }));
    });

    return () => {
      isMounted = false;
      unsub1();
      unsub2();
      unsub3();
      unsub4();
    };
  }, []);

  return (
    <ProviderContext.Provider value={provider}>
      <KomorebiContext.Provider value={komorebi}>
        <WidgetContext.Provider value={widgets}>{children}</WidgetContext.Provider>
      </KomorebiContext.Provider>
    </ProviderContext.Provider>
  );
}

/**
 * Hook: useKomorebiProvider
 * Access the Komorebi provider instance directly
 * 
 * @example
 * ```tsx
 * function MyComponent() {
 *   const provider = useKomorebiProvider();
 *   return <button onClick={() => provider.reloadConfig()}>Reload</button>;
 * }
 * ```
 */
export function useKomorebiProvider(): KomorebiProvider {
  const provider = useContext(ProviderContext);
  if (!provider) {
    throw new Error('useKomorebiProvider must be used within KomorebiContextProvider');
  }
  return provider;
}

/**
 * Hook: useKomorebiContext
 * Access workspace state
 * 
 * @example
 * ```tsx
 * function WorkspaceStatus() {
 *   const { workspace, isConnected } = useKomorebiContext();
 *   return (
 *     <div>
 *       {isConnected ? 'Connected' : 'Disconnected'}
 *       WS {workspace?.workspace_id}
 *     </div>
 *   );
 * }
 * ```
 */
export function useKomorebiContext(): KomorebiProviderState {
  const context = useContext(KomorebiContext);
  if (!context) {
    throw new Error('useKomorebiContext must be used within KomorebiContextProvider');
  }
  return context;
}

/**
 * Hook: useWidgetsContext
 * Access widget state
 * 
 * @example
 * ```tsx
 * function WidgetsList() {
 *   const { widgets, isLoading } = useWidgetsContext();
 *   if (isLoading) return <div>Loading...</div>;
 *   return (
 *     <div>
 *       {widgets.map(w => (
 *         <div key={w.id}>{w.id}: {JSON.stringify(w.data)}</div>
 *       ))}
 *     </div>
 *   );
 * }
 * ```
 */
export function useWidgetsContext(): WidgetProviderState {
  const context = useContext(WidgetContext);
  if (!context) {
    throw new Error('useWidgetsContext must be used within KomorebiContextProvider');
  }
  return context;
}

/**
 * Hook: useWidget
 * Get specific widget from context
 */
export function useWidget(id: string): WidgetRuntime | null {
  const { widgets } = useWidgetsContext();
  return widgets.find((w) => w.id === id) || null;
}

/**
 * Hook: useWorkspace
 * Get workspace state from context
 */
export function useWorkspace(): WorkspaceState | null {
  const { workspace } = useKomorebiContext();
  return workspace;
}

// ============================================================================
// SYSTEM PROVIDER CONTEXT
// ============================================================================

import { SystemProviderState } from '../types/system';
import { getSystemProvider, SystemProvider } from '../providers/system';

const SystemContext = createContext<SystemProviderState | null>(null);
const SystemProviderCtx = createContext<SystemProvider | null>(null);

/**
 * System Provider Context Wrapper
 * Provides audio, battery, memory, and network state
 * 
 * @example
 * ```tsx
 * import { SystemProviderContext } from '@/lib/providers/context';
 * 
 * export default function App() {
 *   return (
 *     <SystemProviderContext>
 *       <YourApp />
 *     </SystemProviderContext>
 *   );
 * }
 * ```
 */
export function SystemProviderContext({ children }: { children: ReactNode }) {
  const provider = getSystemProvider();
  const [state, setState] = useState<SystemProviderState>(provider.getState());

  useEffect(() => {
    // Start polling for system information
    provider.startPolling({
      battery: 5000,
      memory: 1000,
      network: 2000,
    });

    // Subscribe to events
    const unsubscribeAudio = provider.on('audio:volume-changed', () => {
      setState(provider.getState());
    });

    const unsubscribeBattery = provider.on('battery:status-changed', () => {
      setState(provider.getState());
    });

    const unsubscribeMemory = provider.on('memory:updated', () => {
      setState(provider.getState());
    });

    const unsubscribeNetwork = provider.on('network:traffic-updated', () => {
      setState(provider.getState());
    });

    return () => {
      unsubscribeAudio();
      unsubscribeBattery();
      unsubscribeMemory();
      unsubscribeNetwork();
      provider.stopPolling();
    };
  }, []);

  return (
    <SystemContext.Provider value={state}>
      <SystemProviderCtx.Provider value={provider}>
        {children}
      </SystemProviderCtx.Provider>
    </SystemContext.Provider>
  );
}

/**
 * Hook: useSystemContext
 * Access all system provider state
 */
export function useSystemContext(): SystemProviderState {
  const context = useContext(SystemContext);
  if (!context) {
    throw new Error('useSystemContext must be used within SystemProviderContext');
  }
  return context;
}

/**
 * Hook: useSystemProvider
 * Access system provider instance directly
 */
export function useSystemProvider(): SystemProvider {
  const context = useContext(SystemProviderCtx);
  if (!context) {
    throw new Error('useSystemProvider must be used within SystemProviderContext');
  }
  return context;
}
