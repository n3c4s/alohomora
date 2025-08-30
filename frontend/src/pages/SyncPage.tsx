import React, { useState, useEffect } from 'react';
import { 
  Wifi, 
  WifiOff, 
  RefreshCw, 
  Settings, 
  Users, 
  Shield, 
  Clock,
  CheckCircle,
  XCircle,
  AlertTriangle,
  Smartphone,
  Monitor,
  Laptop,
  Tablet,
  Server
} from 'lucide-react';
import { useSyncStore, DeviceInfo, SyncStatus, SyncConfig } from '../stores/syncStore';

const SyncPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'overview' | 'devices' | 'settings'>('overview');
  
  const {
    syncStatus,
    syncConfig,
    discoveredDevices,
    syncStats,
    isLoading,
    error,
    loadSyncData,
    toggleSync,
    startDiscovery,
    syncNow,
    updateConfig,
    trustDevice,
    removeDevice,
    clearError
  } = useSyncStore();

  useEffect(() => {
    loadSyncData();
    const interval = setInterval(loadSyncData, 10000); // Actualizar cada 10 segundos
    return () => clearInterval(interval);
  }, [loadSyncData]);

  const getDeviceTypeIcon = (deviceType: string) => {
    switch (deviceType.toLowerCase()) {
      case 'mobile':
        return <Smartphone className="w-5 h-5" />;
      case 'desktop':
        return <Monitor className="w-5 h-5" />;
      case 'laptop':
        return <Laptop className="w-5 h-5" />;
      case 'tablet':
        return <Tablet className="w-5 h-5" />;
      case 'server':
        return <Server className="w-5 h-5" />;
      default:
        return <Monitor className="w-5 h-5" />;
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status.toLowerCase()) {
      case 'connected':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'discovered':
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
      case 'error':
        return <XCircle className="w-4 h-4 text-red-500" />;
      default:
        return <Clock className="w-4 h-4 text-gray-500" />;
    }
  };

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 max-w-md">
          <div className="flex items-center space-x-3 mb-4">
            <XCircle className="w-8 h-8 text-red-500" />
            <h2 className="text-xl font-semibold text-gray-900">Error</h2>
          </div>
          <p className="text-gray-600 mb-4">{error}</p>
          <div className="flex space-x-3">
            <button
              onClick={clearError}
              className="bg-gray-100 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-200"
            >
              Cerrar
            </button>
            <button
              onClick={loadSyncData}
              className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
            >
              Reintentar
            </button>
          </div>
        </div>
      </div>
    );
  }

  if (!syncStatus || !syncConfig) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <RefreshCw className="w-12 h-12 text-gray-400 mx-auto mb-4 animate-spin" />
          <p className="text-gray-500">Cargando configuración de sincronización...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Sincronización</h1>
          <p className="mt-2 text-gray-600 dark:text-gray-400">
            Gestiona la sincronización automática de contraseñas entre dispositivos
          </p>
        </div>

        {/* Status Card */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              {syncStatus.is_enabled ? (
                <Wifi className="w-8 h-8 text-green-500" />
              ) : (
                <WifiOff className="w-8 h-8 text-red-500" />
              )}
              <div>
                <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
                  Estado de Sincronización
                </h2>
                <p className="text-gray-600 dark:text-gray-400">
                  {syncStatus.is_enabled ? 'Activa' : 'Inactiva'}
                </p>
              </div>
            </div>
            <div className="flex space-x-3">
              <button
                onClick={toggleSync}
                className={`px-4 py-2 rounded-md font-medium ${
                  syncStatus.is_enabled
                    ? 'bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-900 dark:text-red-200 dark:hover:bg-red-800'
                    : 'bg-green-100 text-green-700 hover:bg-green-200 dark:bg-green-900 dark:text-green-200 dark:hover:bg-green-800'
                }`}
              >
                {syncStatus.is_enabled ? 'Desactivar' : 'Activar'}
              </button>
              <button
                onClick={syncNow}
                className="bg-blue-100 text-blue-700 px-4 py-2 rounded-md font-medium hover:bg-blue-200 dark:bg-blue-900 dark:text-blue-200 dark:hover:bg-blue-800"
              >
                Sincronizar Ahora
              </button>
            </div>
          </div>
        </div>

        {/* Tabs */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
          <div className="border-b border-gray-200 dark:border-gray-700">
            <nav className="-mb-px flex space-x-8 px-6">
              {[
                { id: 'overview', name: 'Resumen', icon: RefreshCw },
                { id: 'devices', name: 'Dispositivos', icon: Users },
                { id: 'settings', name: 'Configuración', icon: Settings }
              ].map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id as any)}
                  className={`py-4 px-1 border-b-2 font-medium text-sm ${
                    activeTab === tab.id
                      ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300'
                  }`}
                >
                  <tab.icon className="w-5 h-5 inline mr-2" />
                  {tab.name}
                </button>
              ))}
            </nav>
          </div>

          {/* Tab Content */}
          <div className="p-6">
            {activeTab === 'overview' && (
              <div className="space-y-6">
                {/* Stats Grid */}
                <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
                  <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
                    <div className="flex items-center">
                      <Users className="w-8 h-8 text-blue-600 dark:text-blue-400" />
                      <div className="ml-3">
                        <p className="text-sm font-medium text-blue-600 dark:text-blue-400">Dispositivos Conectados</p>
                        <p className="text-2xl font-bold text-blue-900 dark:text-blue-100">
                          {syncStatus.connected_devices.length}
                        </p>
                      </div>
                    </div>
                  </div>
                  
                  <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
                    <div className="flex items-center">
                      <RefreshCw className="w-8 h-8 text-green-600 dark:text-green-400" />
                      <div className="ml-3">
                        <p className="text-sm font-medium text-green-600 dark:text-green-400">Última Sincronización</p>
                        <p className="text-sm font-bold text-green-900 dark:text-green-100">
                          {syncStatus.last_sync 
                            ? new Date(syncStatus.last_sync).toLocaleTimeString()
                            : 'Nunca'
                          }
                        </p>
                      </div>
                    </div>
                  </div>
                  
                  <div className="bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg">
                    <div className="flex items-center">
                      <Shield className="w-8 h-8 text-purple-600 dark:text-purple-400" />
                      <div className="ml-3">
                        <p className="text-sm font-medium text-purple-600 dark:text-purple-400">Método de Sincronización</p>
                        <p className="text-sm font-bold text-purple-900 dark:text-purple-100">
                          {syncStatus.sync_method}
                        </p>
                      </div>
                    </div>
                  </div>
                  
                  <div className="bg-orange-50 dark:bg-orange-900/20 p-4 rounded-lg">
                    <div className="flex items-center">
                      <Clock className="w-8 h-8 text-orange-600 dark:text-orange-400" />
                      <div className="ml-3">
                        <p className="text-sm font-medium text-orange-600 dark:text-orange-400">Intervalo de Sincronización</p>
                        <p className="text-sm font-bold text-orange-900 dark:text-orange-100">
                          {syncConfig.sync_interval ? `${syncConfig.sync_interval / 60} min` : 'N/A'}
                        </p>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Sync Stats */}
                {syncStats && (
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div className="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
                      <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Total de Sincronizaciones</h3>
                      <p className="text-2xl font-bold text-gray-900 dark:text-white">{syncStats.total_syncs}</p>
                      <p className="text-sm text-gray-500 dark:text-gray-400">
                        {syncStats.successful_syncs} exitosas, {syncStats.failed_syncs} fallidas
                      </p>
                    </div>
                    
                    <div className="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
                      <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Datos Sincronizados</h3>
                      <p className="text-2xl font-bold text-gray-900 dark:text-white">{formatBytes(syncStats.total_data_synced)}</p>
                      <p className="text-sm text-gray-500 dark:text-gray-400">Total acumulado</p>
                    </div>
                    
                    <div className="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
                      <h3 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Última Duración</h3>
                      <p className="text-2xl font-bold text-gray-900 dark:text-white">
                        {syncStats.last_sync_duration ? `${syncStats.last_sync_duration}ms` : 'N/A'}
                      </p>
                      <p className="text-sm text-gray-500 dark:text-gray-400">Tiempo de sincronización</p>
                    </div>
                  </div>
                )}

                {/* Recent Activity */}
                <div>
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Actividad Reciente</h3>
                  <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4">
                    <p className="text-gray-600 dark:text-gray-300 text-sm">
                      Sistema de sincronización funcionando correctamente. 
                      {syncStatus.auto_sync ? ' Sincronización automática activada.' : ' Sincronización manual.'}
                    </p>
                  </div>
                </div>
              </div>
            )}

            {activeTab === 'devices' && (
              <div className="space-y-6">
                <div className="flex justify-between items-center">
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white">Dispositivos Descubiertos</h3>
                  <button
                    onClick={startDiscovery}
                    className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600"
                  >
                    Buscar Dispositivos
                  </button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {discoveredDevices.map((device) => (
                    <div key={device.id} className="bg-gray-50 dark:bg-gray-700 rounded-lg p-4 border border-gray-200 dark:border-gray-600">
                      <div className="flex items-start justify-between">
                        <div className="flex items-center space-x-3">
                          {getDeviceTypeIcon(device.device_type)}
                          <div>
                            <h4 className="font-medium text-gray-900 dark:text-white">{device.name}</h4>
                            <p className="text-sm text-gray-600 dark:text-gray-400">{device.os} {device.os_version}</p>
                            <p className="text-xs text-gray-500 dark:text-gray-500">v{device.app_version}</p>
                          </div>
                        </div>
                        <div className="flex items-center space-x-2">
                          {getStatusIcon(device.status)}
                          <span className="text-xs text-gray-500 dark:text-gray-400">
                            {new Date(device.last_seen).toLocaleTimeString()}
                          </span>
                        </div>
                      </div>
                      
                      <div className="mt-3 flex items-center justify-between">
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                          device.is_trusted 
                            ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' 
                            : 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'
                        }`}>
                          {device.is_trusted ? 'Confiable' : 'No Verificado'}
                        </span>
                        
                        <div className="flex space-x-2">
                          {!device.is_trusted && (
                            <button 
                              onClick={() => trustDevice(device.id)}
                              className="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 text-sm font-medium"
                            >
                              Verificar
                            </button>
                          )}
                          <button 
                            onClick={() => removeDevice(device.id)}
                            className="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 text-sm font-medium"
                          >
                            Eliminar
                          </button>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>

                {discoveredDevices.length === 0 && (
                  <div className="text-center py-8">
                    <Users className="w-12 h-12 text-gray-400 dark:text-gray-500 mx-auto mb-4" />
                    <p className="text-gray-500 dark:text-gray-400">No se han descubierto dispositivos</p>
                    <button
                      onClick={startDiscovery}
                      className="mt-2 text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 font-medium"
                    >
                      Buscar dispositivos en la red
                    </button>
                  </div>
                )}
              </div>
            )}

            {activeTab === 'settings' && (
              <div className="space-y-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white">Configuración de Sincronización</h3>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  {/* General Settings */}
                  <div className="space-y-4">
                    <h4 className="font-medium text-gray-900 dark:text-white">Configuración General</h4>
                    
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                          Descubrimiento Automático
                        </label>
                        <p className="text-xs text-gray-500 dark:text-gray-400">
                          Buscar dispositivos automáticamente en la red
                        </p>
                      </div>
                      <button
                        onClick={() => updateConfig({ auto_discovery: !syncConfig.auto_discovery })}
                        className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                          syncConfig.auto_discovery ? 'bg-blue-600' : 'bg-gray-200 dark:bg-gray-600'
                        }`}
                      >
                        <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                          syncConfig.auto_discovery ? 'translate-x-6' : 'translate-x-1'
                        }`} />
                      </button>
                    </div>

                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                          Sincronización Automática
                        </label>
                        <p className="text-xs text-gray-500 dark:text-gray-400">
                          Sincronizar automáticamente cuando se detecten cambios
                        </p>
                      </div>
                      <button
                        onClick={() => updateConfig({ auto_sync: !syncConfig.auto_sync })}
                        className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                          syncConfig.auto_sync ? 'bg-blue-600' : 'bg-gray-200 dark:bg-gray-600'
                        }`}
                      >
                        <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                          syncConfig.auto_sync ? 'translate-x-6' : 'translate-x-1'
                        }`} />
                      </button>
                    </div>
                  </div>

                  {/* Advanced Settings */}
                  <div className="space-y-4">
                    <h4 className="font-medium text-gray-900 dark:text-white">Configuración Avanzada</h4>
                    
                    <div>
                      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                        Intervalo de Sincronización (minutos)
                      </label>
                      <select 
                        value={syncConfig.sync_interval}
                        onChange={(e) => updateConfig({ sync_interval: parseInt(e.target.value) })}
                        className="block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
                      >
                        <option value={60}>1 minuto</option>
                        <option value={300}>5 minutos</option>
                        <option value={900}>15 minutos</option>
                        <option value={1800}>30 minutos</option>
                        <option value={3600}>1 hora</option>
                      </select>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                        Nivel de Encriptación
                      </label>
                      <select 
                        value={syncConfig.encryption_level}
                        onChange={(e) => updateConfig({ encryption_level: e.target.value })}
                        className="block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
                      >
                        <option value="Basic">Básico (AES-128)</option>
                        <option value="Standard">Estándar (AES-256)</option>
                        <option value="Military">Militar (AES-256 + ChaCha20)</option>
                      </select>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                        Máximo de Dispositivos
                      </label>
                      <input
                        type="number"
                        min="1"
                        max="50"
                        value={syncConfig.max_devices}
                        onChange={(e) => updateConfig({ max_devices: parseInt(e.target.value) })}
                        className="block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
                      />
                    </div>
                  </div>
                </div>

                {/* Network Settings */}
                <div className="border-t border-gray-200 dark:border-gray-700 pt-6">
                  <h4 className="font-medium text-gray-900 dark:text-white mb-4">Redes Permitidas</h4>
                  <div className="space-y-3">
                    {syncConfig.allowed_networks.map((network, index) => (
                      <div key={index} className="flex items-center justify-between bg-gray-50 dark:bg-gray-700 p-3 rounded-lg">
                        <span className="text-sm text-gray-700 dark:text-gray-300">{network}</span>
                        <button className="text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 text-sm">
                          Eliminar
                        </button>
                      </div>
                    ))}
                    <button className="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 text-sm font-medium">
                      + Agregar Red
                    </button>
                  </div>
                </div>

                {/* Save Button */}
                <div className="flex justify-end pt-6 border-t border-gray-200 dark:border-gray-700">
                  <button className="bg-blue-600 text-white px-6 py-2 rounded-md hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600 font-medium">
                    Guardar Configuración
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default SyncPage;
