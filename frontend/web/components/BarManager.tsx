import React, { useState } from 'react';
import { useAvailableBars, useSelectedBar, useBarManager } from '../hooks/useBarManager';
import { BarInfo } from '../types/bar';

/**
 * Bar Selector Component
 * Shows available bars and allows selection
 */
export function BarSelector() {
  const { bars, isLoading, error } = useAvailableBars();
  const { bar: selectedBar, selectBar } = useSelectedBar();

  const handleSelect = async (barId: string) => {
    try {
      await selectBar(barId);
    } catch (err) {
      console.error('Failed to select bar:', err);
    }
  };

  return (
    <div className="bar-selector">
      <label>Status Bar</label>
      <select
        value={selectedBar?.id || ''}
        onChange={e => handleSelect(e.target.value)}
        disabled={isLoading}
      >
        <option value="">Choose a bar...</option>
        {bars.map(bar => (
          <option key={bar.id} value={bar.id}>
            {bar.name} {bar.active ? '✓' : ''}
          </option>
        ))}
      </select>
      {error && <div className="error">{error}</div>}
      {selectedBar && (
        <div className="bar-info">
          <p>
            <strong>{selectedBar.name}</strong> v{selectedBar.version}
          </p>
          <p>{selectedBar.description}</p>
        </div>
      )}
    </div>
  );
}

/**
 * Bars List Component
 * Shows all available bars with details
 */
export function BarsList() {
  const { bars, isLoading, error, refresh } = useAvailableBars();
  const { selectBar } = useSelectedBar();

  const handleSelect = async (bar: BarInfo) => {
    try {
      await selectBar(bar.id);
    } catch (err) {
      console.error('Failed to select bar:', err);
    }
  };

  return (
    <div className="bars-list">
      <div className="bars-header">
        <h3>Available Status Bars</h3>
        <button onClick={refresh} disabled={isLoading}>
          {isLoading ? 'Discovering...' : 'Refresh'}
        </button>
      </div>

      {error && <div className="error">{error}</div>}

      {isLoading && <p>Discovering bars...</p>}

      {bars.length === 0 && !isLoading && (
        <p className="empty">
          No bars found. Create one with "Create New Bar" or place a bar in ~/.unkai/bars/
        </p>
      )}

      <div className="bars-grid">
        {bars.map(bar => (
          <div
            key={bar.id}
            className={`bar-card ${bar.active ? 'active' : ''}`}
            onClick={() => handleSelect(bar)}
          >
            <h4>{bar.name}</h4>
            <p className="version">v{bar.version}</p>
            <p className="description">{bar.description}</p>
            <p className="id">{bar.id}</p>
            {bar.active && <span className="badge">Active</span>}
          </div>
        ))}
      </div>
    </div>
  );
}

/**
 * Create Bar Form Component
 * Allows creating a new bar scaffold
 */
export function CreateBarForm() {
  const [barName, setBarName] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const { createBar } = useBarManager();
  const { refresh } = useAvailableBars();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setSuccess(null);

    if (!barName.trim()) {
      setError('Please enter a bar name');
      return;
    }

    try {
      setIsCreating(true);
      const result = await createBar(barName);
      setSuccess(result);
      setBarName('');
      await refresh();
    } catch (err) {
      setError(String(err));
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <div className="create-bar-form">
      <h3>Create New Bar</h3>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Bar name (e.g., My Status Bar)"
          value={barName}
          onChange={e => setBarName(e.target.value)}
          disabled={isCreating}
        />
        <button type="submit" disabled={isCreating}>
          {isCreating ? 'Creating...' : 'Create'}
        </button>
      </form>

      {error && <div className="error">{error}</div>}
      {success && <div className="success">{success}</div>}

      <div className="help-text">
        <p>
          A new React + TypeScript bar will be created at <code>~/.unkai/bars/[bar-name]/</code>
        </p>
        <p>Edit the bar files and run <code>npm build</code> to compile</p>
      </div>
    </div>
  );
}

/**
 * Bar Settings Panel Component
 * Complete bar management UI
 */
