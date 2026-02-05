/**
 * Komorebi Provider
 * Handles socket communication with Komorebi backend
 * Emits workspace state changes to React/Tauri frontend
 */

import { WorkspaceState, WorkspaceEvent, StateEvent, CommandResponse, Config, WidgetRuntime, MonitorInfo, ExportFormat, ZebarExport } from '../types/komorebi';

/** Event listener type */
type EventListener<T> = (event: T) => void;

/**
 * Komorebi Provider
 * Manages:
 * - Socket connection to YASB backend
 * - Tauri IPC commands (get_config, set_config, etc.)
 * - Event subscriptions (workspace:changed, widget:updated, etc.)
 * - Workspace state management
 */
export class KomorebiProvider {
  private listeners: Map<string, Set<EventListener<any>>> = new Map();
  private workspace: WorkspaceState | null = null;
  private config: Config | null = null;
  private widgets: Map<string, WidgetRuntime> = new Map();
  private isConnected: boolean = false;
  private reconnectAttempts: number = 0;
  private maxReconnectAttempts: number = 5;

  constructor() {
    this.initializeTauriListeners();
  }

  /**
   * Initialize Tauri event listeners
   */
  private initializeTauriListeners(): void {
    if (typeof window === 'undefined') return;

    const { listen } = (window as any).__TAURI__?.tauri;
    if (!listen) return;

    // Listen for app ready
    listen('app:ready', () => {
      this.setConnected(true);
      this.emit('connected', true);
      console.log('[Komorebi] Backend connected');
    });

    // Listen for workspace changes
    listen('workspace:changed', (event: any) => {
      const workspace = event.payload.workspace;
      this.setWorkspace(workspace);
      this.emit('workspace:changed', workspace);
      console.log('[Komorebi] Workspace changed:', workspace);
    });

    // Listen for widget updates
    listen('widget:updated', (event: any) => {
      const { widget_id, data } = event.payload;
      const widget: WidgetRuntime = {
        id: widget_id,
        type: 'System',
        data,
        last_update: new Date().toISOString(),
      };
      this.widgets.set(widget_id, widget);
      this.emit('widget:updated', widget);
      console.log('[Komorebi] Widget updated:', widget_id);
    });

    // Listen for monitor changes
    listen('monitors:changed', (event: any) => {
      this.emit('monitors:changed', event.payload.monitors);
      console.log('[Komorebi] Monitors changed');
    });

    // Listen for widget errors
    listen('widget:error', (event: any) => {
      const { widget_id, error } = event.payload;
      this.emit('widget:error', { widget_id, error });
      console.error('[Komorebi] Widget error:', widget_id, error);
    });

    // Listen for config reload
    listen('config:reloaded', (event: any) => {
      this.config = event.payload.config;
      this.emit('config:reloaded', this.config);
      console.log('[Komorebi] Config reloaded');
    });
  }

  // =========================================================================
  // Socket Connection Management
  // =========================================================================

  /**
   * Connect to backend
   */
  public async connect(): Promise<boolean> {
    try {
      if (this.isConnected) return true;

      const { invoke } = (window as any).__TAURI__?.tauri;
      if (!invoke) {
        console.warn('[Komorebi] Tauri IPC not available');
        return false;
      }

      // Try to load config to verify connection
      const response: CommandResponse<Config> = await invoke('get_config');
      if (response.success && response.data) {
        this.config = response.data;
        this.setConnected(true);
        this.reconnectAttempts = 0;
        return true;
      }

      return false;
    } catch (err) {
      console.error('[Komorebi] Connection failed:', err);
      this.reconnectAttempts++;
      return false;
    }
  }

  /**
   * Disconnect from backend
   */
  public async disconnect(): Promise<void> {
    this.setConnected(false);
    this.listeners.clear();
  }

  /**
   * Check if connected
   */
  public getConnected(): boolean {
    return this.isConnected;
  }

  // =========================================================================
  // Workspace Management
  // =========================================================================

  /**
   * Get current workspace state
   */
  public getWorkspace(): WorkspaceState | null {
    return this.workspace;
  }

  /**
   * Set workspace state
   */
  private setWorkspace(workspace: WorkspaceState): void {
    this.workspace = workspace;
  }

  /**
   * Subscribe to workspace changes
   */
  public onWorkspaceChanged(listener: EventListener<WorkspaceState>): () => void {
    return this.on('workspace:changed', listener);
  }

  // =========================================================================
  // Widget Management
  // =========================================================================

  /**
   * Get all widgets
   */
  public async getAllWidgets(): Promise<WidgetRuntime[]> {
    try {
      const { invoke } = (window as any).__TAURI__?.tauri;
      if (!invoke) return [];

      const response: CommandResponse<any[]> = await invoke('get_all_widgets');
      if (response.success && response.data) {
        response.data.forEach((w) => {
          const widget: WidgetRuntime = {
            id: w.id,
            type: w.type,
            data: w.data || {},
            last_update: w.last_update || new Date().toISOString(),
            error: w.error,
          };
          this.widgets.set(w.id, widget);
        });
        return response.data;
      }
      return [];
    } catch (err) {
      console.error('[Komorebi] Failed to get widgets:', err);
      return [];
    }
  }

