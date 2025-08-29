import { useState } from 'react'
import { X, Download, RefreshCw, AlertTriangle } from 'lucide-react'
import { invoke } from '@tauri-apps/api/tauri'
import toast from 'react-hot-toast'

interface RecoveryModalProps {
  isOpen: boolean
  onClose: () => void
}

const RecoveryModal = ({ isOpen, onClose }: RecoveryModalProps) => {
  const [recoveryKey, setRecoveryKey] = useState('')
  const [isGenerating, setIsGenerating] = useState(false)

  const generateRecoveryKey = async () => {
    setIsGenerating(true)
    try {
      const key = await invoke('generate_recovery_key')
      setRecoveryKey(key as string)
      toast.success('Clave de recuperación generada correctamente')
    } catch (error) {
      toast.error(`Error al generar clave: ${error}`)
    } finally {
      setIsGenerating(false)
    }
  }

  const downloadRecoveryKey = () => {
    if (!recoveryKey) return
    
    const blob = new Blob([recoveryKey], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'alohopass-recovery-key.txt'
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
    
    toast.success('Clave de recuperación descargada')
  }

  const resetDatabase = () => {
    if (confirm('¿Estás seguro de que quieres eliminar toda la base de datos? Esta acción no se puede deshacer.')) {
      // Por ahora solo cerramos el modal
      // En el futuro implementaremos el reset real
      toast.error('Funcionalidad de reset en desarrollo. Por ahora, cierra la app y elimina manualmente la base de datos.')
      onClose()
    }
  }

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
            Recuperación de Contraseña
          </h3>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        <div className="space-y-4">
          <div className="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
            <div className="flex items-center space-x-2">
              <AlertTriangle className="h-5 w-5 text-yellow-600 dark:text-yellow-400" />
              <span className="text-sm text-yellow-800 dark:text-yellow-200 font-medium">
                Importante
              </span>
            </div>
            <p className="mt-2 text-sm text-yellow-700 dark:text-yellow-300">
              Guarda esta clave de recuperación en un lugar seguro. Te permitirá acceder a tus contraseñas si olvidas la maestra.
            </p>
          </div>

          <div className="space-y-3">
            <div className="flex items-center space-x-2">
              <button
                onClick={generateRecoveryKey}
                disabled={isGenerating}
                className="btn-primary flex items-center space-x-2"
              >
                <RefreshCw className={`h-4 w-4 ${isGenerating ? 'animate-spin' : ''}`} />
                <span>{isGenerating ? 'Generando...' : 'Generar Clave'}</span>
              </button>
            </div>

            {recoveryKey && (
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Clave de Recuperación:
                </label>
                <div className="bg-gray-100 dark:bg-gray-700 p-3 rounded border font-mono text-sm break-all">
                  {recoveryKey}
                </div>
                <button
                  onClick={downloadRecoveryKey}
                  className="btn-secondary w-full flex items-center justify-center space-x-2"
                >
                  <Download className="h-4 w-4" />
                  <span>Descargar Clave</span>
                </button>
              </div>
            )}
          </div>

          <div className="border-t pt-4">
            <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
              <h4 className="text-sm font-medium text-red-800 dark:text-red-200 mb-2">
                Último Recurso
              </h4>
              <p className="text-sm text-red-700 dark:text-red-300 mb-3">
                Si no tienes la clave de recuperación y olvidaste tu contraseña maestra, puedes resetear la base de datos.
              </p>
              <button
                onClick={resetDatabase}
                className="btn-danger w-full text-sm"
              >
                Resetear Base de Datos
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default RecoveryModal 