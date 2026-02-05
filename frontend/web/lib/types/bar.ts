/**
 * Status bar management types
 * Types for custom bar discovery and selection
 */

export type BarAnchor =
  | 'top_left'
  | 'top_center'
  | 'top_right'
  | 'bottom_left'
  | 'bottom_center'
  | 'bottom_right';

/**
 * Bar window configuration
 */
export interface BarWindowConfig {
  width: number;
  height: number;
  offsetX?: number;
  offsetY?: number;
  anchor: BarAnchor;
  alwaysOnTop: boolean;
  transparent: boolean;
  resizable: boolean;
  shownInTaskbar: boolean;
}

/**
 * Bar manifest - defines a custom status bar
 * Stored in barpack.json in each bar directory
 */
export interface BarManifest {
  name: string;
  version: string;
  description: string;
  entry: string; // relative path to compiled HTML
  window: BarWindowConfig;
  metadata?: Record<string, any>;
}

/**
 * Bar info for discovery
 */
export interface BarInfo {
  id: string; // directory name
  name: string;
  description: string;
  version: string;
  path: string; // full path to bar directory
  active: boolean;
}

/**
 * Bar management state
 */
export interface BarManagerState {
  availableBars: BarInfo[];
  selectedBar: BarInfo | null;
  isLoading: boolean;
  error: string | null;
}
