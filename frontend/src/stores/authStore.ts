import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { invoke } from '@tauri-apps/api/tauri'

interface AuthState {
  isAuthenticated: boolean
  isInitialized: boolean
  isLoading: boolean
  error: string | null
  
  // Acciones
  initializeMasterPassword: (password: string) => Promise<boolean>
  verifyMasterPassword: (password: string) => Promise<boolean>
  logout: () => void
  clearError: () => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      isAuthenticated: false,
      isInitialized: false,
      isLoading: false,
      error: null,
      
      initializeMasterPassword: async (password: string) => {
        set({ isLoading: true, error: null })
        
        try {
          await invoke('initialize_master_password', { password })
          set({ 
            isAuthenticated: true, 
            isInitialized: true, 
            isLoading: false 
          })
          return true
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : 'Error desconocido'
          set({ 
            error: errorMessage, 
            isLoading: false 
          })
          return false
        }
      },
      
      verifyMasterPassword: async (password: string) => {
        set({ isLoading: true, error: null })
        
        try {
          const isValid = await invoke('verify_master_password', { password })
          
          if (isValid) {
            set({ 
              isAuthenticated: true, 
              isInitialized: true, 
              isLoading: false 
            })
            return true
          } else {
            set({ 
              error: 'Contraseña incorrecta', 
              isLoading: false 
            })
            return false
          }
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : 'Error desconocido'
          set({ 
            error: errorMessage, 
            isLoading: false 
          })
          return false
        }
      },
      
      logout: () => {
        set({ 
          isAuthenticated: false, 
          isInitialized: false,
          error: null 
        })
      },
      
      clearError: () => {
        set({ error: null })
      },
    }),
    {
      name: 'alohopass-auth',
      partialize: (state) => ({
        isInitialized: state.isInitialized,
      }),
    }
  )
) 