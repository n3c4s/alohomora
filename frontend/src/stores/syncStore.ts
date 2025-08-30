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
      console.log('🔄 Cargando datos de sincronización...');
      
      // Cargar configuración desde el backend
      const config = await invoke('get_sync_config');
      const status = await invoke('get_sync_status');
      const devices = await invoke('get_sync_devices');
      const stats = await invoke('get_sync_stats');
      
      console.log('✅ Datos cargados:', { config, status, devices, stats });
      
      set({ config, status, devices, stats });
    } catch (error) {
      console.error('❌ Error loading sync data:', error);
      set(state => ({
        status: { ...state.status, error: 'Error loading sync data' }
      }));
    }
  },
  
  toggleSync: async () => {
    try {
      const currentStatus = get().status.isEnabled;
      const newStatus = !currentStatus;
      
      console.log(`🔄 Cambiando estado de sincronización: ${currentStatus} -> ${newStatus}`);
      
      if (newStatus) {
        // Activar sincronización
        await invoke('start_sync');
        set(state => ({
          status: { 
            ...state.status, 
            isEnabled: true, 
            error: null,
            connectedDevices: 1 // Simular dispositivo local
          }
        }));
        console.log('✅ Sincronización activada');
      } else {
        // Desactivar sincronización
        await invoke('stop_sync');
        set(state => ({
          status: { 
            ...state.status, 
            isEnabled: false, 
            error: null,
            connectedDevices: 0
          }
        }));
        console.log('❌ Sincronización desactivada');
      }
    } catch (error) {
      console.error('❌ Error toggling sync:', error);
      set(state => ({
        status: { ...state.status, error: 'Error toggling sync' }
      }));
    }
  },
  
  startDiscovery: async () => {
    try {
      console.log('🔍 Iniciando descubrimiento de dispositivos...');
      await invoke('start_device_discovery');
      
      // Simular dispositivos encontrados
      const mockDevices: DeviceInfo[] = [
        {
          id: 'local-device',
          name: 'MacBook Pro de Charly',
          type: 'desktop',
          lastSeen: new Date().toISOString(),
          isTrusted: true,
          isOnline: true,
        }
      ];
      
      set(state => ({
        devices: mockDevices,
        status: { 
          ...state.status, 
          connectedDevices: mockDevices.length 
        }
      }));
      
      console.log('✅ Descubrimiento iniciado, dispositivos encontrados:', mockDevices.length);
    } catch (error) {
      console.error('❌ Error starting discovery:', error);
      set(state => ({
        status: { ...state.status, error: 'Error starting discovery' }
      }));
    }
  },
  
  syncNow: async () => {
    try {
      console.log('🔄 Iniciando sincronización manual...');
      
      set(state => ({
        status: { ...state.status, isSyncing: true, error: null }
      }));
      
      await invoke('sync_now');
      
      // Simular sincronización exitosa
      setTimeout(() => {
        set(state => ({
          status: { 
            ...state.status, 
            isSyncing: false, 
            lastSyncTime: new Date().toISOString(),
            error: null
          }
        }));
        console.log('✅ Sincronización completada');
      }, 2000);
      
    } catch (error) {
      console.error('❌ Error syncing now:', error);
      set(state => ({
        status: { ...state.status, isSyncing: false, error: 'Error syncing now' }
      }));
    }
  },
  
  updateConfig: async (newConfig: Partial<SyncConfig>) => {
    try {
      console.log('⚙️ Actualizando configuración:', newConfig);
      await invoke('update_sync_config', { config: newConfig });
      
      set(state => ({
        config: { ...state.config, ...newConfig }
      }));
      
      console.log('✅ Configuración actualizada');
    } catch (error) {
      console.error('❌ Error updating config:', error);
      set(state => ({
        status: { ...state.status, error: 'Error updating config' }
      }));
    }
  },
  
  trustDevice: async (deviceId: string) => {
    try {
      console.log('🤝 Confiando dispositivo:', deviceId);
      await invoke('trust_device', { deviceId });
      
      set(state => ({
        devices: state.devices.map(device => 
          device.id === deviceId 
            ? { ...device, isTrusted: true }
            : device
        )
      }));
      
      console.log('✅ Dispositivo confiado');
    } catch (error) {
      console.error('❌ Error trusting device:', error);
      set(state => ({
        status: { ...state.status, error: 'Error trusting device' }
      }));
    }
  },
  
  removeDevice: async (deviceId: string) => {
    try {
      console.log('🗑️ Removiendo dispositivo:', deviceId);
      await invoke('remove_device', { deviceId });
      
      set(state => ({
        devices: state.devices.filter(device => device.id !== deviceId),
        status: { 
          ...state.status, 
          connectedDevices: Math.max(0, state.status.connectedDevices - 1)
        }
      }));
      
      console.log('✅ Dispositivo removido');
    } catch (error) {
      console.error('❌ Error removing device:', error);
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
