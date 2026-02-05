/**
 * Type definitions for Komorebi and YASB integration
 * Matches Rust engine-core data structures
 */

/** Layout modes from Komorebi */
export type LayoutMode = 'bsp' | 'columns' | 'rows' | 'grid' | 'maximized';

/** Widget types from unkai */
export type WidgetType = 'System' | 'Process' | 'Command' | 'Http' | 'Custom';

/** Data source kinds */
export type DataSourceKind = 'System' | 'Process' | 'Exec' | 'Http';

/**
 * Komorebi workspace state
 * Represents current workspace, layout, and window info
 */
export interface WorkspaceState {
  workspace_id: number;
  monitor_index: number;
  layout: LayoutMode;
  window_count: number;
  focused: boolean;
}

/**
 * Komorebi workspace event
 * Emitted when workspace changes
 */
export interface WorkspaceEvent {
  workspace_id: number;
  monitor_index: number;
  layout: string;
  window_count: number;
  focused: boolean;
}

/**
 * Monitor information
 */
export interface MonitorInfo {
  id: string;
  name: string;
  dpi: u32;
  x: number;
  y: number;
  width: number;
  height: number;
  is_primary: boolean;
}

/**
 * Widget data source configuration
 */
export interface DataSourceConfig {
  name: string;
  kind: DataSourceKind;
  config: Record<string, unknown>;
  cache_ttl_ms?: number;
}

/**
 * Individual widget configuration
 */
export interface WidgetConfig {
  id: string;
  type: WidgetType;
  template: string;
  monitor?: string;
  position?: string;
  width?: number;
  height?: number;
  update_interval_ms: number;
  data_sources: DataSourceConfig[];
  enabled?: boolean;
}

/**
 * Global configuration
 */
export interface GlobalConfig {
  debug?: boolean;
  log_level?: string;
}

/**
 * Global styles
 */
export interface GlobalStyles {
  variables?: Record<string, string>;
  theme?: string;
}

/**
 * Complete app configuration
 */
export interface Config {
  version: string;
  widgets: WidgetConfig[];
  styles?: GlobalStyles;
  displays?: unknown[];
  global?: GlobalConfig;
}

/**
 * Widget runtime state
 * Current data and metadata for a widget
 */
export interface WidgetRuntime {
  id: string;
  type: WidgetType;
  data: Record<string, unknown>;
  last_update: string; // ISO datetime
  error?: string;
}

/**
 * State events from backend
 */
export type StateEvent =
  | {
      type: 'widget:updated';
      widget_id: string;
      data: Record<string, unknown>;
      timestamp: string;
    }
  | {
      type: 'monitors:changed';
      monitors: MonitorInfo[];
    }
  | {
      type: 'config:reloaded';
      config: Config;
    }
  | {
      type: 'widget:error';
      widget_id: string;
      error: string;
      timestamp: string;
    }
  | {
      type: 'workspace:changed';
      workspace: WorkspaceState;
      timestamp: string;
    };

/**
 * Response types from backend commands
 */
export interface CommandResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

/**
 * Template context for rendering
 */
export interface TemplateContext {
  widget_data: Record<string, unknown>;
  workspace_id?: number;
  layout?: string;
  window_count?: number;
  custom?: Record<string, string>;
}

/**
 * Zebar export format (compatible with Zebar widget system)
 */
export interface ZebarWidget {
  id: string;
  type: string;
  template: string;
  data: Record<string, unknown>;
  styles?: Record<string, string>;
}

export interface ZebarExport {
  version: string;
  timestamp: string;
  widgets: ZebarWidget[];
  styles?: string;
}

/**
 * Export format options
 */
export type ExportFormat = 'json' | 'html' | 'css' | 'zebar';

/**
 * Widget provider context
 */
export interface WidgetProviderState {
  widgets: WidgetRuntime[];
  workspace: WorkspaceState;
  config: Config | null;
  monitors: MonitorInfo[];
  isConnected: boolean;
  isLoading: boolean;
  error?: string;
}

/**
 * Komorebi provider context
 */
export interface KomorebiProviderState {
  workspace: WorkspaceState | null;
  isConnected: boolean;
  isListening: boolean;
  error?: string;
  lastUpdate: string | null;
}
