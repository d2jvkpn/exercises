import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

import tsconfigPaths from 'vite-tsconfig-paths';

// get --base /site, not --base=/site
function getArg(key: string): string | undefined {
  const index = process.argv.indexOf(key);
  return index !== -1 ? process.argv[index + 1] : "/";
}

const base = getArg('--base');
console.log(`==> base=${base}`);

// Can't read BASE_URL from env
// const BASE_URL="/";

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), tsconfigPaths()],
  // base: BASE_URL,
  build: {
    outDir: 'dist' + base,
  },
})
