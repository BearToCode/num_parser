import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
// @ts-ignore
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      'tailwind.config.cjs': path.resolve(__dirname, 'tailwind.config.cjs')
    }
  },
  optimizeDeps: {
    include: [
      path.resolve(__dirname, 'tailwind.config.cjs')
    ]
  }
})
