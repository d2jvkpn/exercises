#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


function extract_embeded() {
    key=$1
    sed -n "/^__${key}_START__$/,/^__${key}_END__/p" "$0" | tail -n +2 | head -n -1
}

mkdir -p  archive/src

mv vite.config.ts archive/
extract_embeded VITE > vite.config.ts

mv src/App.vue archive/src/
extract_embeded APP > src/App.vue

# ?? src/assets/main.css
mv src/style.css archive/src/
echo '@import "tailwindcss"' > src/style.css


exit 0

# vite.config.ts
__VITE_START__
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
__VITE_END__

# src/App.vue
__VUE_START__
<script setup lang="ts">
</script>

<template>
  <div
    class="flex items-center justify-center gap-8 min-h-screen bg-gradient-to-br from-green-500 to-sky-400"
  >
    <p>Hello, world!</p>
  </div>
</template>
__VUE_END__
