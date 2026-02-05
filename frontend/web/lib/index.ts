/**
 * YASB Frontend SDK - Barrel Export
 * Re-exports all public types, providers, and hooks
 */

// ============================================================================
// TYPES
// ============================================================================

export * from './types/komorebi';
export * from './types/system';
export * from './types/bar';

// ============================================================================
// PROVIDERS
// ============================================================================

export { KomorebiProvider, getKomorebiProvider, createKomorebiProvider } from './providers/komorebi';
export { SystemProvider, getSystemProvider, createSystemProvider } from './providers/system';
export { BarManager, getBarManager, createBarManager } from './providers/bar';

// ============================================================================
// CONTEXT PROVIDERS
// ============================================================================

export {
  KomorebiContextProvider,
  useKomorebiProvider,
  useKomorebiContext,
  useWidgetsContext,
  useWidget,
  useWorkspace,
  SystemProviderContext,
  useSystemContext,
  useSystemProvider,
} from './providers/context';

// ============================================================================
// HOOKS - KOMOREBI
// ============================================================================

export {
  useKomorebi,
  useWidgets,
  useWidget as useKomorebiWidget,
  useConfig,
  useExport,
  useKomorebiState,
} from './hooks/useKomorebi';

// ============================================================================
// HOOKS - SYSTEM PROVIDERS
// ============================================================================

export {
  useAudio,
  useBattery,
  useMemory,
  useNetwork,
  useSystemProviders,
  useSystemProviderEvents,
} from './hooks/useSystemProviders';

// ============================================================================
// HOOKS - BAR MANAGEMENT
// ============================================================================

export {
  useBarManager,
  useAvailableBars,
  useSelectedBar,
} from './hooks/useBarManager';

// ============================================================================
// COMPONENTS - BAR MANAGEMENT
// ============================================================================

export {
  BarSelector,
  BarsList,
  CreateBarForm,
  BarSettingsPanel,
  QuickBarSelector,
} from '../components/BarManager';

// ============================================================================
// EXAMPLES
// ============================================================================

export {
  WorkspaceIndicator,
  SystemMetricsWidget,
  WidgetList,
  StatusBar,
  ConfigPanel,
  ConnectionStatus,
  CompleteApp,
} from '../examples/KomorebiExamples';

export {
  BatteryStatusWidget,
  MemoryUsageWidget,
  NetworkTrafficWidget,
  AudioDevicesWidget,
  SystemStatusDashboard,
  QuickStats,
  SystemTrayWidget,
  SystemProviderStyles,
} from '../examples/SystemProviderExamples';
