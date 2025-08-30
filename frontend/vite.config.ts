import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5174, // Cambiar puerto para evitar conflictos
    host: true
  },
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
}) 