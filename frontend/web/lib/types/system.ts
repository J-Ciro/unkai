/**
 * System provider type definitions
 * Audio, Battery, Memory, and Network provider types
 */

// ============================================================================
// AUDIO TYPES
// ============================================================================

export type AudioDeviceType = 'playback' | 'recording';

export interface AudioDevice {
  deviceId: string;
  name: string;
  volume: number; // 0-100
  deviceType: AudioDeviceType;
  isDefault: boolean;
}

export interface AudioProviderState {
  playbackDevices: AudioDevice[];
  recordingDevices: AudioDevice[];
  defaultPlaybackDevice: AudioDevice | null;
  defaultRecordingDevice: AudioDevice | null;
  isLoading: boolean;
  error: string | null;
}

// ============================================================================
// BATTERY TYPES
// ============================================================================

export type BatteryStateType =
  | 'charging'
  | 'discharging'
  | 'full'
  | 'empty'
  | 'unknown';

export interface BatteryStatus {
  chargePercent: number; // 0-100
  state: BatteryStateType;
  healthPercent?: number; // 0-100
  timeTillEmpty?: number; // milliseconds
  timeTillFull?: number; // milliseconds
  powerConsumption?: number; // watts
}

export interface BatteryProviderState {
  status: BatteryStatus | null;
  isCharging: boolean;
  isAvailable: boolean;
  isLoading: boolean;
  error: string | null;
  lastUpdated: number; // timestamp
}

// ============================================================================
// MEMORY TYPES
// ============================================================================

export interface MemoryStats {
  total: number; // bytes
  used: number; // bytes
  free: number; // bytes
  available: number; // bytes
  buffers?: number; // bytes
  cached?: number; // bytes
  swapTotal?: number; // bytes
  swapUsed?: number; // bytes
  swapFree?: number; // bytes
}

export interface MemoryProviderState {
  stats: MemoryStats | null;
  usagePercent: number; // 0-100
  isLoading: boolean;
  error: string | null;
  lastUpdated: number; // timestamp
}

// ============================================================================
// NETWORK TYPES
// ============================================================================

export interface NetworkInterface {
  name: string;
  ipAddress?: string;
  macAddress?: string;
  isUp: boolean;
  isDefault: boolean;
}

export interface NetworkTraffic {
  receivedPerSec: number; // bytes
  transmittedPerSec: number; // bytes
  totalReceived: number; // bytes
  totalTransmitted: number; // bytes
}

export interface NetworkProviderState {
  interfaces: NetworkInterface[];
  defaultInterface: NetworkInterface | null;
  traffic: NetworkTraffic | null;
  isLoading: boolean;
  error: string | null;
  lastUpdated: number; // timestamp
}

// ============================================================================
// COMBINED SYSTEM PROVIDER STATE
// ============================================================================

export interface SystemProviderState {
  audio: AudioProviderState;
  battery: BatteryProviderState;
  memory: MemoryProviderState;
  network: NetworkProviderState;
  isConnected: boolean;
  isLoading: boolean;
}
