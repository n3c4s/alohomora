import { Routes, Route } from 'react-router-dom'
import { useEffect } from 'react'
import { useAuthStore } from './stores/authStore'
import Layout from './components/Layout'
import LoginPage from './pages/LoginPage'
import DashboardPage from './pages/DashboardPage'
import PasswordsPage from './pages/PasswordsPage'
import GeneratorPage from './pages/GeneratorPage'
import SettingsPage from './pages/SettingsPage'
import { listen } from '@tauri-apps/api/event'

function App() {
  const { isAuthenticated, isInitialized, checkDatabaseStatus, resetAuthenticationOnAppStart } = useAuthStore()

  useEffect(() => {
    // CRÍTICO: Resetear autenticación al abrir la app
    console.log('🔄 App: Iniciando aplicación...');
    resetAuthenticationOnAppStart();
    
    // Verificar estado de la base de datos
    checkDatabaseStatus();
    
    // Escuchar eventos de Tauri
    const unlisten = listen('app-ready', () => {
      console.log('Alohopass está listo!')
    })

    return () => {
      unlisten.then(f => f())
    }
  }, [checkDatabaseStatus, resetAuthenticationOnAppStart])

  // Solo mostrar LoginPage si la base de datos está inicializada pero el usuario no está autenticado
  if (isInitialized && !isAuthenticated) {
    console.log('🔄 App: Base de datos inicializada pero usuario no autenticado, mostrando LoginPage');
    return <LoginPage />
  }

  // Si la base de datos no está inicializada, mostrar LoginPage para crear contraseña maestra
  if (!isInitialized) {
    console.log('🔄 App: Base de datos no inicializada, mostrando LoginPage para crear contraseña maestra');
    return <LoginPage />
  }

  // Si está autenticado, mostrar la aplicación principal
  console.log('🔄 App: Usuario autenticado, mostrando aplicación principal');
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<DashboardPage />} />
        <Route path="/passwords" element={<PasswordsPage />} />
        <Route path="/generator" element={<GeneratorPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Routes>
    </Layout>
  )
}

export default App 