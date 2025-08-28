import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/tauri'

export interface AutocompleteSuggestion {
  title: string
  username: string
  password: string
}

interface AutocompleteState {
  suggestions: AutocompleteSuggestion[]
  isLoading: boolean
  error: string | null
  
  // Acciones
  getSuggestions: (url: string) => Promise<void>
  saveAutocompleteData: (url: string, username: string, password: string) => Promise<boolean>
  clearSuggestions: () => void
  clearError: () => void
}

export const useAutocompleteStore = create<AutocompleteState>((set, get) => ({
  suggestions: [],
  isLoading: false,
  error: null,
  
  getSuggestions: async (url: string) => {
    if (!url.trim()) {
      set({ suggestions: [] })
      return
    }
    
    set({ isLoading: true, error: null })
    
    try {
      const suggestions = await invoke('get_autocomplete_suggestions', { url })
      set({ suggestions, isLoading: false })
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al obtener sugerencias'
      set({ error: errorMessage, isLoading: false })
    }
  },
  
  saveAutocompleteData: async (url: string, username: string, password: string) => {
    try {
      await invoke('save_autocomplete_data', { url, username, password })
      return true
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al guardar datos'
      set({ error: errorMessage })
      return false
    }
  },
  
  clearSuggestions: () => {
    set({ suggestions: [] })
  },
  
  clearError: () => {
    set({ error: null })
  },
})) 