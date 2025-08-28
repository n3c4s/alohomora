import { Shield, Lock, Key, AlertTriangle, TrendingUp, Clock } from 'lucide-react'

const DashboardPage = () => {
  // Datos de ejemplo - en una implementación real vendrían del store
  const stats = {
    totalPasswords: 24,
    weakPasswords: 3,
    reusedPasswords: 1,
    lastBackup: '2024-01-15',
    securityScore: 85,
  }

  const recentPasswords = [
    { id: 1, title: 'Gmail', username: 'usuario@gmail.com', lastUsed: '2024-01-20' },
    { id: 2, title: 'GitHub', username: 'devuser', lastUsed: '2024-01-19' },
    { id: 3, title: 'Netflix', username: 'streamer', lastUsed: '2024-01-18' },
  ]

  const securityIssues = [
    { type: 'weak', count: 3, description: 'Contraseñas débiles detectadas' },
    { type: 'reused', count: 1, description: 'Contraseñas reutilizadas' },
    { type: 'old', count: 5, description: 'Contraseñas sin actualizar >90 días' },
  ]

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Dashboard
        </h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Resumen de seguridad y estadísticas de tus contraseñas
        </p>
      </div>

      {/* Tarjetas de estadísticas */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-primary-100 dark:bg-primary-900 rounded-lg">
              <Lock className="h-6 w-6 text-primary-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Total Contraseñas
              </p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {stats.totalPasswords}
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-green-100 dark:bg-green-900 rounded-lg">
              <Shield className="h-6 w-6 text-green-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Puntuación de Seguridad
              </p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {stats.securityScore}/100
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-yellow-100 dark:bg-yellow-900 rounded-lg">
              <AlertTriangle className="h-6 w-6 text-yellow-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Contraseñas Débiles
              </p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">
                {stats.weakPasswords}
              </p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg">
              <Clock className="h-6 w-6 text-blue-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Último Backup
              </p>
              <p className="text-lg font-bold text-gray-900 dark:text-white">
                {stats.lastBackup}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Contenido principal */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Contraseñas recientes */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
              Contraseñas Recientes
            </h3>
            <button className="text-primary-600 hover:text-primary-700 text-sm font-medium">
              Ver todas
            </button>
          </div>
          <div className="space-y-3">
            {recentPasswords.map((password) => (
              <div
                key={password.id}
                className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
              >
                <div>
                  <p className="font-medium text-gray-900 dark:text-white">
                    {password.title}
                  </p>
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    {password.username}
                  </p>
                </div>
                <div className="text-right">
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    Último uso
                  </p>
                  <p className="text-sm font-medium text-gray-900 dark:text-white">
                    {password.lastUsed}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Problemas de seguridad */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
              Problemas de Seguridad
            </h3>
            <button className="text-primary-600 hover:text-primary-700 text-sm font-medium">
              Resolver
            </button>
          </div>
          <div className="space-y-3">
            {securityIssues.map((issue, index) => (
              <div
                key={index}
                className="flex items-center justify-between p-3 bg-red-50 dark:bg-red-900/20 rounded-lg border border-red-200 dark:border-red-800"
              >
                <div className="flex items-center">
                  <AlertTriangle className="h-5 w-5 text-red-600 mr-3" />
                  <div>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {issue.description}
                    </p>
                  </div>
                </div>
                <span className="bg-red-100 dark:bg-red-800 text-red-800 dark:text-red-200 text-sm font-medium px-2.5 py-0.5 rounded-full">
                  {issue.count}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Acciones rápidas */}
      <div className="card">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Acciones Rápidas
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <button className="btn-primary flex items-center justify-center">
            <Key className="h-5 w-5 mr-2" />
            Generar Contraseña
          </button>
          <button className="btn-secondary flex items-center justify-center">
            <Lock className="h-5 w-5 mr-2" />
            Agregar Nueva
          </button>
          <button className="btn-secondary flex items-center justify-center">
            <TrendingUp className="h-5 w-5 mr-2" />
            Ver Reporte
          </button>
        </div>
      </div>
    </div>
  )
}

export default DashboardPage 