import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

function getArg(key: string): string | undefined {
  let index = process.argv.indexOf(key);
  // return index !== -1 ? process.argv[index + 1] : "/";

  if (index !== -1) {
    return process.argv[index + 1];
  }

  let base = process.argv.find(v => v.startsWith("--base="));

  return base ? base.slice(7) : "/";
}

const base = getArg('--base');
console.log(`==> vite: base=${base}`);

// Can't read BASE_URL from env
// const BASE_URL="/";

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  // base: BASE_URL,
  build: {
    outDir: 'target/static' + base,
  },
})
