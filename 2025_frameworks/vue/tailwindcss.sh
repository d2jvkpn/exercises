#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _path=$(dirname $0)


# https://tailwindcss.com/docs/installation/using-vite
npm install tailwindcss @tailwindcss/vite

git mv src/assets/main.css src/assets/main.css.bk
git mv src/App.vue src/App.vue.bk
cp vite.config.ts vite.config.ts.bk

echo '@import "tailwindcss";' > src/assets/main.css

cat > src/App.vue <<EOF
<script setup lang="ts">
</script>

<template>
  <div class="flex items-center justify-center gap-8 min-h-screen
  bg-gradient-to-br from-green-500 to-sky-400">
    Profile Card
  </div>
</template>
EOF

sed -i -e "6i import tailwindcss from '@tailwindcss/vite'" \
  -e '/plugins:/a \ \ \ \ tailwindcss(),' vite.config.ts vite.config.ts
