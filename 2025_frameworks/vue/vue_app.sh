#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(dirname $0 | xargs -i readlink -f {})


#### 1. node.js
which node && node --version

npm install --global npm yarn
which npm && which yarn


##### 2. create project
# https://vuejs.org/guide/quick-start.html

#npm install --global @vue/cli
# which vue && vue --version
#vue create -d vue-app01
#cd vue-app01

app=hello-vue

npm create vue@latest $app  # interactive
cd $app

npm install
npm run format
npm fund

cat > Makefile <<EOF
dev:
	npm run dev -- --port=3000
EOF

# git init
git add -A
git commit -m "initial commit"


#### 3. apply tailwindcss
bash ${_dir}/tailwindcss.sh

git add -A
git commit -m "apply tailwindcss"
