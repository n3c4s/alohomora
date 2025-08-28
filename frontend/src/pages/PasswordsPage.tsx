import { useState, useEffect } from 'react'
import { Plus, Search, Edit, Trash2, Eye, EyeOff, Copy, Check } from 'lucide-react'
import { usePasswordStore, PasswordEntry, CreatePasswordRequest } from '../stores/passwordStore'
import toast from 'react-hot-toast'

const PasswordsPage = () => {
  const [searchQuery, setSearchQuery] = useState('')
  const [showAddModal, setShowAddModal] = useState(false)
  const [editingPassword, setEditingPassword] = useState<PasswordEntry | null>(null)
  const [showPasswords, setShowPasswords] = useState<Set<string>>(new Set())
  const [copiedId, setCopiedId] = useState<string | null>(null)
  
  const { 
    passwords, 
    isLoading, 
    error, 
    fetchPasswords, 
    createPassword, 
    updatePassword, 
    deletePassword, 
    searchPasswords 
  } = usePasswordStore()

  const [formData, setFormData] = useState<CreatePasswordRequest>({
    title: '',
    username: '',
    password: '',
    url: '',
    notes: '',
    category_id: '',
    tags: [],
  })

  useEffect(() => {
    fetchPasswords()
  }, [fetchPasswords])

  useEffect(() => {
    if (error) {
      toast.error(error)
    }
  }, [error])

  const handleSearch = (query: string) => {
    setSearchQuery(query)
    searchPasswords(query)
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (editingPassword) {
      const success = await updatePassword(editingPassword.id, formData)
      if (success) {
        toast.success('Contraseña actualizada correctamente')
        setShowAddModal(false)
        setEditingPassword(null)
        resetForm()
      }
    } else {
      const id = await createPassword(formData)
      if (id) {
        toast.success('Contraseña creada correctamente')
        setShowAddModal(false)
        resetForm()
      }
    }
  }

  const handleEdit = (password: PasswordEntry) => {
    setEditingPassword(password)
    setFormData({
      title: password.title,
      username: password.username,
      password: password.password,
      url: password.url || '',
      notes: password.notes || '',
      category_id: password.category_id || '',
      tags: password.tags,
    })
    setShowAddModal(true)
  }

  const handleDelete = async (id: string) => {
    if (confirm('¿Estás seguro de que quieres eliminar esta contraseña?')) {
      const success = await deletePassword(id)
      if (success) {
        toast.success('Contraseña eliminada correctamente')
      }
    }
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

  const resetForm = () => {
    setFormData({
      title: '',
      username: '',
      password: '',
      url: '',
      notes: '',
      category_id: '',
      tags: [],
    })
  }

  const openAddModal = () => {
    setEditingPassword(null)
    resetForm()
    setShowAddModal(true)
  }

  const closeModal = () => {
    setShowAddModal(false)
    setEditingPassword(null)
    resetForm()
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Contraseñas
          </h1>
          <p className="mt-2 text-gray-600 dark:text-gray-400">
            Gestiona todas tus contraseñas de forma segura
          </p>
        </div>
        <button
          onClick={openAddModal}
          className="btn-primary flex items-center"
        >
          <Plus className="h-5 w-5 mr-2" />
          Nueva Contraseña
        </button>
      </div>

      {/* Barra de búsqueda */}
      <div className="relative">
        <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
        <input
          type="text"
          placeholder="Buscar contraseñas..."
          value={searchQuery}
          onChange={(e) => handleSearch(e.target.value)}
          className="input-field pl-10"
        />
      </div>

      {/* Lista de contraseñas */}
      <div className="card">
        {isLoading ? (
          <div className="text-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600 mx-auto"></div>
            <p className="mt-2 text-gray-600 dark:text-gray-400">Cargando contraseñas...</p>
          </div>
        ) : passwords.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            <p>No hay contraseñas guardadas</p>
            <p className="text-sm">Crea tu primera contraseña para comenzar</p>
          </div>
        ) : (
          <div className="space-y-4">
            {passwords.map((password) => (
              <div
                key={password.id}
                className="border border-gray-200 dark:border-gray-700 rounded-lg p-4 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-3 mb-2">
                      <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                        {password.title}
                      </h3>
                      {password.category_id && (
                        <span className="px-2 py-1 bg-primary-100 dark:bg-primary-900 text-primary-800 dark:text-primary-200 text-xs rounded-full">
                          {password.category_id}
                        </span>
                      )}
                    </div>
                    
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <div>
                        <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                          Usuario:
                        </label>
                        <div className="flex items-center space-x-2 mt-1">
                          <span className="text-gray-900 dark:text-white">
                            {password.username}
                          </span>
                          <button
                            onClick={() => handleCopy(password.username, `user-${password.id}`)}
                            className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                          >
                            {copiedId === `user-${password.id}` ? (
                              <Check className="h-4 w-4 text-green-600" />
                            ) : (
                              <Copy className="h-4 w-4" />
                            )}
                          </button>
                        </div>
                      </div>
                      
                      <div>
                        <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                          Contraseña:
                        </label>
                        <div className="flex items-center space-x-2 mt-1">
                          <input
                            type={showPasswords.has(password.id) ? 'text' : 'password'}
                            value={password.password}
                            readOnly
                            className="flex-1 bg-transparent border-none text-gray-900 dark:text-white font-mono"
                          />
                          <button
                            onClick={() => togglePasswordVisibility(password.id)}
                            className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                          >
                            {showPasswords.has(password.id) ? (
                              <EyeOff className="h-4 w-4" />
                            ) : (
                              <Eye className="h-4 w-4" />
                            )}
                          </button>
                          <button
                            onClick={() => handleCopy(password.password, `pass-${password.id}`)}
                            className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                          >
                            {copiedId === `pass-${password.id}` ? (
                              <Check className="h-4 w-4 text-green-600" />
                            ) : (
                              <Copy className="h-4 w-4" />
                            )}
                          </button>
                        </div>
                      </div>
                    </div>
                    
                    {password.url && (
                      <div className="mt-3">
                        <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                          URL:
                        </label>
                        <a
                          href={password.url}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-primary-600 hover:text-primary-700 text-sm ml-2"
                        >
                          {password.url}
                        </a>
                      </div>
                    )}
                    
                    {password.notes && (
                      <div className="mt-3">
                        <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                          Notas:
                        </label>
                        <p className="text-gray-900 dark:text-white text-sm mt-1">
                          {password.notes}
                        </p>
                      </div>
                    )}
                    
                    {password.tags.length > 0 && (
                      <div className="mt-3">
                        <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                          Etiquetas:
                        </label>
                        <div className="flex flex-wrap gap-2 mt-1">
                          {password.tags.map((tag, index) => (
                            <span
                              key={index}
                              className="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs rounded"
                            >
                              {tag}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                  
                  <div className="flex items-center space-x-2 ml-4">
                    <button
                      onClick={() => handleEdit(password)}
                      className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
                    >
                      <Edit className="h-4 w-4" />
                    </button>
                    <button
                      onClick={() => handleDelete(password.id)}
                      className="p-2 text-red-400 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg"
                    >
                      <Trash2 className="h-4 w-4" />
                    </button>
                  </div>
                </div>
                
                <div className="mt-3 pt-3 border-t border-gray-200 dark:border-gray-700 text-xs text-gray-500 dark:text-gray-400">
                  <span>Creada: {new Date(password.created_at).toLocaleDateString()}</span>
                  {password.last_used && (
                    <span className="ml-4">
                      Último uso: {new Date(password.last_used).toLocaleDateString()}
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Modal para agregar/editar contraseña */}
      {showAddModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg max-w-md w-full max-h-[90vh] overflow-y-auto">
            <div className="p-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
                {editingPassword ? 'Editar Contraseña' : 'Nueva Contraseña'}
              </h3>
              
              <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Título *
                  </label>
                  <input
                    type="text"
                    required
                    value={formData.title}
                    onChange={(e) => setFormData(prev => ({ ...prev, title: e.target.value }))}
                    className="input-field"
                    placeholder="Ej: Gmail, GitHub, Netflix"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Usuario *
                  </label>
                  <input
                    type="text"
                    required
                    value={formData.username}
                    onChange={(e) => setFormData(prev => ({ ...prev, username: e.target.value }))}
                    className="input-field"
                    placeholder="usuario@email.com"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Contraseña *
                  </label>
                  <input
                    type="text"
                    required
                    value={formData.password}
                    onChange={(e) => setFormData(prev => ({ ...prev, password: e.target.value }))}
                    className="input-field"
                    placeholder="Tu contraseña"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    URL
                  </label>
                  <input
                    type="url"
                    value={formData.url}
                    onChange={(e) => setFormData(prev => ({ ...prev, url: e.target.value }))}
                    className="input-field"
                    placeholder="https://ejemplo.com"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Notas
                  </label>
                  <textarea
                    value={formData.notes}
                    onChange={(e) => setFormData(prev => ({ ...prev, notes: e.target.value }))}
                    className="input-field"
                    rows={3}
                    placeholder="Notas adicionales..."
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Etiquetas
                  </label>
                  <input
                    type="text"
                    value={formData.tags.join(', ')}
                    onChange={(e) => setFormData(prev => ({ 
                      ...prev, 
                      tags: e.target.value.split(',').map(tag => tag.trim()).filter(Boolean)
                    }))}
                    className="input-field"
                    placeholder="trabajo, personal, importante"
                  />
                  <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    Separa las etiquetas con comas
                  </p>
                </div>
                
                <div className="flex space-x-3 pt-4">
                  <button
                    type="button"
                    onClick={closeModal}
                    className="btn-secondary flex-1"
                  >
                    Cancelar
                  </button>
                  <button
                    type="submit"
                    className="btn-primary flex-1"
                  >
                    {editingPassword ? 'Actualizar' : 'Crear'}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default PasswordsPage 