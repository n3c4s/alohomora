import React, { useState, useEffect } from 'react';
import { Wifi, WifiOff, RefreshCw, AlertTriangle } from 'lucide-react';

interface SyncStatusIndicatorProps {
  className?: string;
}

const SyncStatusIndicator: React.FC<SyncStatusIndicatorProps> = ({ className = '' }) => {
  const [syncStatus, setSyncStatus] = useState<'enabled' | 'disabled' | 'syncing' | 'error'>('enabled');
  const [lastSync, setLastSync] = useState<string | null>(null);
  const [connectedDevices, setConnectedDevices] = useState(0);

  useEffect(() => {
    // TODO: Implementar llamadas a Tauri para obtener estado real
    // Por ahora usamos datos de ejemplo
    const interval = setInterval(() => {
      // Simular cambios de estado
      const statuses: Array<'enabled' | 'disabled' | 'syncing' | 'error'> = ['enabled', 'syncing', 'enabled'];
      const randomStatus = statuses[Math.floor(Math.random() * statuses.length)];
      setSyncStatus(randomStatus);
      
      if (randomStatus === 'enabled') {
        setLastSync(new Date().toLocaleTimeString());
        setConnectedDevices(Math.floor(Math.random() * 3) + 1);
      }
    }, 10000); // Cambiar cada 10 segundos para demostración

    return () => clearInterval(interval);
  }, []);

  const getStatusIcon = () => {
    switch (syncStatus) {
      case 'enabled':
        return <Wifi className="w-4 h-4 text-green-500" />;
      case 'disabled':
        return <WifiOff className="w-4 h-4 text-gray-400" />;
      case 'syncing':
        return <RefreshCw className="w-4 h-4 text-blue-500 animate-spin" />;
      case 'error':
        return <AlertTriangle className="w-4 h-4 text-red-500" />;
      default:
        return <Wifi className="w-4 h-4 text-gray-400" />;
    }
  };

  const getStatusText = () => {
    switch (syncStatus) {
      case 'enabled':
        return 'Sincronizado';
      case 'disabled':
        return 'Desconectado';
      case 'syncing':
        return 'Sincronizando...';
      case 'error':
        return 'Error';
      default:
        return 'Desconocido';
    }
  };

  const getStatusColor = () => {
    switch (syncStatus) {
      case 'enabled':
        return 'text-green-600 bg-green-50 border-green-200';
      case 'disabled':
        return 'text-gray-600 bg-gray-50 border-gray-200';
      case 'syncing':
        return 'text-blue-600 bg-blue-50 border-blue-200';
      case 'error':
        return 'text-red-600 bg-red-50 border-red-200';
      default:
        return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  return (
    <div className={`flex items-center space-x-2 ${className}`}>
      <div className={`flex items-center space-x-2 px-3 py-1.5 rounded-full border text-xs font-medium ${getStatusColor()}`}>
        {getStatusIcon()}
        <span>{getStatusText()}</span>
      </div>
      
      {syncStatus === 'enabled' && connectedDevices > 0 && (
        <div className="text-xs text-gray-500">
          {connectedDevices} dispositivo{connectedDevices !== 1 ? 's' : ''}
        </div>
      )}
      
      {lastSync && (
        <div className="text-xs text-gray-400">
          Última: {lastSync}
        </div>
      )}
    </div>
  );
};

export default SyncStatusIndicator;
