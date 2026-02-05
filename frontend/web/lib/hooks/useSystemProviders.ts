import { useEffect, useState } from 'react';
import {
  AudioDevice,
  AudioProviderState,
  BatteryProviderState,
  MemoryProviderState,
  MemoryStats,
  NetworkProviderState,
  NetworkTraffic,
  SystemProviderState,
} from '../types/system';
import { getSystemProvider } from '../providers/system';

/**
 * Hook for audio provider state
 */
export function useAudio(): AudioProviderState & {
  setVolume: (deviceId: string, volume: number) => Promise<void>;
  getPlaybackDevices: () => Promise<AudioDevice[]>;
  getRecordingDevices: () => Promise<AudioDevice[]>;
} {
  const provider = getSystemProvider();
  const [state, setState] = useState<AudioProviderState>(provider.getAudioState());

  useEffect(() => {
    const unsubscribe = provider.on('audio:volume-changed', () => {
      setState(provider.getAudioState());
    });

    return unsubscribe;
  }, [provider]);

  return {
    ...state,
    setVolume: (deviceId, volume) => provider.setVolume(deviceId, volume),
    getPlaybackDevices: () => provider.getPlaybackDevices(),
    getRecordingDevices: () => provider.getRecordingDevices(),
  };
}

/**
 * Hook for battery provider state
 */
export function useBattery(): BatteryProviderState & {
  refresh: () => Promise<void>;
  getStatus: () => Promise<void>;
} {
  const provider = getSystemProvider();
  const [state, setState] = useState<BatteryProviderState>(
    provider.getBatteryState()
  );

  useEffect(() => {
    const unsubscribe = provider.on('battery:status-changed', () => {
      setState(provider.getBatteryState());
    });

    return unsubscribe;
  }, [provider]);

  return {
    ...state,
    refresh: () => provider.updateBatteryStatus(),
    getStatus: () => provider.updateBatteryStatus(),
  };
}

/**
 * Hook for memory provider state
 */
export function useMemory(): MemoryProviderState & {
  refresh: () => Promise<void>;
  getStats: () => Promise<MemoryStats | null>;
  getUsagePercent: () => number;
} {
  const provider = getSystemProvider();
  const [state, setState] = useState<MemoryProviderState>(
    provider.getMemoryState()
  );

  useEffect(() => {
    const unsubscribe = provider.on('memory:updated', () => {
      setState(provider.getMemoryState());
    });

    return unsubscribe;
  }, [provider]);

  return {
    ...state,
    refresh: () => provider.updateMemoryStats(),
    getStats: () => provider.getMemoryStats(),
    getUsagePercent: () => provider.getMemoryUsagePercent(),
  };
}

/**
 * Hook for network provider state
 */
export function useNetwork(): NetworkProviderState & {
  refresh: () => Promise<void>;
  getTraffic: () => Promise<NetworkTraffic | null>;
  getInterfaces: () => Promise<any>;
} {
  const provider = getSystemProvider();
  const [state, setState] = useState<NetworkProviderState>(
    provider.getNetworkState()
  );

  useEffect(() => {
    const unsubscribe = provider.on('network:traffic-updated', () => {
      setState(provider.getNetworkState());
    });

    return unsubscribe;
  }, [provider]);

  return {
    ...state,
    refresh: () => provider.updateNetworkTraffic(),
    getTraffic: () => provider.getNetworkTraffic(),
    getInterfaces: () => provider.getNetworkInterfaces(),
  };
}

/**
 * Hook for combined system provider state with polling
 */
export function useSystemProviders(options?: {
  autoPoll?: boolean;
  batteryInterval?: number;
  memoryInterval?: number;
  networkInterval?: number;
}): SystemProviderState & {
  startPolling: () => void;
  stopPolling: () => void;
} {
  const provider = getSystemProvider();
  const [state, setState] = useState<SystemProviderState>(provider.getState());

  useEffect(() => {
    if (options?.autoPoll ?? true) {
      provider.startPolling({
        battery: options?.batteryInterval,
        memory: options?.memoryInterval,
        network: options?.networkInterval,
      });
    }

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
      if (options?.autoPoll ?? true) {
        provider.stopPolling();
      }
    };
  }, [provider, options]);

  return {
    ...state,
    startPolling: () =>
      provider.startPolling({
        battery: options?.batteryInterval,
        memory: options?.memoryInterval,
        network: options?.networkInterval,
      }),
    stopPolling: () => provider.stopPolling(),
  };
}

/**
 * Hook for reactive system provider updates
 * Automatically subscribes to provider events
 */
export function useSystemProviderEvents() {
  const provider = getSystemProvider();

  const on = (event: string, callback: (data: any) => void) => {
    return provider.on(event as any, callback);
  };

  return { on, provider };
}
