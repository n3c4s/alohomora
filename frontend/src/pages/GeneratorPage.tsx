import { useState } from 'react'
import { Copy, RefreshCw, Check, Eye, EyeOff } from 'lucide-react'
import { usePasswordStore, PasswordGenerationRequest } from '../stores/passwordStore'
import toast from 'react-hot-toast'

const GeneratorPage = () => {
  const [generatedPassword, setGeneratedPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [copied, setCopied] = useState(false)
  const [strength, setStrength] = useState<any>(null)
  
  const [settings, setSettings] = useState<PasswordGenerationRequest>({
    length: 16,
    include_uppercase: true,
    include_lowercase: true,
    include_numbers: true,
    include_symbols: true,
    exclude_similar: true,
  })

  const { generatePassword, checkPasswordStrength } = usePasswordStore()

  const handleGenerate = async () => {
    const password = await generatePassword(settings)
    if (password) {
      setGeneratedPassword(password)
      setCopied(false)
      
      // Verificar fortaleza
      const strengthResult = await checkPasswordStrength(password)
      if (strengthResult) {
        setStrength(strengthResult)
      }
    }
  }

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(generatedPassword)
      setCopied(true)
      toast.success('Contraseña copiada al portapapeles')
      
      setTimeout(() => setCopied(false), 2000)
    } catch (error) {
      toast.error('Error al copiar la contraseña')
    }
  }

  const getStrengthColor = (score: number) => {
    if (score >= 80) return 'text-green-600 bg-green-100 dark:bg-green-900/20'
    if (score >= 60) return 'text-yellow-600 bg-yellow-100 dark:bg-yellow-900/20'
    if (score >= 40) return 'text-orange-600 bg-orange-100 dark:bg-orange-900/20'
    return 'text-red-600 bg-red-100 dark:bg-red-900/20'
  }

  const getStrengthText = (score: number) => {
    if (score >= 80) return 'Muy Fuerte'
    if (score >= 60) return 'Fuerte'
    if (score >= 40) return 'Media'
    return 'Débil'
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Generador de Contraseñas
        </h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Crea contraseñas seguras y únicas para todas tus cuentas
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Configuración */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
            Configuración
          </h3>
          
          <div className="space-y-4">
            {/* Longitud */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Longitud: {settings.length} caracteres
              </label>
              <input
                type="range"
                min="8"
                max="64"
                value={settings.length}
                onChange={(e) => setSettings(prev => ({ ...prev, length: parseInt(e.target.value) }))}
                className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
              />
            </div>

            {/* Opciones de caracteres */}
            <div className="space-y-3">
              <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                Tipos de caracteres:
              </label>
              
              <div className="space-y-2">
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={settings.include_uppercase}
                    onChange={(e) => setSettings(prev => ({ ...prev, include_uppercase: e.target.checked }))}
                    className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                  />
                  <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                    Letras mayúsculas (A-Z)
                  </span>
                </label>
                
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={settings.include_lowercase}
                    onChange={(e) => setSettings(prev => ({ ...prev, include_lowercase: e.target.checked }))}
                    className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                  />
                  <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                    Letras minúsculas (a-z)
                  </span>
                </label>
                
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={settings.include_numbers}
                    onChange={(e) => setSettings(prev => ({ ...prev, include_numbers: e.target.checked }))}
                    className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                  />
                  <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                    Números (0-9)
                  </span>
                </label>
                
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={settings.include_symbols}
                    onChange={(e) => setSettings(prev => ({ ...prev, include_symbols: e.target.checked }))}
                    className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                  />
                  <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                    Símbolos (!@#$%^&*)
                  </span>
                </label>
                
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={settings.exclude_similar}
                    onChange={(e) => setSettings(prev => ({ ...prev, exclude_similar: e.target.checked }))}
                    className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                  />
                  <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                    Excluir caracteres similares (l, 1, I, O, 0)
                  </span>
                </label>
              </div>
            </div>

            {/* Botón generar */}
            <button
              onClick={handleGenerate}
              className="w-full btn-primary flex items-center justify-center"
            >
              <RefreshCw className="h-5 w-5 mr-2" />
              Generar Contraseña
            </button>
          </div>
        </div>

        {/* Contraseña generada */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
            Contraseña Generada
          </h3>
          
          {generatedPassword ? (
            <div className="space-y-4">
              {/* Contraseña */}
              <div className="relative">
                <input
                  type={showPassword ? 'text' : 'password'}
                  value={generatedPassword}
                  readOnly
                  className="input-field pr-20 font-mono text-lg"
                />
                <div className="absolute inset-y-0 right-0 flex items-center pr-2">
                  <button
                    onClick={() => setShowPassword(!showPassword)}
                    className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  >
                    {showPassword ? <EyeOff className="h-5 w-5" /> : <Eye className="h-5 w-5" />}
                  </button>
                  <button
                    onClick={handleCopy}
                    className="ml-2 p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  >
                    {copied ? <Check className="h-5 w-5 text-green-600" /> : <Copy className="h-5 w-5" />}
                  </button>
                </div>
              </div>

              {/* Fortaleza */}
              {strength && (
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      Fortaleza:
                    </span>
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${getStrengthColor(strength.score)}`}>
                      {getStrengthText(strength.score)} ({strength.score}%)
                    </span>
                  </div>
                  
                  {/* Barra de progreso */}
                  <div className="w-full bg-gray-200 rounded-full h-2 dark:bg-gray-700">
                    <div
                      className={`h-2 rounded-full transition-all duration-300 ${
                        strength.score >= 80 ? 'bg-green-500' :
                        strength.score >= 60 ? 'bg-yellow-500' :
                        strength.score >= 40 ? 'bg-orange-500' : 'bg-red-500'
                      }`}
                      style={{ width: `${strength.score}%` }}
                    />
                  </div>

                  {/* Sugerencias */}
                  {strength.suggestions && strength.suggestions.length > 0 && (
                    <div>
                      <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                        Sugerencias:
                      </h4>
                      <ul className="space-y-1">
                        {strength.suggestions.map((suggestion: string, index: number) => (
                          <li key={index} className="text-sm text-gray-600 dark:text-gray-400 flex items-start">
                            <span className="text-primary-600 mr-2">•</span>
                            {suggestion}
                          </li>
                        ))}
                      </ul>
                    </div>
                  )}
                </div>
              )}
            </div>
          ) : (
            <div className="text-center py-12 text-gray-500 dark:text-gray-400">
              <RefreshCw className="h-12 w-12 mx-auto mb-4 text-gray-300" />
              <p>Haz clic en "Generar Contraseña" para crear una nueva contraseña</p>
            </div>
          )}
        </div>
      </div>

      {/* Información de seguridad */}
      <div className="card bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800">
        <h3 className="text-lg font-semibold text-blue-900 dark:text-blue-100 mb-3">
          💡 Consejos de Seguridad
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-blue-800 dark:text-blue-200">
          <div>
            <h4 className="font-medium mb-2">Longitud mínima:</h4>
            <ul className="space-y-1">
              <li>• Mínimo 8 caracteres</li>
              <li>• Recomendado 12+ caracteres</li>
              <li>• Para alta seguridad: 16+ caracteres</li>
            </ul>
          </div>
          <div>
            <h4 className="font-medium mb-2">Complejidad:</h4>
            <ul className="space-y-1">
              <li>• Usa mayúsculas y minúsculas</li>
              <li>• Incluye números y símbolos</li>
              <li>• Evita patrones comunes</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  )
}

export default GeneratorPage 