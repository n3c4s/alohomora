import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { invoke } from '@tauri-apps/api/tauri'

interface AuthState {
  isAuthenticated: boolean
  isInitialized: boolean
  isLoading: boolean
  error: string | null
  checkDatabaseStatus: () => Promise<void>
  initializeMasterPassword: (password: string) => Promise<boolean>
  verifyMasterPassword: (password: string) => Promise<boolean>
  changeMasterPassword: (oldPassword: string, newPassword: string) => Promise<boolean>
  logout: () => void
  clearError: () => void
  clearPersistedState: () => void
  resetAuthenticationOnAppStart: () => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      isAuthenticated: false, // Siempre false al abrir la app
      isInitialized: false,
      isLoading: false,
      error: null,
      
      initializeMasterPassword: async (password: string) => {
        console.log('🔄 Frontend: Iniciando creación de contraseña maestra...');
        set({ isLoading: true, error: null })
        
        try {
          console.log('🔄 Frontend: Llamando a initialize_master_password...');
          await invoke('initialize_master_password', { password })
          console.log('✅ Frontend: Contraseña maestra creada exitosamente');
          set({ 
            isAuthenticated: true, // Solo true durante esta sesión
            isInitialized: true, 
            isLoading: false 
          })
          return true
        } catch (error) {
          console.error('❌ Frontend: Error al crear contraseña maestra:', error);
          const errorMessage = error instanceof Error ? error.message : 'Error desconocido'
          console.error('❌ Frontend: Mensaje de error:', errorMessage);
          set({ 
            error: errorMessage, 
            isLoading: false 
          })
          return false
        }
      },
      
      verifyMasterPassword: async (password: string) => {
        console.log('🔄 Frontend: verifyMasterPassword iniciado con contraseña de longitud:', password.length);
        set({ isLoading: true, error: null })
        
        try {
          console.log('🔄 Frontend: Llamando a invoke verify_master_password...');
          const isValid = await invoke('verify_master_password', { password })
          console.log('✅ Frontend: Respuesta de verify_master_password recibida:', isValid);
          
          if (isValid) {
            console.log('✅ Frontend: Contraseña válida, estableciendo isAuthenticated: true');
            set({ 
              isAuthenticated: true, // Solo true durante esta sesión
              isInitialized: true, 
              isLoading: false 
            })
            return true
          } else {
            console.log('❌ Frontend: Contraseña incorrecta');
            set({ 
              error: 'Contraseña incorrecta', 
              isLoading: false 
            })
            return false
          }
        } catch (error) {
          console.error('❌ Frontend: Error en verifyMasterPassword:', error);
          const errorMessage = error instanceof Error ? error.message : 'Error desconocido'
          console.error('❌ Frontend: Mensaje de error:', errorMessage);
          set({ 
            error: errorMessage, 
            isLoading: false 
          })
          return false
        }
      },
      
      changeMasterPassword: async (oldPassword: string, newPassword: string) => {
        set({ isLoading: true, error: null })
        
        try {
          await invoke('change_master_password', { oldPassword, newPassword })
          set({ isLoading: false })
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

      clearPersistedState: () => {
        console.log('🔄 Frontend: Limpiando estado persistente...');
        set({ 
          isAuthenticated: false, 
          isInitialized: false, 
          isLoading: false, 
          error: null 
        });
        console.log('✅ Frontend: Estado persistente limpiado');
      },

      checkDatabaseStatus: async () => {
        console.log('🔄 Frontend: Iniciando verificación de base de datos...');
        try {
          console.log('🔄 Frontend: Llamando a check_database_status...');
          const isInitialized = await invoke('check_database_status')
          console.log('✅ Frontend: Respuesta recibida:', isInitialized);
          console.log('🔄 Frontend: Estado actual antes de actualizar:', get().isInitialized);
          
          // IMPORTANTE: Solo actualizar isInitialized, NUNCA isAuthenticated
          // isAuthenticated debe ser false al abrir la app para forzar login
          set({ 
            isInitialized: isInitialized as boolean,
            isAuthenticated: false // CRÍTICO: Siempre false al abrir la app
          })
          console.log('✅ Frontend: Estado actualizado - isInitialized:', get().isInitialized, 'isAuthenticated:', get().isAuthenticated);
        } catch (error) {
          console.error('❌ Frontend: Error al verificar estado de la base de datos:', error)
          set({ 
            isInitialized: false,
            isAuthenticated: false 
          })
        }
      },
      
      // Función para resetear el estado de autenticación al abrir la app
      resetAuthenticationOnAppStart: () => {
        console.log('🔄 Frontend: Reseteando estado de autenticación al abrir la app...');
        set({ 
          isAuthenticated: false, // CRÍTICO: Siempre false al abrir la app
          isLoading: false, 
          error: null 
        })
        console.log('✅ Frontend: Estado de autenticación reseteado - isAuthenticated:', get().isAuthenticated);
      }
    }),
    {
      name: 'alohopass-auth',
      partialize: (state) => ({
        isInitialized: state.isInitialized,
        // CRÍTICO: NO persistir isAuthenticated - siempre requerir login
      }),
    }
  )
) 