import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
// @ts-ignore
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      'tailwind.config.cjs': path.resolve(__dirname, 'tailwind.config.cjs'),
      '@components': path.resolve(__dirname, './src/lib'),
      '@styles': path.resolve(__dirname, './src/styles'),
      '@routes': path.resolve(__dirname, './src/routes'),
      '@utils': path.resolve(__dirname, './src/utils')
    }
  },
  optimizeDeps: {
    include: [
      path.resolve(__dirname, 'tailwind.config.cjs')
    ]
  }
})
