import React from 'react';
import { useAudio, useBattery, useMemory, useNetwork, useSystemProviders } from '../hooks/useSystemProviders';
import { useSystemContext, useSystemProvider } from '../providers/context';

/**
 * Battery Status Component
 * Displays current battery status and charging state
 */
export function BatteryStatusWidget() {
  const battery = useBattery();

  if (!battery.isAvailable) {
    return (
      <div className="battery-widget">
        <p>No battery detected (running on AC power)</p>
      </div>
    );
  }

  if (!battery.status) {
    return <div className="battery-widget">Loading battery info...</div>;
  }

  const status = battery.status;
  const batClass = status.chargePercent < 20 ? 'low' : status.chargePercent < 50 ? 'medium' : 'high';

  return (
    <div className={`battery-widget battery-${batClass}`}>
      <div className="battery-header">
        <span>🔋 Battery</span>
        <span>{status.chargePercent.toFixed(1)}%</span>
      </div>
      <div className="battery-bar">
        <div
          className="battery-fill"
          style={{ width: `${status.chargePercent}%` }}
        />
      </div>
      <div className="battery-details">
        <p>Status: {status.state.charAt(0).toUpperCase() + status.state.slice(1)}</p>
        {status.timeTillFull && (
          <p>Time till full: {(status.timeTillFull / 1000 / 60).toFixed(0)} min</p>
        )}
        {status.timeTillEmpty && (
          <p>Time till empty: {(status.timeTillEmpty / 1000 / 60).toFixed(0)} min</p>
        )}
        {status.healthPercent && <p>Health: {status.healthPercent.toFixed(1)}%</p>}
      </div>
    </div>
  );
}

/**
 * Memory Usage Component
 * Displays memory usage with visual progress bar
 */
export function MemoryUsageWidget() {
  const memory = useMemory();

  if (!memory.stats) {
    return <div>Loading memory info...</div>;
  }

  const usedGb = (memory.stats.used / 1024 / 1024 / 1024).toFixed(2);
  const totalGb = (memory.stats.total / 1024 / 1024 / 1024).toFixed(2);
  const usagePercent = memory.usagePercent;
  const memClass = usagePercent > 80 ? 'critical' : usagePercent > 60 ? 'warning' : 'normal';

  return (
    <div className={`memory-widget memory-${memClass}`}>
      <div className="memory-header">
        <span>💾 Memory</span>
        <span>{usagePercent.toFixed(1)}%</span>
      </div>
      <div className="memory-bar">
        <div className="memory-fill" style={{ width: `${usagePercent}%` }} />
      </div>
      <div className="memory-details">
        <p>
          {usedGb} GB / {totalGb} GB
        </p>
        <p>Free: {((memory.stats.free / 1024 / 1024 / 1024).toFixed(2))} GB</p>
      </div>
    </div>
  );
}

/**
 * Network Traffic Component
 * Displays network traffic speeds
 */
export function NetworkTrafficWidget() {
  const network = useNetwork();

  if (!network.traffic) {
    return <div>Loading network info...</div>;
  }

  const traffic = network.traffic;
  const receivedMbps = (traffic.receivedPerSec * 8) / 1_000_000;
  const transmittedMbps = (traffic.transmittedPerSec * 8) / 1_000_000;

  return (
    <div className="network-widget">
      <div className="network-header">
        <span>🌐 Network</span>
        {network.defaultInterface && <span>{network.defaultInterface.name}</span>}
      </div>
      <div className="network-traffic">
        <div className="traffic-item">
          <span>⬇️ Download</span>
          <span className="traffic-value">{receivedMbps.toFixed(2)} Mbps</span>
        </div>
        <div className="traffic-item">
          <span>⬆️ Upload</span>
          <span className="traffic-value">{transmittedMbps.toFixed(2)} Mbps</span>
        </div>
      </div>
      <div className="network-total">
        <p>Total: ↓ {(traffic.totalReceived / 1_000_000_000).toFixed(1)} GB / ↑ {(traffic.totalTransmitted / 1_000_000_000).toFixed(1)} GB</p>
      </div>
    </div>
  );
}

/**
 * Audio Devices Component
 * Lists and manages audio devices
 */
