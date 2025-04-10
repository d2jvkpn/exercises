#!/bin/bash
set -eu -o pipefail

#### 1. node.js
which node
node

npm install --global npm yarn @vue/cli
which npm
which yarn
which vue

vue --version


#### 2. install vue
#npm install --global @vue/cli
#vue create -d vue-app01
#cd vue-app01

npm create vue@latest web01

cd web01
npm install
npm install tailwindcss @tailwindcss/vite

npm run format

npm fund

# git init
git add -A
git commit -m "initial commit"


#### 3. run
npm run dev -- --port=3000

