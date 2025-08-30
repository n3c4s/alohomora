import React, { useEffect, useState } from 'react';
import { useSyncStore } from '../stores/syncStore';
import { 
  RefreshCw, 
  Wifi, 
  Shield, 
  Settings, 
  Play, 
  Pause, 
  Search,
  CheckCircle,
  XCircle,
  Clock,
  Smartphone,
  Monitor,
  Tablet
} from 'lucide-react';

const SyncPage: React.FC = () => {
  const {
    status,
    config,
    stats,
    devices,
    loadSyncData,
    toggleSync,
    startDiscovery,
    syncNow,
    updateConfig,
    trustDevice,
    removeDevice,
    clearError
  } = useSyncStore();

  const [activeTab, setActiveTab] = useState('overview');

  useEffect(() => {
    loadSyncData();
  }, [loadSyncData]);

  const getDeviceIcon = (type: string) => {
    switch (type) {
      case 'mobile':
        return <Smartphone className="w-5 h-5" />;
      case 'desktop':
        return <Monitor className="w-5 h-5" />;
      case 'tablet':
        return <Tablet className="w-5 h-5" />;
      default:
        return <Monitor className="w-5 h-5" />;
    }
  };

  const formatLastSeen = (lastSeen: string) => {
    const date = new Date(lastSeen);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    
    if (diffMins < 1) return 'Ahora mismo';
    if (diffMins < 60) return `Hace ${diffMins} min`;
    if (diffMins < 1440) return `Hace ${Math.floor(diffMins / 60)}h`;
    return `Hace ${Math.floor(diffMins / 1440)}d`;
  };

  const tabs = [
    { id: 'overview', name: 'Resumen', icon: Wifi },
    { id: 'devices', name: 'Dispositivos', icon: Monitor },
    { id: 'settings', name: 'Configuración', icon: Settings },
  ];

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Sincronización P2P
          </h1>
          <p className="mt-2 text-gray-600 dark:text-gray-400">
            Sincroniza tus contraseñas de forma segura entre dispositivos usando conexiones directas
          </p>
        </div>

        {/* Status Bar */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6 mb-8">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <div className={`w-3 h-3 rounded-full ${
                status.isEnabled ? 'bg-green-500' : 'bg-gray-400'
              }`} />
              <span className="text-sm font-medium text-gray-900 dark:text-white">
                {status.isEnabled ? 'Sincronización activa' : 'Sincronización desactivada'}
              </span>
            </div>
            
            <div className="flex items-center space-x-4 text-sm text-gray-600 dark:text-gray-400">
              <div className="flex items-center space-x-2">
                <Clock className="w-4 h-4" />
                <span>
                  {status.lastSyncTime ? formatLastSeen(status.lastSyncTime) : 'Nunca'}
                </span>
              </div>
              <div className="flex items-center space-x-2">
                <Wifi className="w-4 h-4" />
                <span>{status.connectedDevices} dispositivos</span>
              </div>
            </div>

            <div className="flex space-x-3">
              <button
                onClick={startDiscovery}
                disabled={status.isSyncing}
                className="inline-flex items-center px-3 py-2 border border-gray-300 dark:border-gray-600 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
              >
                <Search className="w-4 h-4 mr-2" />
                Buscar
              </button>
              
              <button
                onClick={syncNow}
                disabled={status.isSyncing || !status.isEnabled}
                className="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
              >
                {status.isSyncing ? (
                  <RefreshCw className="w-4 h-4 mr-2 animate-spin" />
                ) : (
                  <Play className="w-4 h-4 mr-2" />
                )}
                {status.isSyncing ? 'Sincronizando...' : 'Sincronizar'}
              </button>

              <button
                onClick={toggleSync}
                className={`inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md ${
                  status.isEnabled
                    ? 'text-red-700 bg-red-100 hover:bg-red-200 dark:text-red-200 dark:bg-red-900 dark:hover:bg-red-800'
                    : 'text-green-700 bg-green-100 hover:bg-green-200 dark:text-green-200 dark:bg-green-900 dark:hover:bg-green-800'
                } focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500`}
              >
                {status.isEnabled ? (
                  <>
                    <Pause className="w-4 h-4 mr-2" />
                    Detener
                  </>
                ) : (
                  <>
                    <Play className="w-4 h-4 mr-2" />
                    Activar
                  </>
                )}
              </button>
            </div>
          </div>
        </div>

        {/* Error Display */}
        {status.error && (
          <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-8">
            <div className="flex">
              <XCircle className="w-5 h-5 text-red-400 mr-3" />
              <div className="flex-1">
                <h3 className="text-sm font-medium text-red-800 dark:text-red-200">
                  Error de sincronización
                </h3>
                <p className="mt-1 text-sm text-red-700 dark:text-red-300">
                  {status.error}
                </p>
              </div>
              <button
                onClick={clearError}
                className="text-red-400 hover:text-red-600 dark:hover:text-red-300"
              >
                <XCircle className="w-5 h-5" />
              </button>
            </div>
          </div>
        )}

        {/* Tabs */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
          <div className="border-b border-gray-200 dark:border-gray-700">
            <nav className="-mb-px flex space-x-8 px-6">
              {tabs.map((tab) => {
                const Icon = tab.icon;
                return (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={`py-4 px-1 border-b-2 font-medium text-sm ${
                      activeTab === tab.id
                        ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                        : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300'
                    }`}
                  >
                    <Icon className="w-5 h-5 inline mr-2" />
                    {tab.name}
                  </button>
                );
              })}
            </nav>
          </div>

          <div className="p-6">
            {/* Overview Tab */}
            {activeTab === 'overview' && (
              <div className="space-y-6">
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-6">
                    <div className="flex items-center">
                      <div className="flex-shrink-0">
                        <Shield className="w-8 h-8 text-blue-600 dark:text-blue-400" />
                      </div>
                      <div className="ml-4">
                        <p className="text-sm font-medium text-blue-600 dark:text-blue-400">
                          Contraseñas totales
                        </p>
                        <p className="text-2xl font-semibold text-blue-900 dark:text-blue-100">
                          {stats.totalPasswords}
                        </p>
                      </div>
                    </div>
                  </div>

                  <div className="bg-green-50 dark:bg-green-900/20 rounded-lg p-6">
                    <div className="flex items-center">
                      <div className="flex-shrink-0">
                        <CheckCircle className="w-8 h-8 text-green-600 dark:text-green-400" />
                      </div>
                      <div className="ml-4">
                        <p className="text-sm font-medium text-green-600 dark:text-green-400">
                          Sincronizadas
                        </p>
                        <p className="text-2xl font-semibold text-green-900 dark:text-green-100">
                          {stats.syncedPasswords}
                        </p>
                      </div>
                    </div>
                  </div>

                  <div className="bg-purple-50 dark:bg-purple-900/20 rounded-lg p-6">
                    <div className="flex items-center">
                      <div className="flex-shrink-0">
                        <Wifi className="w-8 h-8 text-purple-600 dark:text-purple-400" />
                      </div>
                      <div className="ml-4">
                        <p className="text-sm font-medium text-purple-600 dark:text-purple-400">
                          Dispositivos
                        </p>
                        <p className="text-2xl font-semibold text-purple-900 dark:text-purple-100">
                          {stats.devicesCount}
                        </p>
                      </div>
                    </div>
                  </div>
                </div>

                <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                    Última sincronización
                  </h3>
                  <div className="space-y-3">
                    <div className="flex justify-between text-sm">
                      <span className="text-gray-600 dark:text-gray-400">Estado:</span>
                      <span className={`font-medium ${
                        status.isEnabled ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'
                      }`}>
                        {status.isEnabled ? 'Activa' : 'Inactiva'}
                      </span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span className="text-gray-600 dark:text-gray-400">Última sincronización:</span>
                      <span className="text-gray-900 dark:text-white">
                        {status.lastSyncTime ? formatLastSeen(status.lastSyncTime) : 'Nunca'}
                      </span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span className="text-gray-600 dark:text-gray-400">Duración última sync:</span>
                      <span className="text-gray-900 dark:text-white">
                        {stats.lastSyncDuration > 0 ? `${stats.lastSyncDuration}s` : 'N/A'}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Devices Tab */}
            {activeTab === 'devices' && (
              <div className="space-y-6">
                <div className="flex justify-between items-center">
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                    Dispositivos ({devices.length})
                  </h3>
                  <button
                    onClick={startDiscovery}
                    disabled={status.isSyncing}
                    className="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 shadow-sm text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                  >
                    <Search className="w-4 h-4 mr-2" />
                    Buscar dispositivos
                  </button>
                </div>

                {devices.length === 0 ? (
                  <div className="text-center py-12">
                    <Wifi className="mx-auto h-12 w-12 text-gray-400" />
                    <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
                      No hay dispositivos
                    </h3>
                    <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                      Haz clic en "Buscar dispositivos" para descubrir dispositivos en tu red
                    </p>
                  </div>
                ) : (
                  <div className="bg-white dark:bg-gray-700 shadow overflow-hidden sm:rounded-md">
                    <ul className="divide-y divide-gray-200 dark:divide-gray-600">
                      {devices.map((device) => (
                        <li key={device.id}>
                          <div className="px-4 py-4 flex items-center justify-between sm:px-6">
                            <div className="flex items-center">
                              <div className="flex-shrink-0">
                                {getDeviceIcon(device.type)}
                              </div>
                              <div className="ml-4">
                                <div className="flex items-center">
                                  <p className="text-sm font-medium text-gray-900 dark:text-white">
                                    {device.name}
                                  </p>
                                  {device.isTrusted && (
                                    <Shield className="ml-2 w-4 h-4 text-green-500" />
                                  )}
                                </div>
                                <div className="flex items-center mt-1">
                                  <span className="text-xs text-gray-500 dark:text-gray-400 capitalize">
                                    {device.type}
                                  </span>
                                  <span className="mx-2 text-gray-300 dark:text-gray-600">•</span>
                                  <span className="text-xs text-gray-500 dark:text-gray-400">
                                    {formatLastSeen(device.lastSeen)}
                                  </span>
                                </div>
                              </div>
                            </div>
                            <div className="flex items-center space-x-2">
                              {!device.isTrusted && (
                                <button
                                  onClick={() => trustDevice(device.id)}
                                  className="inline-flex items-center px-3 py-1 border border-transparent text-xs font-medium rounded text-green-700 bg-green-100 hover:bg-green-200 dark:text-green-200 dark:bg-green-900 dark:hover:bg-green-800"
                                >
                                  Confiar
                                </button>
                              )}
                              <button
                                onClick={() => removeDevice(device.id)}
                                className="inline-flex items-center px-3 py-1 border border-transparent text-xs font-medium rounded text-red-700 bg-red-100 hover:bg-red-200 dark:text-red-200 dark:bg-red-900 dark:hover:bg-red-800"
                              >
                                Remover
                              </button>
                            </div>
                          </div>
                        </li>
                      ))}
                    </ul>
                  </div>
                )}
              </div>
            )}

            {/* Settings Tab */}
            {activeTab === 'settings' && (
              <div className="space-y-6">
                <div className="bg-white dark:bg-gray-700 shadow sm:rounded-lg">
                  <div className="px-4 py-5 sm:p-6">
                    <h3 className="text-lg leading-6 font-medium text-gray-900 dark:text-white">
                      Configuración de sincronización
                    </h3>
                    <div className="mt-6 space-y-6">
                      <div className="flex items-center justify-between">
                        <div>
                          <label className="text-sm font-medium text-gray-900 dark:text-white">
                            Sincronización automática
                          </label>
                          <p className="text-sm text-gray-500 dark:text-gray-400">
                            Sincroniza automáticamente cada {config.syncInterval} minutos
                          </p>
                        </div>
                        <button
                          onClick={() => updateConfig({ autoSync: !config.autoSync })}
                          className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                            config.autoSync ? 'bg-blue-600' : 'bg-gray-200 dark:bg-gray-600'
                          }`}
                        >
                          <span className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                            config.autoSync ? 'translate-x-5' : 'translate-x-0'
                          }`} />
                        </button>
                      </div>

                      <div className="flex items-center justify-between">
                        <div>
                          <label className="text-sm font-medium text-gray-900 dark:text-white">
                            Descubrimiento automático
                          </label>
                          <p className="text-sm text-gray-500 dark:text-gray-400">
                            Busca automáticamente dispositivos en tu red
                          </p>
                        </div>
                        <button
                          onClick={() => updateConfig({ discoveryEnabled: !config.discoveryEnabled })}
                          className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                            config.discoveryEnabled ? 'bg-blue-600' : 'bg-gray-200 dark:bg-gray-600'
                          }`}
                        >
                          <span className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                            config.discoveryEnabled ? 'translate-x-5' : 'translate-x-0'
                          }`} />
                        </button>
                      </div>

                      <div className="flex items-center justify-between">
                        <div>
                          <label className="text-sm font-medium text-gray-900 dark:text-white">
                            Conexiones entrantes
                          </label>
                          <p className="text-sm text-gray-500 dark:text-gray-400">
                            Permite que otros dispositivos se conecten a ti
                          </p>
                        </div>
                        <button
                          onClick={() => updateConfig({ allowIncomingConnections: !config.allowIncomingConnections })}
                          className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                            config.allowIncomingConnections ? 'bg-blue-600' : 'bg-gray-200 dark:bg-gray-600'
                          }`}
                        >
                          <span className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                            config.allowIncomingConnections ? 'translate-x-5' : 'translate-x-0'
                          }`} />
                        </button>
                      </div>

                      <div>
                        <label className="block text-sm font-medium text-gray-900 dark:text-white">
                          Intervalo de sincronización (minutos)
                        </label>
                        <select
                          value={config.syncInterval}
                          onChange={(e) => updateConfig({ syncInterval: parseInt(e.target.value) })}
                          className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md dark:bg-gray-600 dark:text-white"
                        >
                          <option value={5}>5 minutos</option>
                          <option value={15}>15 minutos</option>
                          <option value={30}>30 minutos</option>
                          <option value={60}>1 hora</option>
                          <option value={120}>2 horas</option>
                        </select>
                      </div>
                    </div>
                  </div>
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
