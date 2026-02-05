import { invoke } from '@tauri-apps/api/tauri';
import { BarInfo, BarManifest, BarManagerState } from '../types/bar';

/**
 * Bar manager for discovering and selecting custom status bars
 */
export class BarManager {
  private availableBars: BarInfo[] = [];
  private selectedBar: BarInfo | null = null;
  private isLoading = false;
  private error: string | null = null;
  private eventListeners = new Map<
    'bars:discovered' | 'bar:selected',
    Set<(data: any) => void>
  >();

  async discoverBars(): Promise<BarInfo[]> {
    try {
      this.isLoading = true;
      this.error = null;

      const result = await invoke<any>('discover_available_bars');

      if (result.success) {
        this.availableBars = result.data;
        this.emit('bars:discovered', this.availableBars);
        return this.availableBars;
      } else {
        throw new Error(result.error || 'Unknown error');
      }
    } catch (err) {
      this.error = String(err);
      throw err;
    } finally {
      this.isLoading = false;
    }
  }

  async selectBar(barId: string): Promise<void> {
    try {
      this.isLoading = true;
      this.error = null;

      const result = await invoke<any>('select_bar', { barId });

      if (result.success) {
        this.selectedBar = this.availableBars.find(b => b.id === barId) || null;
        this.emit('bar:selected', this.selectedBar);
      } else {
        throw new Error(result.error || 'Unknown error');
      }
    } catch (err) {
      this.error = String(err);
      throw err;
    } finally {
      this.isLoading = false;
    }
  }

  async getSelectedBar(): Promise<BarInfo | null> {
    try {
      const result = await invoke<any>('get_selected_bar');

      if (result.success) {
        this.selectedBar = result.data;
        return this.selectedBar;
      } else {
        throw new Error(result.error || 'Unknown error');
      }
    } catch (err) {
      this.error = String(err);
      return null;
    }
  }

  async getBarManifest(barId: string): Promise<BarManifest | null> {
    try {
      const result = await invoke<any>('get_bar_manifest', { barId });

      if (result.success) {
        return result.data;
      } else {
        throw new Error(result.error || 'Unknown error');
      }
    } catch (err) {
      this.error = String(err);
      return null;
    }
  }

  async createBarScaffold(barName: string): Promise<string> {
    try {
      this.isLoading = true;
      this.error = null;

      const result = await invoke<any>('create_bar_scaffold', { barName });

      if (result.success) {
        // Refresh available bars
        await this.discoverBars();
        return result.data;
      } else {
        throw new Error(result.error || 'Unknown error');
      }
    } catch (err) {
      this.error = String(err);
      throw err;
    } finally {
      this.isLoading = false;
    }
  }

  getState(): BarManagerState {
    return {
      availableBars: this.availableBars,
      selectedBar: this.selectedBar,
      isLoading: this.isLoading,
      error: this.error,
    };
  }

  on(event: 'bars:discovered' | 'bar:selected', listener: (data: any) => void): () => void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, new Set());
    }

    this.eventListeners.get(event)!.add(listener);

    return () => {
      this.eventListeners.get(event)?.delete(listener);
    };
  }

  private emit(event: 'bars:discovered' | 'bar:selected', data: any) {
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
}

// Singleton instance
let barManagerInstance: BarManager | null = null;

export function getBarManager(): BarManager {
  if (!barManagerInstance) {
    barManagerInstance = new BarManager();
  }
  return barManagerInstance;
}

export function createBarManager(): BarManager {
  return new BarManager();
}