export function BarSettingsPanel() {
  const [activeTab, setActiveTab] = useState<'select' | 'list' | 'create'>('select');

  return (
    <div className="bar-settings-panel">
      <div className="tabs">
        <button
          className={`tab ${activeTab === 'select' ? 'active' : ''}`}
          onClick={() => setActiveTab('select')}
        >
          Select Bar
        </button>
        <button
          className={`tab ${activeTab === 'list' ? 'active' : ''}`}
          onClick={() => setActiveTab('list')}
        >
          All Bars
        </button>
        <button
          className={`tab ${activeTab === 'create' ? 'active' : ''}`}
          onClick={() => setActiveTab('create')}
        >
          Create Bar
        </button>
      </div>

      <div className="tab-content">
        {activeTab === 'select' && <BarSelector />}
        {activeTab === 'list' && <BarsList />}
        {activeTab === 'create' && <CreateBarForm />}
      </div>

      <style>{`
        .bar-settings-panel {
          padding: 16px;
          background: #1e1e1e;
          border-radius: 8px;
          color: #e0e0e0;
        }

        .tabs {
          display: flex;
          gap: 8px;
          margin-bottom: 16px;
          border-bottom: 1px solid #333;
        }

        .tab {
          padding: 8px 16px;
          background: transparent;
          border: none;
          color: #999;
          cursor: pointer;
          border-bottom: 2px solid transparent;
          transition: all 0.2s;
        }

        .tab.active {
          color: #0099ff;
          border-bottom-color: #0099ff;
        }

        .tab:hover:not(.active) {
          color: #ccc;
        }

        .tab-content {
          margin-top: 16px;
        }

        .bar-selector {
          display: flex;
          flex-direction: column;
          gap: 12px;
        }

        .bar-selector label {
          font-weight: 600;
        }

        .bar-selector select {
          padding: 8px 12px;
          background: #2a2a2a;
          border: 1px solid #444;
          border-radius: 4px;
          color: #e0e0e0;
          cursor: pointer;
        }

        .bar-info {
          padding: 12px;
          background: #262626;
          border-radius: 4px;
          border-left: 3px solid #0099ff;
        }

        .bar-info p {
          margin: 4px 0;
          font-size: 13px;
        }

        .bar-info strong {
          color: #0099ff;
        }

        .bars-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 16px;
        }

        .bars-header button {
          padding: 6px 12px;
          background: #0099ff;
          border: none;
          border-radius: 4px;
          color: white;
          cursor: pointer;
          font-size: 12px;
        }

        .bars-header button:hover:not(:disabled) {
          background: #0077cc;
        }

        .bars-header button:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }

        .bars-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
          gap: 12px;
        }

        .bar-card {
          padding: 12px;
          background: #2a2a2a;
          border: 1px solid #444;
          border-radius: 4px;
          cursor: pointer;
          transition: all 0.2s;
          position: relative;
        }

        .bar-card:hover {
          border-color: #0099ff;
          background: #323232;
        }

        .bar-card.active {
          border-color: #0099ff;
          background: #003366;
        }

        .bar-card h4 {
          margin: 0 0 4px 0;
          color: #0099ff;
        }

        .bar-card .version {
          font-size: 12px;
          color: #888;
          margin: 0;
        }

        .bar-card .description {
          font-size: 12px;
          color: #bbb;
          margin: 8px 0;
        }

        .bar-card .id {
          font-size: 11px;
          color: #666;
          font-family: monospace;
          margin: 4px 0 0 0;
        }

        .bar-card .badge {
          position: absolute;
          top: 8px;
          right: 8px;
          background: #0099ff;
          color: white;
          padding: 2px 8px;
          border-radius: 3px;
          font-size: 11px;
        }

        .create-bar-form {
          max-width: 400px;
        }

        .create-bar-form form {
          display: flex;
          gap: 8px;
          margin: 16px 0;
        }

        .create-bar-form input {
          flex: 1;
          padding: 8px 12px;
          background: #2a2a2a;
          border: 1px solid #444;
          border-radius: 4px;
          color: #e0e0e0;
        }

        .create-bar-form button {
          padding: 8px 16px;
          background: #0099ff;
          border: none;
          border-radius: 4px;
          color: white;
          cursor: pointer;
        }

        .create-bar-form button:hover:not(:disabled) {
          background: #0077cc;
        }

        .create-bar-form button:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }

        .help-text {
          margin-top: 16px;
          padding: 12px;
          background: #262626;
          border-radius: 4px;
          border-left: 3px solid #666;
          font-size: 12px;
        }

        .help-text p {
          margin: 4px 0;
        }

        .help-text code {
          background: #1a1a1a;
          padding: 2px 6px;
          border-radius: 2px;
          color: #0099ff;
          font-family: monospace;
        }

        .error {
          padding: 8px 12px;
          background: rgba(255, 0, 0, 0.1);
          border-left: 3px solid #ff0000;
          color: #ff6b6b;
          border-radius: 4px;
          font-size: 12px;
        }

        .success {
          padding: 8px 12px;
          background: rgba(0, 255, 0, 0.1);
          border-left: 3px solid #00ff00;
          color: #6bff6b;
          border-radius: 4px;
          font-size: 12px;
        }

        .empty {
          padding: 32px;
          text-align: center;
          color: #888;
        }
      `}</style>
    </div>
  );
}

/**
 * Bar List Quick Component
 * Minimal bar selector for tray menu
 */
export function QuickBarSelector() {
  const { bars } = useAvailableBars();
  const { bar: selectedBar, selectBar } = useSelectedBar();

  return (
    <div className="quick-bar-selector">
      {bars.map(bar => (
        <button
          key={bar.id}
          className={`bar-option ${bar.active ? 'active' : ''}`}
          onClick={() => selectBar(bar.id)}
        >
          {bar.active && <span>✓ </span>}
          {bar.name}
        </button>
      ))}
    </div>
  );
}
