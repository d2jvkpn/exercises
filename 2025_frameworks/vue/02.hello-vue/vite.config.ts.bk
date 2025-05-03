import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import tailwindcss from "@tailwindcss/vite"

import path from 'path';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    tailwindcss(),
    vue(),
    vueDevTools(),
  ],

  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },

    // '@': path.resolve(__dirname, 'src'),
    // extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue'],
  },

  css: {
    preprocessorOptions: {
      css: {
        preprocessorOptions: {
          scss: {
            additionalData: `@import "@/assets/scss/base.scss";`,
            javascriptEnabled: true,
          },
        },
      },
    },
  },

  envDir: './src/',
})
