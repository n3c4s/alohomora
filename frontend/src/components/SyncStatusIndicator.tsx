import React from 'react';
import { useSyncStore } from '../stores/syncStore';
import { Wifi, WifiOff, RefreshCw, AlertCircle } from 'lucide-react';

const SyncStatusIndicator: React.FC = () => {
  const { status, devices } = useSyncStore();

  const getStatusIcon = () => {
    if (status.error) {
      return <AlertCircle className="w-4 h-4 text-red-500" />;
    }
    if (status.isSyncing) {
      return <RefreshCw className="w-4 h-4 text-blue-500 animate-spin" />;
    }
    if (status.isEnabled) {
      return <Wifi className="w-4 h-4 text-green-500" />;
    }
    return <WifiOff className="w-4 h-4 text-gray-400" />;
  };

  const getStatusText = () => {
    if (status.error) {
      return 'Error';
    }
    if (status.isSyncing) {
      return 'Sincronizando...';
    }
    if (status.isEnabled) {
      return `${status.connectedDevices} dispositivo${status.connectedDevices !== 1 ? 's' : ''}`;
    }
    return 'Desactivado';
  };

  const getStatusColor = () => {
    if (status.error) {
      return 'text-red-600 dark:text-red-400';
    }
    if (status.isSyncing) {
      return 'text-blue-600 dark:text-blue-400';
    }
    if (status.isEnabled) {
      return 'text-green-600 dark:text-green-400';
    }
    return 'text-gray-500 dark:text-gray-400';
  };

  return (
    <div className="flex items-center space-x-2 px-3 py-2 rounded-md bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors cursor-pointer">
      {getStatusIcon()}
      <span className={`text-sm font-medium ${getStatusColor()}`}>
        {getStatusText()}
      </span>
    </div>
  );
};

export default SyncStatusIndicator;
