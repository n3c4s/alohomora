import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/tauri';

export interface DeviceInfo {
  id: string;
  name: string;
  type: 'desktop' | 'mobile' | 'tablet';
  lastSeen: string;
  isTrusted: boolean;
  isOnline: boolean;
}

export interface SyncStatus {
  isEnabled: boolean;
  isSyncing: boolean;
  lastSyncTime: string | null;
  error: string | null;
  connectedDevices: number;
}

export interface SyncConfig {
  autoSync: boolean;
  syncInterval: number; // en minutos
  discoveryEnabled: boolean;
  allowIncomingConnections: boolean;
}

export interface SyncStats {
  totalPasswords: number;
  syncedPasswords: number;
  lastSyncDuration: number; // en segundos
  devicesCount: number;
}

interface SyncStore {
  // Estado
  status: SyncStatus;
  config: SyncConfig;
  stats: SyncStats;
  devices: DeviceInfo[];
  
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
  status: {
    isEnabled: false,
    isSyncing: false,
    lastSyncTime: null,
    error: null,
    connectedDevices: 0,
  },
  
  config: {
    autoSync: true,
    syncInterval: 15,
    discoveryEnabled: true,
    allowIncomingConnections: true,
  },
  
  stats: {
    totalPasswords: 0,
    syncedPasswords: 0,
    lastSyncDuration: 0,
    devicesCount: 0,
  },
  
  devices: [],
  
  // Acciones
  loadSyncData: async () => {
    try {
      // Cargar configuración desde el backend
      const config = await invoke('get_sync_config');
      const status = await invoke('get_sync_status');
      const devices = await invoke('get_sync_devices');
      const stats = await invoke('get_sync_stats');
      
      set({ config, status, devices, stats });
    } catch (error) {
      console.error('Error loading sync data:', error);
      set(state => ({
        status: { ...state.status, error: 'Error loading sync data' }
      }));
    }
  },
  
  toggleSync: async () => {
    try {
      const currentStatus = get().status.isEnabled;
      const newStatus = !currentStatus;
      
      if (newStatus) {
        await invoke('start_sync');
      } else {
        await invoke('stop_sync');
      }
      
      set(state => ({
        status: { ...state.status, isEnabled: newStatus }
      }));
    } catch (error) {
      console.error('Error toggling sync:', error);
      set(state => ({
        status: { ...state.status, error: 'Error toggling sync' }
      }));
    }
  },
  
  startDiscovery: async () => {
    try {
      set(state => ({
        status: { ...state.status, isSyncing: true }
      }));
      
      await invoke('start_device_discovery');
      
      // Actualizar dispositivos descubiertos
      const devices = await invoke('get_sync_devices');
      set({ devices });
      
      set(state => ({
        status: { ...state.status, isSyncing: false }
      }));
    } catch (error) {
      console.error('Error starting discovery:', error);
      set(state => ({
        status: { ...state.status, error: 'Error starting discovery', isSyncing: false }
      }));
    }
  },
  
  syncNow: async () => {
    try {
      set(state => ({
        status: { ...state.status, isSyncing: true }
      }));
      
      const startTime = Date.now();
      await invoke('sync_now');
      const duration = (Date.now() - startTime) / 1000;
      
      // Actualizar estadísticas
      const stats = await invoke('get_sync_stats');
      const status = await invoke('get_sync_status');
      
      set({ stats: { ...stats, lastSyncDuration: duration }, status });
    } catch (error) {
      console.error('Error syncing now:', error);
      set(state => ({
        status: { ...state.status, error: 'Error syncing now', isSyncing: false }
      }));
    }
  },
  
  updateConfig: async (newConfig: Partial<SyncConfig>) => {
    try {
      const updatedConfig = { ...get().config, ...newConfig };
      await invoke('update_sync_config', { config: updatedConfig });
      set({ config: updatedConfig });
    } catch (error) {
      console.error('Error updating config:', error);
      set(state => ({
        status: { ...state.status, error: 'Error updating config' }
      }));
    }
  },
  
  trustDevice: async (deviceId: string) => {
    try {
      await invoke('trust_device', { deviceId });
      
      // Actualizar estado del dispositivo
      set(state => ({
        devices: state.devices.map(device =>
          device.id === deviceId ? { ...device, isTrusted: true } : device
        )
      }));
    } catch (error) {
      console.error('Error trusting device:', error);
      set(state => ({
        status: { ...state.status, error: 'Error trusting device' }
      }));
    }
  },
  
  removeDevice: async (deviceId: string) => {
    try {
      await invoke('remove_device', { deviceId });
      
      // Remover dispositivo de la lista
      set(state => ({
        devices: state.devices.filter(device => device.id !== deviceId)
      }));
    } catch (error) {
      console.error('Error removing device:', error);
      set(state => ({
        status: { ...state.status, error: 'Error removing device' }
      }));
    }
  },
  
  clearError: () => {
    set(state => ({
      status: { ...state.status, error: null }
    }));
  },
}));
