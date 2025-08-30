import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';

export interface DeviceInfo {
  id: string;
  name: string;
  device_type: string;
  os: string;
  os_version: string;
  app_version: string;
  status: string;
  last_seen: string;
  is_trusted: boolean;
  ip_address?: string;
  port?: number;
}

export interface SyncStatus {
  is_enabled: boolean;
  connected_devices: DeviceInfo[];
  last_sync: string | null;
  sync_method: string;
  auto_sync: boolean;
}

export interface SyncConfig {
  auto_discovery: boolean;
  auto_sync: boolean;
  sync_interval: number;
  max_devices: number;
  encryption_level: string;
  allowed_networks: string[];
}

export interface SyncStats {
  total_syncs: number;
  successful_syncs: number;
  failed_syncs: number;
  total_data_synced: number;
  last_sync_duration?: number;
  devices_synced_with: string[];
}

interface SyncStore {
  // Estado
  syncStatus: SyncStatus | null;
  syncConfig: SyncConfig | null;
  discoveredDevices: DeviceInfo[];
  syncStats: SyncStats | null;
  isLoading: boolean;
  error: string | null;

  // Acciones
  loadSyncData: () => Promise<void>;
  toggleSync: () => Promise<void>;
  startDiscovery: () => Promise<void>;
  syncNow: () => Promise<void>;
  updateConfig: (config: Partial<SyncConfig>) => Promise<void>;
  trustDevice: (deviceId: string) => Promise<void>;
  removeDevice: (deviceId: string) => Promise<void>;
  clearError: () => void;
}

export const useSyncStore = create<SyncStore>((set, get) => ({
  // Estado inicial
  syncStatus: null,
  syncConfig: null,
  discoveredDevices: [],
  syncStats: null,
  isLoading: false,
  error: null,

  // Cargar datos de sincronización
  loadSyncData: async () => {
    try {
      set({ isLoading: true, error: null });
      
      // TODO: Implementar llamadas reales a Tauri
      // Por ahora usamos datos de ejemplo
      const mockStatus: SyncStatus = {
        is_enabled: true,
        connected_devices: [
          {
            id: '1',
            name: 'MacBook-Pro-de-Charly',
            device_type: 'Laptop',
            os: 'macOS',
            os_version: '14.0',
            app_version: '1.0.0',
            status: 'Connected',
            last_seen: new Date().toISOString(),
            is_trusted: true
          }
        ],
        last_sync: new Date(Date.now() - 300000).toISOString(),
        sync_method: 'Hybrid',
        auto_sync: true
      };

      const mockConfig: SyncConfig = {
        auto_discovery: true,
        auto_sync: true,
        sync_interval: 300,
        max_devices: 10,
        encryption_level: 'Military',
        allowed_networks: ['WiFi Casa', 'WiFi Trabajo']
      };

      const mockDiscovered: DeviceInfo[] = [
        {
          id: '2',
          name: 'iPhone-Charly',
          device_type: 'Mobile',
          os: 'iOS',
          os_version: '17.0',
          app_version: '1.0.0',
          status: 'Discovered',
          last_seen: new Date(Date.now() - 60000).toISOString(),
          is_trusted: false
        },
        {
          id: '3',
          name: 'PC-Windows',
          device_type: 'Desktop',
          os: 'Windows',
          os_version: '11',
          app_version: '1.0.0',
          status: 'Discovered',
          last_seen: new Date(Date.now() - 120000).toISOString(),
          is_trusted: false
        }
      ];

      const mockStats: SyncStats = {
        total_syncs: 156,
        successful_syncs: 148,
        failed_syncs: 8,
        total_data_synced: 2048576, // 2MB
        last_sync_duration: 1250,
        devices_synced_with: ['MacBook-Pro-de-Charly', 'iPhone-Charly']
      };

      set({
        syncStatus: mockStatus,
        syncConfig: mockConfig,
        discoveredDevices: mockDiscovered,
        syncStats: mockStats,
        isLoading: false
      });
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Error desconocido',
        isLoading: false 
      });
    }
  },

  // Alternar sincronización
  toggleSync: async () => {
    try {
      const { syncStatus } = get();
      if (!syncStatus) return;

      // TODO: Implementar llamada a Tauri
      const newStatus = !syncStatus.is_enabled;
      
      set({
        syncStatus: {
          ...syncStatus,
          is_enabled: newStatus
        }
      });

      console.log(`Sincronización ${newStatus ? 'activada' : 'desactivada'}`);
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Error al alternar sincronización'
      });
    }
  },

  // Iniciar descubrimiento
  startDiscovery: async () => {
    try {
      set({ error: null });
      
      // TODO: Implementar llamada a Tauri
      console.log('Iniciando descubrimiento de dispositivos...');
      
      // Simular descubrimiento
      setTimeout(() => {
        get().loadSyncData();
      }, 2000);
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Error al iniciar descubrimiento'
      });
    }
  },

  // Sincronizar ahora
  syncNow: async () => {
    try {
      set({ error: null });
      
      // TODO: Implementar llamada a Tauri
      console.log('Iniciando sincronización manual...');
      
      // Simular sincronización
      setTimeout(() => {
        get().loadSyncData();
      }, 1500);
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Error al sincronizar'
      });
    }
  },

  // Actualizar configuración
  updateConfig: async (config: Partial<SyncConfig>) => {
    try {
      const { syncConfig } = get();
      if (!syncConfig) return;

      // TODO: Implementar llamada a Tauri
      const newConfig = { ...syncConfig, ...config };
      
      set({ syncConfig: newConfig });
      console.log('Configuración actualizada:', newConfig);
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Error al actualizar configuración'
      });
    }
  },

  // Confiar en dispositivo
  trustDevice: async (deviceId: string) => {
    try {
      const { discoveredDevices } = get();
      
      // TODO: Implementar llamada a Tauri
      const updatedDevices = discoveredDevices.map(device =>
        device.id === deviceId 
          ? { ...device, is_trusted: true, status: 'Connected' }
          : device
      );
      
      set({ discoveredDevices: updatedDevices });
      console.log(`Dispositivo ${deviceId} marcado como confiable`);
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Error al confiar en dispositivo'
      });
    }
  },

  // Eliminar dispositivo
  removeDevice: async (deviceId: string) => {
    try {
      const { discoveredDevices } = get();
      
      // TODO: Implementar llamada a Tauri
      const updatedDevices = discoveredDevices.filter(device => device.id !== deviceId);
      
      set({ discoveredDevices: updatedDevices });
      console.log(`Dispositivo ${deviceId} eliminado`);
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Error al eliminar dispositivo'
      });
    }
  },

  // Limpiar error
  clearError: () => set({ error: null })
}));
