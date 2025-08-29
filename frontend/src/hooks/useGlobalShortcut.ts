import { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

export const useGlobalShortcut = () => {
  useEffect(() => {
    const handleKeyDown = async (event: KeyboardEvent) => {
      // Ctrl+Shift+A para activar autofill
      if (event.ctrlKey && event.shiftKey && event.key === 'A') {
        event.preventDefault()
        
        try {
          // Obtener la URL activa del navegador (esto requeriría permisos adicionales)
          const activeUrl = await invoke('get_active_browser_url')
          console.log('Activando autofill para:', activeUrl)
          
          // Aquí podrías abrir un modal o mostrar las sugerencias
          // Por ahora solo mostramos un mensaje
          alert('Autofill activado! (Funcionalidad en desarrollo)')
        } catch (error) {
          console.error('Error activando autofill:', error)
        }
      }
      
      // Ctrl+Shift+G para generar nueva contraseña
      if (event.ctrlKey && event.shiftKey && event.key === 'G') {
        event.preventDefault()
        console.log('Generando nueva contraseña...')
        // Aquí podrías abrir el generador
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    
    return () => {
      document.removeEventListener('keydown', handleKeyDown)
    }
  }, [])
} 