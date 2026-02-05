import { useEffect, useState } from 'react';
import { BarInfo, BarManagerState } from '../types/bar';
import { getBarManager } from '../providers/bar';

/**
 * Hook for bar management
 */
export function useBarManager(): BarManagerState & {
  discoverBars: () => Promise<BarInfo[]>;
  selectBar: (barId: string) => Promise<void>;
  getSelectedBar: () => Promise<BarInfo | null>;
  createBar: (barName: string) => Promise<string>;
} {
  const manager = getBarManager();
  const [state, setState] = useState<BarManagerState>(manager.getState());

  useEffect(() => {
    const unsubscribe = manager.on('bars:discovered', () => {
      setState(manager.getState());
    });

    const unsubscribe2 = manager.on('bar:selected', () => {
      setState(manager.getState());
    });

    return () => {
      unsubscribe();
      unsubscribe2();
    };
  }, [manager]);

  return {
    ...state,
    discoverBars: () => manager.discoverBars(),
    selectBar: (barId: string) => manager.selectBar(barId),
    getSelectedBar: () => manager.getSelectedBar(),
    createBar: (barName: string) => manager.createBarScaffold(barName),
  };
}

/**
 * Hook for available bars list
 */
export function useAvailableBars(): {
  bars: BarInfo[];
  isLoading: boolean;
  error: string | null;
  refresh: () => Promise<void>;
} {
  const manager = getBarManager();
  const [bars, setBars] = useState<BarInfo[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = async () => {
    try {
      setIsLoading(true);
      await manager.discoverBars();
      setBars(manager.getState().availableBars);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    refresh();
  }, []);

  useEffect(() => {
    const unsubscribe = manager.on('bars:discovered', (bars) => {
      setBars(bars);
    });

    return unsubscribe;
  }, [manager]);

  return { bars, isLoading, error, refresh };
}

/**
 * Hook for selected bar
 */
export function useSelectedBar(): {
  bar: BarInfo | null;
  isLoading: boolean;
  selectBar: (barId: string) => Promise<void>;
} {
  const manager = getBarManager();
  const [bar, setBar] = useState<BarInfo | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    const loadSelected = async () => {
      try {
        setIsLoading(true);
        await manager.getSelectedBar();
        setBar(manager.getState().selectedBar);
      } finally {
        setIsLoading(false);
      }
    };

    loadSelected();
  }, [manager]);

  useEffect(() => {
    const unsubscribe = manager.on('bar:selected', (selectedBar) => {
      setBar(selectedBar);
    });

    return unsubscribe;
  }, [manager]);

  return {
    bar,
    isLoading,
    selectBar: (barId: string) => manager.selectBar(barId),
  };
}
