import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { usePasswordStore } from '../stores/passwordStore'
import { Eye, EyeOff, Copy, RefreshCw } from 'lucide-react'
import toast from 'react-hot-toast'

interface BrowserAutofillProps {
  url: string
  username?: string
  onFill: (username: string, password: string) => void
}

const BrowserAutofill = ({ url, username, onFill }: BrowserAutofillProps) => {
  const [suggestions, setSuggestions] = useState<any[]>([])
  const [selectedPassword, setSelectedPassword] = useState<any>(null)
  const [showPassword, setShowPassword] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  
  const { generatePassword } = usePasswordStore()

  useEffect(() => {
    if (url) {
      loadSuggestions()
    }
  }, [url])

  const loadSuggestions = async () => {
    setIsLoading(true)
    try {
      const results = await invoke('get_autocomplete_suggestions', { 
        request: { url, username: username || '' } 
      })
      setSuggestions(results as any[])
    } catch (error) {
      console.error('Error cargando sugerencias:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const handleGenerateNew = async () => {
    try {
      const newPassword = await generatePassword({
        length: 16,
        include_uppercase: true,
        include_lowercase: true,
        include_numbers: true,
        include_symbols: true,
        exclude_similar: true,
      })
      
      if (newPassword) {
        onFill(username || '', newPassword)
        toast.success('Nueva contraseña generada y aplicada')
      }
    } catch (error) {
      toast.error('Error generando nueva contraseña')
    }
  }

  const handleFill = (suggestion: any) => {
    setSelectedPassword(suggestion)
    onFill(suggestion.username, suggestion.password)
    toast.success('Credenciales aplicadas')
  }

  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text)
      toast.success('Copiado al portapapeles')
    } catch (error) {
      toast.error('Error al copiar')
    }
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-4">
        <RefreshCw className="h-5 w-5 animate-spin text-primary-600" />
        <span className="ml-2 text-sm text-gray-600">Buscando credenciales...</span>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
          Credenciales para {url}
        </h3>
        <button
          onClick={handleGenerateNew}
          className="btn btn-primary btn-sm"
        >
          Generar Nueva
        </button>
      </div>

      {suggestions.length > 0 ? (
        <div className="space-y-3">
          {suggestions.map((suggestion, index) => (
            <div
              key={index}
              className="card p-4 hover:shadow-md transition-shadow cursor-pointer"
              onClick={() => handleFill(suggestion)}
            >
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-3">
                    <div className="w-8 h-8 bg-primary-100 dark:bg-primary-900/20 rounded-full flex items-center justify-center">
                      <span className="text-primary-600 font-semibold text-sm">
                        {suggestion.username.charAt(0).toUpperCase()}
                      </span>
                    </div>
                    <div>
                      <p className="font-medium text-gray-900 dark:text-white">
                        {suggestion.username}
                      </p>
                      <p className="text-sm text-gray-500 dark:text-gray-400">
                        {suggestion.title || 'Sin título'}
                      </p>
                    </div>
                  </div>
                </div>
                
                <div className="flex items-center space-x-2">
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      copyToClipboard(suggestion.username)
                    }}
                    className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  >
                    <Copy className="h-4 w-4" />
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation()
                      copyToClipboard(suggestion.password)
                    }}
                    className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  >
                    <Copy className="h-4 w-4" />
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="text-center py-8 text-gray-500 dark:text-gray-400">
          <p>No se encontraron credenciales guardadas para este sitio</p>
          <button
            onClick={handleGenerateNew}
            className="btn btn-primary mt-3"
          >
            Generar Nueva Contraseña
          </button>
        </div>
      )}
    </div>
  )
}

export default BrowserAutofill 