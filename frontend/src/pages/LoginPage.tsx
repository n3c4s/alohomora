import { useState, useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { Shield, Eye, EyeOff, Sparkles } from 'lucide-react'
import { useAuthStore } from '../stores/authStore'
import toast from 'react-hot-toast'

interface LoginForm {
  password: string
  confirmPassword?: string
}

const LoginPage = () => {
  const [isFirstTime, setIsFirstTime] = useState(false)
  const [showPassword, setShowPassword] = useState(false)
  const [showConfirmPassword, setShowConfirmPassword] = useState(false)
  
  const { 
    isInitialized, 
    isLoading, 
    error, 
    initializeMasterPassword, 
    verifyMasterPassword, 
    clearError 
  } = useAuthStore()
  
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<LoginForm>()
  
  const password = watch('password')
  const confirmPassword = watch('confirmPassword')

  // Verificar si es la primera vez
  useEffect(() => {
    if (!isInitialized) {
      setIsFirstTime(true)
    }
  }, [isInitialized])

  // Limpiar errores cuando cambien
  useEffect(() => {
    if (error) {
      toast.error(error)
      clearError()
    }
  }, [error, clearError])

  const onSubmit = async (data: LoginForm) => {
    if (isFirstTime && data.password !== data.confirmPassword) {
      toast.error('Las contraseñas no coinciden')
      return
    }
    
    if (isFirstTime) {
      const success = await initializeMasterPassword(data.password)
      if (success) {
        toast.success('¡Bienvenido a Alohopass!')
      }
    } else {
      const success = await verifyMasterPassword(data.password)
      if (success) {
        toast.success('¡Bienvenido de vuelta!')
      }
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-accent-50 dark:from-gray-900 dark:to-gray-800 p-4">
      <div className="max-w-md w-full space-y-8">
        {/* Header */}
        <div className="text-center">
          <div className="mx-auto h-20 w-20 bg-gradient-to-r from-primary-600 to-accent-600 rounded-full flex items-center justify-center">
            <Shield className="h-10 w-10 text-white" />
          </div>
          <h2 className="mt-6 text-3xl font-bold text-gray-900 dark:text-white">
            {isFirstTime ? 'Configura Alohopass' : 'Bienvenido de vuelta'}
          </h2>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {isFirstTime 
              ? 'Crea tu contraseña maestra para comenzar'
              : 'Ingresa tu contraseña maestra para continuar'
            }
          </p>
        </div>

        {/* Formulario */}
        <form className="mt-8 space-y-6" onSubmit={handleSubmit(onSubmit)}>
          <div className="space-y-4">
            {/* Contraseña */}
            <div>
              <label htmlFor="password" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                {isFirstTime ? 'Contraseña Maestra' : 'Contraseña'}
              </label>
              <div className="relative">
                <input
                  {...register('password', { 
                    required: 'La contraseña es requerida',
                    minLength: { value: 8, message: 'Mínimo 8 caracteres' }
                  })}
                  type={showPassword ? 'text' : 'password'}
                  id="password"
                  className="input-field pr-10"
                  placeholder={isFirstTime ? 'Crea una contraseña segura' : 'Ingresa tu contraseña'}
                />
                <button
                  type="button"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                >
                  {showPassword ? <EyeOff className="h-5 w-5" /> : <Eye className="h-5 w-5" />}
                </button>
              </div>
              {errors.password && (
                <p className="mt-1 text-sm text-red-600 dark:text-red-400">
                  {errors.password.message}
                </p>
              )}
            </div>

            {/* Confirmar contraseña (solo primera vez) */}
            {isFirstTime && (
              <div>
                <label htmlFor="confirmPassword" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Confirmar Contraseña
                </label>
                <div className="relative">
                  <input
                    {...register('confirmPassword', { 
                      required: 'Confirma tu contraseña',
                      validate: value => value === password || 'Las contraseñas no coinciden'
                    })}
                    type={showConfirmPassword ? 'text' : 'password'}
                    id="confirmPassword"
                    className="input-field pr-10"
                    placeholder="Confirma tu contraseña"
                  />
                  <button
                    type="button"
                    onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                    className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  >
                    {showConfirmPassword ? <EyeOff className="h-5 w-5" /> : <Eye className="h-5 w-5" />}
                  </button>
                </div>
                {errors.confirmPassword && (
                  <p className="mt-1 text-sm text-red-600 dark:text-red-400">
                    {errors.confirmPassword.message}
                  </p>
                )}
              </div>
            )}
          </div>

          {/* Botón de envío */}
          <button
            type="submit"
            disabled={isLoading}
            className="w-full btn-primary py-3 text-lg font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? (
              <div className="flex items-center justify-center">
                <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white mr-2"></div>
                {isFirstTime ? 'Configurando...' : 'Iniciando...'}
              </div>
            ) : (
              isFirstTime ? 'Configurar Alohopass' : 'Iniciar Sesión'
            )}
          </button>

          {/* Botón de restablecer (solo cuando no es primera vez) */}
          {!isFirstTime && (
            <div className="space-y-3">
              <div className="text-center">
                <span className="text-sm text-gray-500 dark:text-gray-400">
                  ¿Olvidaste tu contraseña maestra?
                </span>
              </div>
              <button
                type="button"
                onClick={() => {
                  toast.error('Funcionalidad de recuperación en desarrollo. Por ahora, puedes eliminar la base de datos para empezar de nuevo.');
                }}
                className="w-full btn-secondary py-2 text-sm"
              >
                Restablecer Contraseña
              </button>
            </div>
          )}
        </form>

        {/* Información adicional */}
        <div className="text-center">
          <div className="flex items-center justify-center space-x-2 text-sm text-gray-500 dark:text-gray-400">
            <Sparkles className="h-4 w-4" />
            <span>Tu contraseña maestra nunca se almacena en texto plano</span>
          </div>
        </div>

        {/* Footer */}
        <div className="text-center text-xs text-gray-400">
          <p>Alohopass - Gestor de Contraseñas Seguro</p>
          <p className="mt-1">Inspirado en el encantamiento Alohomora de Harry Potter</p>
        </div>
      </div>
    </div>
  )
}

export default LoginPage 