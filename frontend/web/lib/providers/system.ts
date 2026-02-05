import { invoke } from '@tauri-apps/api/tauri';
import {
  AudioDevice,
  AudioProviderState,
  BatteryProviderState,
  BatteryStatus,
  MemoryProviderState,
  MemoryStats,
  NetworkProviderState,
  NetworkTraffic,
  SystemProviderState,
} from '../types/system';

export type SystemEventType =
  | 'audio:devices-changed'
  | 'audio:volume-changed'
  | 'battery:status-changed'
  | 'memory:updated'
  | 'network:traffic-updated';

type SystemEventListener<T = any> = (data: T) => void;

/**
 * System provider for audio, battery, memory, and network information
 */
export class SystemProvider {
  private audioState: AudioProviderState = {
    playbackDevices: [],
    recordingDevices: [],
    defaultPlaybackDevice: null,
    defaultRecordingDevice: null,
    isLoading: false,
    error: null,
  };

  private batteryState: BatteryProviderState = {
    status: null,
    isCharging: false,
    isAvailable: false,
    isLoading: false,
    error: null,
    lastUpdated: 0,
  };

  private memoryState: MemoryProviderState = {
    stats: null,
    usagePercent: 0,
    isLoading: false,
    error: null,
    lastUpdated: 0,
  };

  private networkState: NetworkProviderState = {
    interfaces: [],
    defaultInterface: null,
    traffic: null,
    isLoading: false,
    error: null,
    lastUpdated: 0,
  };

  private eventListeners = new Map<SystemEventType, Set<SystemEventListener>>();
  private isConnected = false;
  private pollIntervals: NodeJS.Timeout[] = [];

  constructor() {
    this.initializeEventListeners();
  }

  /**
   * Initialize Tauri event listeners for system events
   */
  private initializeEventListeners() {
    // System event listeners would be set up here
    // For now, we'll use polling instead
  }

  /**
   * Start polling for system information updates
   */
  async startPolling(intervals?: {
    battery?: number;
    memory?: number;
    network?: number;
  }) {
    const batteryInterval = intervals?.battery ?? 5000;
    const memoryInterval = intervals?.memory ?? 1000;
    const networkInterval = intervals?.network ?? 2000;

    // Battery polling
    this.pollIntervals.push(
      setInterval(() => this.updateBatteryStatus(), batteryInterval)
    );

    // Memory polling
    this.pollIntervals.push(
      setInterval(() => this.updateMemoryStats(), memoryInterval)
    );

    // Network polling
    this.pollIntervals.push(
      setInterval(() => this.updateNetworkTraffic(), networkInterval)
    );

    this.isConnected = true;
  }

  /**
   * Stop all polling intervals
   */
  stopPolling() {
    this.pollIntervals.forEach(interval => clearInterval(interval));
    this.pollIntervals = [];
    this.isConnected = false;
  }

  // ==========================================================================
  // AUDIO METHODS
  // ==========================================================================

  /**
   * Get all audio playback devices
   */
  async getPlaybackDevices(): Promise<AudioDevice[]> {
    try {
      this.audioState.isLoading = true;
      const result = await invoke<AudioDevice[]>('get_audio_playback_devices');
      this.audioState.playbackDevices = result;
      this.audioState.error = null;
      return result;
    } catch (error) {
      this.audioState.error = String(error);
      return [];
    } finally {
      this.audioState.isLoading = false;
    }
  }

  /**
   * Get all audio recording devices
   */
  async getRecordingDevices(): Promise<AudioDevice[]> {
    try {
      this.audioState.isLoading = true;
      const result = await invoke<AudioDevice[]>('get_audio_recording_devices');
      this.audioState.recordingDevices = result;
      this.audioState.error = null;
      return result;
    } catch (error) {
      this.audioState.error = String(error);
      return [];
    } finally {
      this.audioState.isLoading = false;
    }
  }

  /**
   * Get default playback device
   */
  async getDefaultPlaybackDevice(): Promise<AudioDevice | null> {
    try {
      const result = await invoke<AudioDevice | null>(
        'get_default_playback_device'
      );
      this.audioState.defaultPlaybackDevice = result;
      this.audioState.error = null;
      return result;
    } catch (error) {
      this.audioState.error = String(error);
      return null;
    }
  }

  /**
   * Set volume for a device (0-100)
   */
  async setVolume(deviceId: string, volume: number): Promise<void> {
    try {
      if (volume < 0 || volume > 100) {
        throw new Error('Volume must be between 0 and 100');
      }
      await invoke('set_audio_volume', { deviceId, volume });
      this.audioState.error = null;
      this.emit('audio:volume-changed', { deviceId, volume });
    } catch (error) {
      this.audioState.error = String(error);
      throw error;
    }
  }

  /**
   * Get audio provider state
   */
  getAudioState(): AudioProviderState {
    return { ...this.audioState };
  }

  // ==========================================================================
  // BATTERY METHODS
  // ==========================================================================

  /**
   * Update battery status from backend
   */
  async updateBatteryStatus(): Promise<void> {
    try {
      this.batteryState.isLoading = true;
      const result = await invoke<BatteryStatus | null>('get_battery_status');

      if (result) {
        this.batteryState.status = result;
        this.batteryState.isAvailable = true;
        this.batteryState.isCharging = result.state === 'charging';
        this.batteryState.error = null;
        this.batteryState.lastUpdated = Date.now();
        this.emit('battery:status-changed', result);
      } else {
        this.batteryState.isAvailable = false;
      }
    } catch (error) {
      this.batteryState.error = String(error);
      this.batteryState.isAvailable = false;
    } finally {
      this.batteryState.isLoading = false;
    }
  }

