import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface AuthState {
  isAuthenticated: boolean
  masterPasswordHash: string | null
  salt: Uint8Array | null
  checkAuthStatus: () => void
  login: (password: string) => Promise<boolean>
  logout: () => void
  setMasterPassword: (hash: string, salt: Uint8Array) => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      isAuthenticated: false,
      masterPasswordHash: null,
      salt: null,
      
      checkAuthStatus: () => {
        const { masterPasswordHash, salt } = get()
        if (masterPasswordHash && salt) {
          set({ isAuthenticated: true })
        }
      },
      
      login: async (password: string) => {
        const { masterPasswordHash, salt } = get()
        
        if (!masterPasswordHash || !salt) {
          // Primera vez - crear nueva contraseña maestra
          // Aquí implementarías la lógica de Tauri para crear la clave
          set({ isAuthenticated: true })
          return true
        }
        
        // Verificar contraseña existente
        // Aquí implementarías la lógica de Tauri para verificar
        try {
          // Simular verificación por ahora
          set({ isAuthenticated: true })
          return true
        } catch (error) {
          console.error('Error en login:', error)
          return false
        }
      },
      
      logout: () => {
        set({ isAuthenticated: false })
      },
      
      setMasterPassword: (hash: string, salt: Uint8Array) => {
        set({ masterPasswordHash: hash, salt: salt })
      },
    }),
    {
      name: 'alohopass-auth',
      partialize: (state) => ({
        masterPasswordHash: state.masterPasswordHash,
        salt: state.salt,
      }),
    }
  )
) 