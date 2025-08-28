import { useState, useEffect } from 'react'
import { Shield, Lock, Key, AlertTriangle, TrendingUp, Clock, Plus, Search } from 'lucide-react'
import { usePasswordStore } from '../stores/passwordStore'
import { Link } from 'react-router-dom'

const DashboardPage = () => {
  const { passwords, fetchPasswords } = usePasswordStore()
  const [stats, setStats] = useState({
    total: 0,
    weak: 0,
    medium: 0,
    strong: 0,
    veryStrong: 0,
    recent: 0,
    categories: 0,
  })

  useEffect(() => {
    fetchPasswords()
  }, [fetchPasswords])

  useEffect(() => {
    if (passwords.length > 0) {
      // Calcular estadísticas
      const total = passwords.length
      const weak = passwords.filter(p => p.password.length < 8).length
      const medium = passwords.filter(p => p.password.length >= 8 && p.password.length < 12).length
      const strong = passwords.filter(p => p.password.length >= 12 && p.password.length < 16).length
      const veryStrong = passwords.filter(p => p.password.length >= 16).length
      
      // Contar contraseñas recientes (últimos 7 días)
      const weekAgo = new Date()
      weekAgo.setDate(weekAgo.getDate() - 7)
      const recent = passwords.filter(p => new Date(p.created_at) > weekAgo).length
      
      // Contar categorías únicas
      const categories = new Set(passwords.map(p => p.category_id).filter(Boolean)).size
      
      setStats({
        total,
        weak,
        medium,
        strong,
        veryStrong,
        recent,
        categories,
      })
    }
  }, [passwords])

  const getSecurityScore = () => {
    if (stats.total === 0) return 0
    const score = ((stats.strong + stats.veryStrong) / stats.total) * 100
    return Math.round(score)
  }

  const getSecurityColor = (score: number) => {
    if (score >= 80) return 'text-green-600 bg-green-100 dark:bg-green-900/20'
    if (score >= 60) return 'text-yellow-600 bg-yellow-100 dark:bg-yellow-900/20'
    if (score >= 40) return 'text-orange-600 bg-orange-100 dark:bg-orange-900/20'
    return 'text-red-600 bg-red-100 dark:bg-red-900/20'
  }

  const getSecurityText = (score: number) => {
    if (score >= 80) return 'Excelente'
    if (score >= 60) return 'Buena'
    if (score >= 40) return 'Regular'
    return 'Necesita Mejora'
  }

  const recentPasswords = passwords
    .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    .slice(0, 5)

  const weakPasswords = passwords
    .filter(p => p.password.length < 8)
    .slice(0, 3)

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Dashboard
        </h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Resumen de la seguridad de tus contraseñas
        </p>
      </div>

      {/* Tarjetas de estadísticas */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-primary-100 dark:bg-primary-900/20 rounded-lg">
              <Shield className="h-6 w-6 text-primary-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Puntuación de Seguridad
              </p>
              <p className={`text-2xl font-bold ${getSecurityColor(getSecurityScore())}`}>
                {getSecurityScore()}%
              </p>
            </div>
          </div>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {getSecurityText(getSecurityScore())}
          </p>
        </div>

        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-blue-100 dark:bg-blue-900/20 rounded-lg">
              <Lock className="h-6 w-6 text-blue-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Total de Contraseñas
              </p>
              <p className="text-2xl font-bold text-blue-600">
                {stats.total}
              </p>
            </div>
          </div>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {stats.recent} nuevas esta semana
          </p>
        </div>

        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-green-100 dark:bg-green-900/20 rounded-lg">
              <Key className="h-6 w-6 text-green-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Contraseñas Fuertes
              </p>
              <p className="text-2xl font-bold text-green-600">
                {stats.strong + stats.veryStrong}
              </p>
            </div>
          </div>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {stats.veryStrong} muy fuertes
          </p>
        </div>

        <div className="card">
          <div className="flex items-center">
            <div className="p-2 bg-yellow-100 dark:bg-yellow-900/20 rounded-lg">
              <AlertTriangle className="h-6 w-6 text-yellow-600" />
            </div>
            <div className="ml-4">
              <p className="text-sm font-medium text-gray-600 dark:text-gray-400">
                Necesitan Mejora
              </p>
              <p className="text-2xl font-bold text-yellow-600">
                {stats.weak + stats.medium}
              </p>
            </div>
          </div>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {stats.weak} débiles
          </p>
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
            <Link
              to="/passwords"
              className="text-primary-600 hover:text-primary-700 text-sm font-medium"
            >
              Ver todas
            </Link>
          </div>
          
          {recentPasswords.length === 0 ? (
            <div className="text-center py-8 text-gray-500 dark:text-gray-400">
              <Clock className="h-12 w-12 mx-auto mb-4 text-gray-300" />
              <p>No hay contraseñas recientes</p>
            </div>
          ) : (
            <div className="space-y-3">
              {recentPasswords.map((password) => (
                <div
                  key={password.id}
                  className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg"
                >
                  <div>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {password.title}
                    </p>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      {password.username}
                    </p>
                  </div>
                  <span className="text-xs text-gray-500 dark:text-gray-400">
                    {new Date(password.created_at).toLocaleDateString()}
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Contraseñas que necesitan mejora */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
              Necesitan Mejora
            </h3>
            <Link
              to="/generator"
              className="text-primary-600 hover:text-primary-700 text-sm font-medium"
            >
              Generar
            </Link>
          </div>
          
          {weakPasswords.length === 0 ? (
            <div className="text-center py-8 text-gray-500 dark:text-gray-400">
              <Shield className="h-12 w-12 mx-auto mb-4 text-green-300" />
              <p>¡Excelente! Todas tus contraseñas son seguras</p>
            </div>
          ) : (
            <div className="space-y-3">
              {weakPasswords.map((password) => (
                <div
                  key={password.id}
                  className="flex items-center justify-between p-3 bg-red-50 dark:bg-red-900/20 rounded-lg border border-red-200 dark:border-red-800"
                >
                  <div>
                    <p className="font-medium text-gray-900 dark:text-white">
                      {password.title}
                    </p>
                    <p className="text-sm text-red-600 dark:text-red-400">
                      Solo {password.password.length} caracteres
                    </p>
                  </div>
                  <span className="text-xs text-red-500 dark:text-red-400">
                    Débil
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Acciones rápidas */}
      <div className="card">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Acciones Rápidas
        </h3>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <Link
            to="/passwords"
            className="flex items-center p-4 bg-primary-50 dark:bg-primary-900/20 border border-primary-200 dark:border-primary-800 rounded-lg hover:bg-primary-100 dark:hover:bg-primary-900/40 transition-colors"
          >
            <Plus className="h-8 w-8 text-primary-600 mr-3" />
            <div>
              <h4 className="font-medium text-primary-900 dark:text-primary-100">
                Nueva Contraseña
              </h4>
              <p className="text-sm text-primary-700 dark:text-primary-300">
                Agregar una nueva entrada
              </p>
            </div>
          </Link>
          
          <Link
            to="/generator"
            className="flex items-center p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/40 transition-colors"
          >
            <Key className="h-8 w-8 text-blue-600 mr-3" />
            <div>
              <h4 className="font-medium text-blue-900 dark:text-blue-100">
                Generar Contraseña
              </h4>
              <p className="text-sm text-blue-700 dark:text-blue-300">
                Crear contraseña segura
              </p>
            </div>
          </Link>
          
          <Link
            to="/passwords"
            className="flex items-center p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg hover:bg-green-100 dark:hover:bg-green-900/40 transition-colors"
          >
            <Search className="h-8 w-8 text-green-600 mr-3" />
            <div>
              <h4 className="font-medium text-green-900 dark:text-green-100">
                Buscar
              </h4>
              <p className="text-sm text-green-700 dark:text-green-300">
                Encontrar contraseñas
              </p>
            </div>
          </Link>
        </div>
      </div>

      {/* Consejos de seguridad */}
      <div className="card bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-900/20 dark:to-indigo-900/20 border-blue-200 dark:border-blue-800">
        <h3 className="text-lg font-semibold text-blue-900 dark:text-blue-100 mb-3">
          🔒 Consejos de Seguridad
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-blue-800 dark:text-blue-200">
          <div>
            <h4 className="font-medium mb-2">Mejores Prácticas:</h4>
            <ul className="space-y-1">
              <li>• Usa contraseñas únicas para cada cuenta</li>
              <li>• Cambia contraseñas regularmente</li>
              <li>• Activa la autenticación de dos factores</li>
            </ul>
          </div>
          <div>
            <h4 className="font-medium mb-2">Características de Seguridad:</h4>
            <ul className="space-y-1">
              <li>• Mínimo 12 caracteres</li>
              <li>• Combinar letras, números y símbolos</li>
              <li>• Evitar información personal</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  )
}

export default DashboardPage 