  /**
   * Get battery status
   */
  async getBatteryStatus(): Promise<BatteryStatus | null> {
    await this.updateBatteryStatus();
    return this.batteryState.status;
  }

  /**
   * Check if device is charging
   */
  isCharging(): boolean {
    return this.batteryState.isCharging;
  }

  /**
   * Get battery provider state
   */
  getBatteryState(): BatteryProviderState {
    return { ...this.batteryState };
  }

  // ==========================================================================
  // MEMORY METHODS
  // ==========================================================================

  /**
   * Update memory statistics from backend
   */
  async updateMemoryStats(): Promise<void> {
    try {
      this.memoryState.isLoading = true;
      const result = await invoke<MemoryStats>('get_memory_stats');

      this.memoryState.stats = result;
      this.memoryState.usagePercent = (result.used / result.total) * 100;
      this.memoryState.error = null;
      this.memoryState.lastUpdated = Date.now();
      this.emit('memory:updated', result);
    } catch (error) {
      this.memoryState.error = String(error);
    } finally {
      this.memoryState.isLoading = false;
    }
  }

  /**
   * Get memory statistics
   */
  async getMemoryStats(): Promise<MemoryStats | null> {
    await this.updateMemoryStats();
    return this.memoryState.stats;
  }

  /**
   * Get memory usage percentage
   */
  getMemoryUsagePercent(): number {
    return this.memoryState.usagePercent;
  }

  /**
   * Get memory provider state
   */
  getMemoryState(): MemoryProviderState {
    return { ...this.memoryState };
  }

  // ==========================================================================
  // NETWORK METHODS
  // ==========================================================================

  /**
   * Get network interfaces
   */
  async getNetworkInterfaces() {
    try {
      this.networkState.isLoading = true;
      const result = await invoke('get_network_interfaces');
      this.networkState.interfaces = result;
      this.networkState.error = null;
      return result;
    } catch (error) {
      this.networkState.error = String(error);
      return [];
    } finally {
      this.networkState.isLoading = false;
    }
  }

  /**
   * Get default network interface
   */
  async getDefaultNetworkInterface() {
    try {
      const result = await invoke('get_default_network_interface');
      this.networkState.defaultInterface = result || null;
      this.networkState.error = null;
      return result;
    } catch (error) {
      this.networkState.error = String(error);
      return null;
    }
  }

  /**
   * Update network traffic from backend
   */
  async updateNetworkTraffic(): Promise<void> {
    try {
      this.networkState.isLoading = true;
      const result = await invoke<NetworkTraffic>('get_network_traffic');

      this.networkState.traffic = result;
      this.networkState.error = null;
      this.networkState.lastUpdated = Date.now();
      this.emit('network:traffic-updated', result);
    } catch (error) {
      this.networkState.error = String(error);
    } finally {
      this.networkState.isLoading = false;
    }
  }

  /**
   * Get network traffic
   */
  async getNetworkTraffic(): Promise<NetworkTraffic | null> {
    await this.updateNetworkTraffic();
    return this.networkState.traffic;
  }

  /**
   * Get network provider state
   */
  getNetworkState(): NetworkProviderState {
    return { ...this.networkState };
  }

  // ==========================================================================
  // STATE MANAGEMENT
  // ==========================================================================

  /**
   * Get combined system provider state
   */
  getState(): SystemProviderState {
    return {
      audio: this.getAudioState(),
      battery: this.getBatteryState(),
      memory: this.getMemoryState(),
      network: this.getNetworkState(),
      isConnected: this.isConnected,
      isLoading:
        this.audioState.isLoading ||
        this.batteryState.isLoading ||
        this.memoryState.isLoading ||
        this.networkState.isLoading,
    };
  }

  // ==========================================================================
  // EVENT HANDLING
  // ==========================================================================

  /**
   * Subscribe to system events
   */
  on<T = any>(
    event: SystemEventType,
    listener: SystemEventListener<T>
  ): () => void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, new Set());
    }

    this.eventListeners.get(event)!.add(listener);

    // Return unsubscribe function
    return () => {
      this.eventListeners.get(event)?.delete(listener);
    };
  }

  /**
   * Emit system event
   */
  private emit<T = any>(event: SystemEventType, data: T) {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      listeners.forEach(listener => {
        try {
          listener(data);
        } catch (error) {
          console.error(`Error in event listener for ${event}:`, error);
        }
      });
    }
  }

  /**
   * Get is connected state
   */
  getConnected(): boolean {
    return this.isConnected;
  }

  /**
   * Disconnect from system providers
   */
  disconnect() {
    this.stopPolling();
    this.eventListeners.clear();
  }
}

// Singleton instance
let systemProviderInstance: SystemProvider | null = null;

/**
 * Get or create SystemProvider singleton
 */
export function getSystemProvider(): SystemProvider {
  if (!systemProviderInstance) {
    systemProviderInstance = new SystemProvider();
  }
  return systemProviderInstance;
}

/**
 * Create new SystemProvider instance (for testing)
 */
export function createSystemProvider(): SystemProvider {
  return new SystemProvider();
}
