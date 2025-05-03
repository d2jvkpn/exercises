#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


function extract_embeded() {
    key=$1
    sed -n "/^__${key}-0__$/,/^__${key}-1__/p" "$0" | tail -n +2 | head -n -1
}

mv src/App.vue src/App.vue.bk
mv vite.config.ts vite.config.ts.bk
# ?? src/assets/main.css
mv src/style.css src/style.css.bk

echo '@import "tailwindcss"' > src/style.css

extract_embeded APP > src/App.vue
extract_embeded VITE > vite.config.ts

exit 0

# src/App.vue
__APP-0__
<script setup lang="ts">
</script>

<template>
  <div
    class="flex items-center justify-center gap-8 min-h-screen bg-gradient-to-br from-green-500 to-sky-400"
  >
    Hello, world!
  </div>
</template>
__APP-1__

# vite.config.ts
__VITE-0__
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

import vueDevTools from 'vite-plugin-vue-devtools'
import { fileURLToPath, URL } from 'node:url'
import tailwindcss from "@tailwindcss/vite"

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
    tailwindcss(),
  ],

  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      // extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue'],
    },
  },
})
__VITE-0__
