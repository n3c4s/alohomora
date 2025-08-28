import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/tauri'

export interface PasswordEntry {
  id: string
  title: string
  username: string
  password: string
  url?: string
  notes?: string
  category_id?: string
  tags: string[]
  created_at: string
  updated_at: string
  last_used?: string
}

export interface CreatePasswordRequest {
  title: string
  username: string
  password: string
  url?: string
  notes?: string
  category_id?: string
  tags: string[]
}

export interface PasswordGenerationRequest {
  length: number
  include_uppercase: boolean
  include_lowercase: boolean
  include_numbers: boolean
  include_symbols: boolean
  exclude_similar: boolean
}

interface PasswordState {
  passwords: PasswordEntry[]
  isLoading: boolean
  error: string | null
  
  // Acciones
  fetchPasswords: () => Promise<void>
  createPassword: (request: CreatePasswordRequest) => Promise<string | null>
  updatePassword: (id: string, updates: Partial<CreatePasswordRequest>) => Promise<boolean>
  deletePassword: (id: string) => Promise<boolean>
  generatePassword: (request: PasswordGenerationRequest) => Promise<string | null>
  checkPasswordStrength: (password: string) => Promise<any>
  searchPasswords: (query: string) => Promise<void>
  clearError: () => void
}

export const usePasswordStore = create<PasswordState>((set, get) => ({
  passwords: [],
  isLoading: false,
  error: null,
  
  fetchPasswords: async () => {
    set({ isLoading: true, error: null })
    
    try {
      const passwords = await invoke('get_password_entries')
      set({ passwords, isLoading: false })
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al obtener contraseñas'
      set({ error: errorMessage, isLoading: false })
    }
  },
  
  createPassword: async (request: CreatePasswordRequest) => {
    set({ isLoading: true, error: null })
    
    try {
      const id = await invoke('create_password_entry', { request })
      await get().fetchPasswords() // Recargar lista
      set({ isLoading: false })
      return id
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al crear contraseña'
      set({ error: errorMessage, isLoading: false })
      return null
    }
  },
  
  updatePassword: async (id: string, updates: Partial<CreatePasswordRequest>) => {
    set({ isLoading: true, error: null })
    
    try {
      await invoke('update_password_entry', { 
        request: { id, ...updates } 
      })
      await get().fetchPasswords() // Recargar lista
      set({ isLoading: false })
      return true
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al actualizar contraseña'
      set({ error: errorMessage, isLoading: false })
      return false
    }
  },
  
  deletePassword: async (id: string) => {
    set({ isLoading: true, error: null })
    
    try {
      await invoke('delete_password_entry', { id })
      await get().fetchPasswords() // Recargar lista
      set({ isLoading: false })
      return true
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al eliminar contraseña'
      set({ error: errorMessage, isLoading: false })
      return false
    }
  },
  
  generatePassword: async (request: PasswordGenerationRequest) => {
    try {
      const password = await invoke('generate_password', { request })
      return password
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al generar contraseña'
      set({ error: errorMessage })
      return null
    }
  },
  
  checkPasswordStrength: async (password: string) => {
    try {
      const strength = await invoke('check_password_strength', { password })
      return strength
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al verificar fortaleza'
      set({ error: errorMessage })
      return null
    }
  },
  
  searchPasswords: async (query: string) => {
    if (!query.trim()) {
      await get().fetchPasswords()
      return
    }
    
    set({ isLoading: true, error: null })
    
    try {
      const results = await invoke('search_passwords', { 
        request: { query, category_id: null, tags: [] } 
      })
      set({ passwords: results, isLoading: false })
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error en búsqueda'
      set({ error: errorMessage, isLoading: false })
    }
  },
  
  clearError: () => {
    set({ error: null })
  },
})) 