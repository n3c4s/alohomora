import { useState, useEffect, useRef } from 'react'
import { Eye, EyeOff, Copy, Check, Plus } from 'lucide-react'
import { useAutocompleteStore, AutocompleteSuggestion } from '../stores/autocompleteStore'
import { usePasswordStore } from '../stores/passwordStore'
import toast from 'react-hot-toast'

interface BrowserAutocompleteProps {
  url: string
  onSuggestionSelect: (suggestion: AutocompleteSuggestion) => void
}

const BrowserAutocomplete = ({ url, onSuggestionSelect }: BrowserAutocompleteProps) => {
  const [isVisible, setIsVisible] = useState(false)
  const [showPasswords, setShowPasswords] = useState<Set<string>>(new Set())
  const [copiedId, setCopiedId] = useState<string | null>(null)
  const [showSaveForm, setShowSaveForm] = useState(false)
  const [saveData, setSaveData] = useState({
    title: '',
    username: '',
    password: '',
    notes: '',
  })
  
  const { suggestions, isLoading, getSuggestions, saveAutocompleteData } = useAutocompleteStore()
  const { createPassword } = usePasswordStore()
  
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (url) {
      getSuggestions(url)
    }
  }, [url, getSuggestions])

  useEffect(() => {
    // Cerrar al hacer clic fuera
    const handleClickOutside = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsVisible(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  const handleSuggestionClick = (suggestion: AutocompleteSuggestion) => {
    onSuggestionSelect(suggestion)
    setIsVisible(false)
    toast.success('Datos autocompletados')
  }

  const handleCopy = async (text: string, id: string) => {
    try {
      await navigator.clipboard.writeText(text)
      setCopiedId(id)
      toast.success('Copiado al portapapeles')
      
      setTimeout(() => setCopiedId(null), 2000)
    } catch (error) {
      toast.error('Error al copiar')
    }
  }

  const togglePasswordVisibility = (id: string) => {
    const newSet = new Set(showPasswords)
    if (newSet.has(id)) {
      newSet.delete(id)
    } else {
      newSet.add(id)
    }
    setShowPasswords(newSet)
  }

  const handleSavePassword = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!saveData.title || !saveData.username || !saveData.password) {
      toast.error('Completa todos los campos requeridos')
      return
    }
    
    try {
      const id = await createPassword({
        title: saveData.title,
        username: saveData.username,
        password: saveData.password,
        url: url,
        notes: saveData.notes,
        category_id: '',
        tags: [],
      })
      
      if (id) {
        toast.success('Contraseña guardada en Alohopass')
        setShowSaveForm(false)
        setSaveData({ title: '', username: '', password: '', notes: '' })
        
        // Recargar sugerencias
        getSuggestions(url)
      }
    } catch (error) {
      toast.error('Error al guardar la contraseña')
    }
  }

  const extractDomainName = (url: string) => {
    try {
      const domain = new URL(url).hostname
      return domain.replace('www.', '')
    } catch {
      return url
    }
  }

  if (!url || suggestions.length === 0) {
    return null
  }

  return (
    <div ref={containerRef} className="relative">
      {/* Botón para mostrar/ocultar */}
      <button
        onClick={() => setIsVisible(!isVisible)}
        className="fixed bottom-4 right-4 z-50 bg-primary-600 hover:bg-primary-700 text-white p-3 rounded-full shadow-lg transition-all duration-200 hover:scale-110"
        title="Alohopass - Autocompletado"
      >
        <div className="relative">
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
          </svg>
          {suggestions.length > 0 && (
            <span className="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-5 w-5 flex items-center justify-center">
              {suggestions.length}
            </span>
          )}
        </div>
      </button>

      {/* Panel de autocompletado */}
      {isVisible && (
        <div className="fixed bottom-20 right-4 z-50 w-96 max-h-96 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
          {/* Header */}
          <div className="bg-primary-600 text-white p-4">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-semibold">Alohopass</h3>
                <p className="text-sm text-primary-100">
                  {extractDomainName(url)}
                </p>
              </div>
              <button
                onClick={() => setShowSaveForm(!showSaveForm)}
                className="p-2 hover:bg-primary-700 rounded-lg transition-colors"
                title="Guardar nueva contraseña"
              >
                <Plus className="h-5 w-5" />
              </button>
            </div>
          </div>

          {/* Contenido */}
          <div className="max-h-80 overflow-y-auto">
            {showSaveForm ? (
              /* Formulario para guardar nueva contraseña */
              <div className="p-4">
                <h4 className="font-medium text-gray-900 dark:text-white mb-3">
                  Guardar Nueva Contraseña
                </h4>
                
                <form onSubmit={handleSavePassword} className="space-y-3">
                  <div>
                    <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Título *
                    </label>
                    <input
                      type="text"
                      required
                      value={saveData.title}
                      onChange={(e) => setSaveData(prev => ({ ...prev, title: e.target.value }))}
                      className="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                      placeholder="Ej: Gmail, GitHub"
                    />
                  </div>
                  
                  <div>
                    <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Usuario *
                    </label>
                    <input
                      type="text"
                      required
                      value={saveData.username}
                      onChange={(e) => setSaveData(prev => ({ ...prev, username: e.target.value }))}
                      className="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                      placeholder="usuario@email.com"
                    />
                  </div>
                  
                  <div>
                    <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Contraseña *
                    </label>
                    <input
                      type="text"
                      required
                      value={saveData.password}
                      onChange={(e) => setSaveData(prev => ({ ...prev, password: e.target.value }))}
                      className="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                      placeholder="Tu contraseña"
                    />
                  </div>
                  
                  <div>
                    <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                      Notas
                    </label>
                    <textarea
                      value={saveData.notes}
                      onChange={(e) => setSaveData(prev => ({ ...prev, notes: e.target.value }))}
                      className="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                      rows={2}
                      placeholder="Notas adicionales..."
                    />
                  </div>
                  
                  <div className="flex space-x-2 pt-2">
                    <button
                      type="button"
                      onClick={() => setShowSaveForm(false)}
                      className="flex-1 px-3 py-2 text-sm bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500 transition-colors"
                    >
                      Cancelar
                    </button>
                    <button
                      type="submit"
                      className="flex-1 px-3 py-2 text-sm bg-primary-600 text-white rounded-md hover:bg-primary-700 transition-colors"
                    >
                      Guardar
                    </button>
                  </div>
                </form>
              </div>
            ) : (
              /* Lista de sugerencias */
              <div className="p-4">
                <h4 className="font-medium text-gray-900 dark:text-white mb-3">
                  Contraseñas Guardadas ({suggestions.length})
                </h4>
                
                {isLoading ? (
                  <div className="text-center py-4">
                    <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-primary-600 mx-auto"></div>
                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-2">
                      Buscando contraseñas...
                    </p>
                  </div>
                ) : (
                  <div className="space-y-3">
                    {suggestions.map((suggestion, index) => (
                      <div
                        key={index}
                        className="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors cursor-pointer"
                        onClick={() => handleSuggestionClick(suggestion)}
                      >
                        <div className="flex items-start justify-between mb-2">
                          <h5 className="font-medium text-gray-900 dark:text-white text-sm">
                            {suggestion.title}
                          </h5>
                          <button
                            onClick={(e) => {
                              e.stopPropagation()
                              handleCopy(suggestion.password, `suggestion-${index}`)
                            }}
                            className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                          >
                            {copiedId === `suggestion-${index}` ? (
                              <Check className="h-4 w-4 text-green-600" />
                            ) : (
                              <Copy className="h-4 w-4" />
                            )}
                          </button>
                        </div>
                        
                        <div className="space-y-2">
                          <div className="flex items-center justify-between">
                            <span className="text-xs text-gray-600 dark:text-gray-400">Usuario:</span>
                            <span className="text-sm text-gray-900 dark:text-white font-mono">
                              {suggestion.username}
                            </span>
                          </div>
                          
                          <div className="flex items-center justify-between">
                            <span className="text-xs text-gray-600 dark:text-gray-400">Contraseña:</span>
                            <div className="flex items-center space-x-1">
                              <input
                                type={showPasswords.has(`suggestion-${index}`) ? 'text' : 'password'}
                                value={suggestion.password}
                                readOnly
                                className="w-24 text-sm bg-transparent border-none text-gray-900 dark:text-white font-mono"
                                onClick={(e) => e.stopPropagation()}
                              />
                              <button
                                onClick={(e) => {
                                  e.stopPropagation()
                                  togglePasswordVisibility(`suggestion-${index}`)
                                }}
                                className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                              >
                                {showPasswords.has(`suggestion-${index}`) ? (
                                  <EyeOff className="h-3 w-3" />
                                ) : (
                                  <Eye className="h-3 w-3" />
                                )}
                              </button>
                            </div>
                          </div>
                        </div>
                        
                        <div className="mt-2 pt-2 border-t border-gray-200 dark:border-gray-600">
                          <p className="text-xs text-gray-500 dark:text-gray-400 text-center">
                            Haz clic para autocompletar
                          </p>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  )
}

export default BrowserAutocomplete 