import { useState } from 'react'
import { X, Download, Eye, EyeOff, RefreshCw } from 'lucide-react'
import { useRecoveryStore } from '../stores/recoveryStore'
import toast from 'react-hot-toast'

interface RecoveryModalProps {
  isOpen: boolean
  onClose: () => void
  onSuccess: () => void
}

const RecoveryModal = ({ isOpen, onClose, onSuccess }: RecoveryModalProps) => {
  const [mode, setMode] = useState<'generate' | 'reset'>('generate')
  const [password, setPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [recoveryKey, setRecoveryKey] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [showNewPassword, setShowNewPassword] = useState(false)
  
  const { generateRecoveryKey, resetWithRecoveryKey, isLoading, error, recoveryKey: storedRecoveryKey } = useRecoveryStore()

  const handleGenerateRecoveryKey = async () => {
    if (!password.trim()) {
      toast.error('Por favor ingresa tu contraseña maestra')
      return
    }
    
    const key = await generateRecoveryKey(password)
    if (key) {
      toast.success('Clave de recuperación generada. ¡Guárdala en un lugar seguro!')
    }
  }

  const handleResetPassword = async () => {
    if (!recoveryKey.trim() || !newPassword.trim()) {
      toast.error('Por favor completa todos los campos')
      return
    }
    
    if (newPassword.length < 8) {
      toast.error('La nueva contraseña debe tener al menos 8 caracteres')
      return
    }
    
    const success = await resetWithRecoveryKey(recoveryKey, newPassword)
    if (success) {
      toast.success('Contraseña maestra restablecida correctamente')
      onSuccess()
      onClose()
    }
  }

  const downloadRecoveryKey = () => {
    if (storedRecoveryKey) {
      const blob = new Blob([storedRecoveryKey], { type: 'text/plain' })
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
  }

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4">
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
            Recuperación de Contraseña Maestra
          </h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          >
            <X className="h-6 w-6" />
          </button>
        </div>

        <div className="p-6 space-y-4">
          {/* Tabs */}
          <div className="flex space-x-1 bg-gray-100 dark:bg-gray-700 rounded-lg p-1">
            <button
              onClick={() => setMode('generate')}
              className={`flex-1 py-2 px-3 rounded-md text-sm font-medium transition-colors ${
                mode === 'generate'
                  ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
                  : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white'
              }`}
            >
              Generar Clave
            </button>
            <button
              onClick={() => setMode('reset')}
              className={`flex-1 py-2 px-3 rounded-md text-sm font-medium transition-colors ${
                mode === 'reset'
                  ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
                  : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white'
              }`}
            >
              Restablecer
            </button>
          </div>

          {mode === 'generate' ? (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Contraseña Maestra Actual
                </label>
                <div className="relative">
                  <input
                    type={showPassword ? 'text' : 'password'}
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    className="input-field pr-10"
                    placeholder="Ingresa tu contraseña maestra"
                  />
                  <button
                    onClick={() => setShowPassword(!showPassword)}
                    className="absolute inset-y-0 right-0 pr-3 text-gray-400 hover:text-gray-600"
                  >
                    {showPassword ? <EyeOff className="h-5 w-5" /> : <Eye className="h-5 w-5" />}
                  </button>
                </div>
              </div>

              <button
                onClick={handleGenerateRecoveryKey}
                disabled={isLoading}
                className="btn btn-primary w-full"
              >
                {isLoading ? (
                  <RefreshCw className="h-4 w-4 animate-spin mr-2" />
                ) : null}
                Generar Clave de Recuperación
              </button>

              {storedRecoveryKey && (
                <div className="space-y-3">
                  <div className="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
                    <h4 className="text-sm font-medium text-yellow-800 dark:text-yellow-200 mb-2">
                      ⚠️ Clave de Recuperación Generada
                    </h4>
                    <p className="text-sm text-yellow-700 dark:text-yellow-300 mb-3">
                      Guarda esta clave en un lugar seguro. Te permitirá restablecer tu contraseña maestra si la olvidas.
                    </p>
                    <div className="bg-white dark:bg-gray-800 p-3 rounded border font-mono text-sm break-all">
                      {storedRecoveryKey}
                    </div>
                  </div>
                  
                  <button
                    onClick={downloadRecoveryKey}
                    className="btn btn-secondary w-full flex items-center justify-center"
                  >
                    <Download className="h-4 w-4 mr-2" />
                    Descargar Clave
                  </button>
                </div>
              )}
            </div>
          ) : (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Clave de Recuperación
                </label>
                <input
                  type="text"
                  value={recoveryKey}
                  onChange={(e) => setRecoveryKey(e.target.value)}
                  className="input-field"
                  placeholder="Ingresa tu clave de recuperación"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Nueva Contraseña Maestra
                </label>
                <div className="relative">
                  <input
                    type={showNewPassword ? 'text' : 'password'}
                    value={newPassword}
                    onChange={(e) => setNewPassword(e.target.value)}
                    className="input-field pr-10"
                    placeholder="Nueva contraseña maestra"
                  />
                  <button
                    onClick={() => setShowNewPassword(!showNewPassword)}
                    className="absolute inset-y-0 right-0 pr-3 text-gray-400 hover:text-gray-600"
                  >
                    {showNewPassword ? <EyeOff className="h-5 w-5" /> : <Eye className="h-5 w-5" />}
                  </button>
                </div>
              </div>

              <button
                onClick={handleResetPassword}
                disabled={isLoading}
                className="btn btn-primary w-full"
              >
                {isLoading ? (
                  <RefreshCw className="h-4 w-4 animate-spin mr-2" />
                ) : null}
                Restablecer Contraseña
              </button>
            </div>
          )}

          {error && (
            <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3">
              <p className="text-sm text-red-700 dark:text-red-300">{error}</p>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default RecoveryModal 