  /**
   * Get specific widget
   */
  public async getWidget(id: string): Promise<WidgetRuntime | null> {
    try {
      const { invoke } = (window as any).__TAURI__?.tauri;
      if (!invoke) return null;

      const response: CommandResponse<any> = await invoke('get_widget_state', { id });
      if (response.success && response.data) {
        const widget: WidgetRuntime = {
          id: response.data.id,
          type: 'System',
          data: response.data.data || {},
          last_update: response.data.last_update || new Date().toISOString(),
          error: response.data.error,
        };
        this.widgets.set(id, widget);
        return widget;
      }
      return null;
    } catch (err) {
      console.error('[Komorebi] Failed to get widget:', id, err);
      return null;
    }
  }

  /**
   * Subscribe to widget updates
   */
  public onWidgetUpdated(listener: EventListener<WidgetRuntime>): () => void {
    return this.on('widget:updated', listener);
  }

  // =========================================================================
  // Configuration Management
  // =========================================================================

  /**
   * Get current configuration
   */
  public async getConfig(): Promise<Config | null> {
    try {
      const { invoke } = (window as any).__TAURI__?.tauri;
      if (!invoke) return null;

      const response: CommandResponse<Config> = await invoke('get_config');
      if (response.success && response.data) {
        this.config = response.data;
        return response.data;
      }
      return null;
    } catch (err) {
      console.error('[Komorebi] Failed to get config:', err);
      return null;
    }
  }

  /**
   * Set configuration
   */
  public async setConfig(config: Config): Promise<boolean> {
    try {
      const { invoke } = (window as any).__TAURI__?.tauri;
      if (!invoke) return false;

      const response: CommandResponse<string> = await invoke('set_config', { config });
      if (response.success) {
        this.config = config;
        this.emit('config:changed', config);
        return true;
      }
      return false;
    } catch (err) {
      console.error('[Komorebi] Failed to set config:', err);
      return false;
    }
  }

  /**
   * Reload configuration from disk
   */
  public async reloadConfig(): Promise<Config | null> {
    try {
      const { invoke } = (window as any).__TAURI__?.tauri;
      if (!invoke) return null;

      const response: CommandResponse<Config> = await invoke('reload_config');
      if (response.success && response.data) {
        this.config = response.data;
        this.emit('config:reloaded', response.data);
        return response.data;
      }
      return null;
    } catch (err) {
      console.error('[Komorebi] Failed to reload config:', err);
      return null;
    }
  }

  /**
   * Subscribe to config changes
   */
  public onConfigChanged(listener: EventListener<Config>): () => void {
    return this.on('config:changed', listener);
  }

  /**
   * Subscribe to config reload
   */
  public onConfigReloaded(listener: EventListener<Config>): () => void {
    return this.on('config:reloaded', listener);
  }

  // =========================================================================
  // Export Management
  // =========================================================================

  /**
   * Export widgets in specified format
   * Formats: 'json', 'html', 'css', 'zebar'
   */
  public async exportWidgets(format: ExportFormat): Promise<string | null> {
    try {
      const { invoke } = (window as any).__TAURI__?.tauri;
      if (!invoke) return null;

      const response: CommandResponse<string> = await invoke('export_widgets', { format });
      if (response.success && response.data) {
        return response.data;
      }
      return null;
    } catch (err) {
      console.error('[Komorebi] Failed to export widgets:', err);
      return null;
    }
  }

  /**
   * Export widgets as Zebar format
   */
  public async exportZebar(): Promise<ZebarExport | null> {
    try {
      const json = await this.exportWidgets('zebar');
      if (json) {
        return JSON.parse(json);
      }
      return null;
    } catch (err) {
      console.error('[Komorebi] Failed to export as Zebar:', err);
      return null;
    }
  }

  // =========================================================================
  // Event Management
  // =========================================================================

  /**
   * Subscribe to event
   */
  private on<T>(event: string, listener: EventListener<T>): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(listener);

    // Return unsubscribe function
    return () => {
      const listeners = this.listeners.get(event);
      if (listeners) {
        listeners.delete(listener);
      }
    };
  }

  /**
   * Emit event
   */
  private emit<T>(event: string, data: T): void {
    const listeners = this.listeners.get(event);
    if (listeners) {
      listeners.forEach((listener) => {
        try {
          listener(data);
        } catch (err) {
          console.error(`[Komorebi] Error in listener for ${event}:`, err);
        }
      });
    }
  }

  /**
   * Subscribe to any event
   */
  public subscribe(event: string, listener: EventListener<any>): () => void {
    return this.on(event, listener);
  }

  // =========================================================================
  // State Accessors
  // =========================================================================

  private setConnected(connected: boolean): void {
    this.isConnected = connected;
  }

  /**
   * Get workspace ID
   */
  public getWorkspaceId(): number {
    return this.workspace?.workspace_id || 1;
  }

  /**
   * Get layout mode
   */
  public getLayout(): string {
    return this.workspace?.layout || 'bsp';
  }

  /**
   * Get window count in current workspace
   */
  public getWindowCount(): number {
    return this.workspace?.window_count || 0;
  }

  /**
   * Check if workspace is focused
   */
  public isWorkspaceFocused(): boolean {
    return this.workspace?.focused || false;
  }
}

/** Singleton instance */
let instance: KomorebiProvider | null = null;

/**
 * Get or create Komorebi provider instance
 */
export function getKomorebiProvider(): KomorebiProvider {
  if (!instance) {
    instance = new KomorebiProvider();
  }
  return instance;
}

/**
 * Create new Komorebi provider instance
 */
export function createKomorebiProvider(): KomorebiProvider {
  return new KomorebiProvider();
}
