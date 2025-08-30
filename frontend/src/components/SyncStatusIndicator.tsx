import React from 'react';
import { useSyncStore } from '../stores/syncStore';
import { RefreshCw, Wifi, WifiOff } from 'lucide-react';

const SyncStatusIndicator: React.FC = () => {
  const { status } = useSyncStore();

  if (!status.isEnabled) {
    return (
      <div className="flex items-center gap-2 text-gray-500">
        <WifiOff className="h-4 w-4" />
        <span className="text-sm">Sync desactivado</span>
      </div>
    );
  }

  return (
    <div className="flex items-center gap-2 text-green-600">
      <Wifi className="h-4 w-4" />
      <span className="text-sm">
        {status.connectedDevices} dispositivo{status.connectedDevices !== 1 ? 's' : ''}
      </span>
      {status.isSyncing && (
        <RefreshCw className="h-4 w-4 animate-spin" />
      )}
    </div>
  );
};

export default SyncStatusIndicator;
