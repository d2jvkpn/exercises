#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _path=$(dirname $0)


#### 1. node.js
which node && node --version

npm install --global npm yarn
which npm && which yarn


##### 2. create project 
#npm install --global @vue/cli
# which vue && vue --version
#vue create -d vue-app01
#cd vue-app01

npm create vue@latest web01
cd web01

npm install
npm run format
npm fund

# git init
git add -A
git commit -m "initial commit"


#### 3. apply tailwindcss
bash ${_path}/tailwindcss.sh

git add -A
git commit -m "apply tailwindcss"


#### 4. run
npm run dev -- --port=3000
