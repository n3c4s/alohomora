import { create } from 'zustand'
import { invoke } from '@tauri-apps/api/tauri'

interface RecoveryState {
  recoveryKey: string | null
  isLoading: boolean
  error: string | null
  
  // Acciones
  generateRecoveryKey: (password: string) => Promise<string | null>
  resetWithRecoveryKey: (recoveryKey: string, newPassword: string) => Promise<boolean>
  clearError: () => void
}

export const useRecoveryStore = create<RecoveryState>((set, get) => ({
  recoveryKey: null,
  isLoading: false,
  error: null,
  
  generateRecoveryKey: async (password: string) => {
    set({ isLoading: true, error: null })
    
    try {
      const recoveryKey = await invoke('generate_recovery_key', { password })
      set({ recoveryKey: recoveryKey as string, isLoading: false })
      return recoveryKey as string
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al generar clave de recuperación'
      set({ error: errorMessage, isLoading: false })
      return null
    }
  },
  
  resetWithRecoveryKey: async (recoveryKey: string, newPassword: string) => {
    set({ isLoading: true, error: null })
    
    try {
      await invoke('reset_master_password_with_recovery', { 
        recoveryKey, 
        newPassword 
      })
      set({ isLoading: false })
      return true
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Error al restablecer contraseña'
      set({ error: errorMessage, isLoading: false })
      return false
    }
  },
  
  clearError: () => {
    set({ error: null })
  },
})) 