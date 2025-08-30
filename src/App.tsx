import { Routes, Route } from 'react-router-dom'
import { useEffect } from 'react'
import { useAuthStore } from './stores/authStore'
import Layout from './components/Layout'
import LoginPage from './pages/LoginPage'
import DashboardPage from './pages/DashboardPage'
import PasswordsPage from './pages/PasswordsPage'
import GeneratorPage from './pages/GeneratorPage'
import SettingsPage from './pages/SettingsPage'
import SyncPage from './pages/SyncPage'
import { listen } from '@tauri-apps/api/event'

function App() {
  const { isAuthenticated, checkAuthStatus } = useAuthStore()

  useEffect(() => {
    // Verificar estado de autenticación al cargar
    checkAuthStatus()
    
    // Escuchar eventos de Tauri
    const unlisten = listen('app-ready', () => {
      console.log('Alohopass está listo!')
    })

    return () => {
      unlisten.then(f => f())
    }
  }, [checkAuthStatus])

  if (!isAuthenticated) {
    return <LoginPage />
  }

  return (
    <Layout>
      <Routes>
        <Route path="/" element={<DashboardPage />} />
        <Route path="/passwords" element={<PasswordsPage />} />
        <Route path="/generator" element={<GeneratorPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/sync" element={<SyncPage />} />
      </Routes>
    </Layout>
  )
}

export default App 