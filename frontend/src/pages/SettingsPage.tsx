import { useAuthStore } from '../stores/authStore'
import { useNavigate } from 'react-router-dom'
import { LogOut, Shield, User, Settings as SettingsIcon, Trash2 } from 'lucide-react'

const SettingsPage = () => {
  const { logout, isAuthenticated } = useAuthStore()
  const navigate = useNavigate()

  const handleLogout = () => {
    console.log('🔄 Frontend: Iniciando logout...')
    logout()
    console.log('✅ Frontend: Logout completado')
    navigate('/login')
  }

  const handleChangeMasterPassword = () => {
    // TODO: Implementar cambio de contraseña maestra
    console.log('🔄 Frontend: Función de cambio de contraseña maestra (pendiente)')
  }

  const handleExportPasswords = () => {
    // TODO: Implementar exportación de contraseñas
    console.log('🔄 Frontend: Función de exportación (pendiente)')
  }

  const handleImportPasswords = () => {
    // TODO: Implementar importación de contraseñas
    console.log('🔄 Frontend: Función de importación (pendiente)')
  }

  const handleClearAllData = () => {
    // TODO: Implementar limpieza de datos
    console.log('🔄 Frontend: Función de limpieza de datos (pendiente)')
  }

  return (
    <div className="max-w-4xl mx-auto p-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white flex items-center gap-3">
          <SettingsIcon className="w-8 h-8" />
          Configuración
        </h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Personaliza Alohopass según tus preferencias y gestiona tu cuenta
        </p>
      </div>

      <div className="grid gap-6">
        {/* Sección de Cuenta */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
            <User className="w-5 h-5" />
            Cuenta
          </h2>
          
          <div className="space-y-4">
            <div className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
              <div>
                <h3 className="font-medium text-gray-900 dark:text-white">Estado de Sesión</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  {isAuthenticated ? 'Sesión activa' : 'No autenticado'}
                </p>
              </div>
              <div className="flex items-center gap-2">
                <div className={`w-3 h-3 rounded-full ${isAuthenticated ? 'bg-green-500' : 'bg-red-500'}`}></div>
                <span className="text-sm text-gray-600 dark:text-gray-400">
                  {isAuthenticated ? 'Conectado' : 'Desconectado'}
                </span>
              </div>
            </div>

            <button
              onClick={handleChangeMasterPassword}
              className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            >
              <Shield className="w-4 h-4" />
              Cambiar Contraseña Maestra
            </button>

            <button
              onClick={handleLogout}
              className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
            >
              <LogOut className="w-4 h-4" />
              Cerrar Sesión
            </button>
          </div>
        </div>

        {/* Sección de Datos */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
            <Shield className="w-5 h-5" />
            Gestión de Datos
          </h2>
          
          <div className="space-y-4">
            <button
              onClick={handleExportPasswords}
              className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
            >
              Exportar Contraseñas
            </button>

            <button
              onClick={handleImportPasswords}
              className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            >
              Importar Contraseñas
            </button>

            <button
              onClick={handleClearAllData}
              className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
            >
              <Trash2 className="w-4 h-4" />
              Limpiar Todos los Datos
            </button>
          </div>
        </div>

        {/* Sección de Información */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
            Información
          </h2>
          
          <div className="space-y-3 text-sm text-gray-600 dark:text-gray-400">
            <div className="flex justify-between">
              <span>Versión:</span>
              <span>1.0.0</span>
            </div>
            <div className="flex justify-between">
              <span>Base de Datos:</span>
              <span>SQLite</span>
            </div>
            <div className="flex justify-between">
              <span>Encriptación:</span>
              <span>ChaCha20-Poly1305</span>
            </div>
            <div className="flex justify-between">
              <span>Hash de Contraseñas:</span>
              <span>Argon2</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default SettingsPage 