export function AudioDevicesWidget() {
  const audio = useAudio();

  return (
    <div className="audio-widget">
      <div className="audio-header">
        <span>🔊 Audio Devices</span>
      </div>

      <div className="audio-section">
        <h4>Playback Devices</h4>
        {audio.playbackDevices.length === 0 ? (
          <p>No playback devices</p>
        ) : (
          <ul className="device-list">
            {audio.playbackDevices.map(device => (
              <li key={device.deviceId} className={device.isDefault ? 'default' : ''}>
                <span>{device.name}</span>
                <span>Volume: {device.volume.toFixed(0)}%</span>
                <input
                  type="range"
                  min="0"
                  max="100"
                  value={device.volume}
                  onChange={e => audio.setVolume(device.deviceId, parseInt(e.target.value))}
                />
              </li>
            ))}
          </ul>
        )}
      </div>

      <div className="audio-section">
        <h4>Recording Devices</h4>
        {audio.recordingDevices.length === 0 ? (
          <p>No recording devices</p>
        ) : (
          <ul className="device-list">
            {audio.recordingDevices.map(device => (
              <li key={device.deviceId} className={device.isDefault ? 'default' : ''}>
                <span>{device.name}</span>
                <span>Volume: {device.volume.toFixed(0)}%</span>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}

/**
 * System Status Dashboard
 * Comprehensive system information display
 */
export function SystemStatusDashboard() {
  const systemState = useSystemProviders({ autoPoll: true });

  if (!systemState.isConnected) {
    return <div>Connecting to system providers...</div>;
  }

  return (
    <div className="system-dashboard">
      <div className="dashboard-grid">
        <div className="dashboard-card">
          <BatteryStatusWidget />
        </div>
        <div className="dashboard-card">
          <MemoryUsageWidget />
        </div>
        <div className="dashboard-card">
          <NetworkTrafficWidget />
        </div>
        <div className="dashboard-card">
          <AudioDevicesWidget />
        </div>
      </div>
    </div>
  );
}

/**
 * Quick Stats Component
 * Minimal display of key system metrics
 */
export function QuickStats() {
  const system = useSystemContext();

  if (!system) {
    return <div>Loading sys

tem info...</div>;
  }

  return (
    <div className="quick-stats">
      {system.memory.stats && (
        <div className="stat-item">
          <span>RAM</span>
          <span>{system.memory.usagePercent.toFixed(0)}%</span>
        </div>
      )}

      {system.battery.status && (
        <div className="stat-item">
          <span>Battery</span>
          <span>{system.battery.status.chargePercent.toFixed(0)}%</span>
        </div>
      )}

      {system.network.traffic && (
        <div className="stat-item">
          <span>Network</span>
          <span>
            {((system.network.traffic.receivedPerSec * 8) / 1_000_000).toFixed(1)} Mbps
          </span>
        </div>
      )}
    </div>
  );
}

/**
 * System Tray Widget
 * Compact system information for taskbar
 */
export function SystemTrayWidget() {
  const memory = useMemory();
  const battery = useBattery();
  const network = useNetwork();

  return (
    <div className="system-tray">
      <div className="tray-items">
        {memory.stats && (
          <div className="tray-item" title={`Memory: ${memory.usagePercent.toFixed(1)}%`}>
            💾 {memory.usagePercent.toFixed(0)}%
          </div>
        )}

        {battery.status && (
          <div className="tray-item" title={`Battery: ${battery.status.chargePercent.toFixed(1)}%`}>
            🔋 {battery.status.chargePercent.toFixed(0)}%
          </div>
        )}

        {network.traffic && (
          <div
            className="tray-item"
            title={`Network: ${((network.traffic.receivedPerSec * 8) / 1_000_000).toFixed(2)} Mbps`}
          >
            🌐 {((network.traffic.receivedPerSec * 8) / 1_000_000).toFixed(1)} Mbps
          </div>
        )}
      </div>
    </div>
  );
}

/**
 * Styled system provider examples
 * Include this in your component imports for complete styling
 */
export const SystemProviderStyles = `
  .battery-widget {
    padding: 12px;
    background: #1e1e1e;
    border-radius: 8px;
    color: #e0e0e0;
  }

  .battery-widget.battery-low {
    background: rgba(255, 0, 0, 0.1);
  }

  .battery-widget.battery-medium {
    background: rgba(255, 255, 0, 0.1);
  }

  .battery-widget.battery-high {
    background: rgba(0, 255, 0, 0.1);
  }

  .battery-bar {
    width: 100%;
    height: 20px;
    background: #333;
    border-radius: 4px;
    margin: 8px 0;
    overflow: hidden;
  }

  .battery-fill {
    height: 100%;
    background: linear-gradient(90deg, #ff0000, #ffff00, #00ff00);
    transition: width 0.3s ease;
  }

  .memory-widget {
    padding: 12px;
    background: #1e1e1e;
    border-radius: 8px;
    color: #e0e0e0;
  }

  .memory-bar {
    width: 100%;
    height: 20px;
    background: #333;
    border-radius: 4px;
    margin: 8px 0;
    overflow: hidden;
  }

  .memory-fill {
    height: 100%;
    background: linear-gradient(90deg, #0099ff, #ffff00);
    transition: width 0.3s ease;
  }

  .network-widget {
    padding: 12px;
    background: #1e1e1e;
    border-radius: 8px;
    color: #e0e0e0;
  }

  .traffic-item {
    display: flex;
    justify-content: space-between;
    margin: 8px 0;
  }

  .audio-widget {
    padding: 12px;
    background: #1e1e1e;
    border-radius: 8px;
    color: #e0e0e0;
  }

  .device-list {
    list-style: none;
    padding: 0;
  }

  .device-list li {
    padding: 8px;
    background: #2a2a2a;
    border-radius: 4px;
    margin: 4px 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .device-list li.default {
    background: #003366;
  }

  .system-dashboard {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 16px;
    padding: 16px;
  }

  .dashboard-card {
    background: #262626;
    border-radius: 8px;
    padding: 12px;
  }

  .quick-stats {
    display: flex;
    gap: 16px;
    padding: 8px;
    background: #1e1e1e;
    border-radius: 4px;
  }

  .stat-item {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .system-tray {
    display: flex;
    gap: 16px;
    padding: 8px;
  }

  .tray-item {
    font-size: 12px;
    padding: 4px 8px;
    background: #2a2a2a;
    border-radius: 4px;
    white-space: nowrap;
  }
`;
