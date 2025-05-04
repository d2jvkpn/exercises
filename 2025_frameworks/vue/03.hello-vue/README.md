# Vue 3 + TypeScript + Vite

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

Learn more about the recommended Project Setup and IDE Support in the [Vue Docs TypeScript Guide](https://vuejs.org/guide/typescript/overview.html#project-setup).


#### ch01. setup
1. create env file
```
cp env .env
```

2. run
```
make run

npm run dev -- --port=3001 --host=0.0.0.0
```

3. build
```
make build

rm -r target/dist
npm run build -- --base=/local --outDir=target/dist/local

ls -alt target/dist/local
```
