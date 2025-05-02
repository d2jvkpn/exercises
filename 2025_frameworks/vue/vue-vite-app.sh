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

npm init vite@latest $app -- --template=vue-ts  # interactive
cd $app

npm install
#npm run format
npm fund

cat > env <<"EOF"
# path: .env
PORT=3001

VITE_ENV=local
VITE_BASE=/site
VITE_API_URL=localhost:3011
EOF

cp env .env


cat > Makefile <<"EOF"
#!/bin/make

include .env

SHELL = /bin/bash

run:
	# node node_modules/vite/bin/vite.js --help
	npm run dev -- --port=$(PORT) --host=0.0.0.0 # --mode dev

build:
	rm -rf target/dist
	# node node_modules/vite/bin/vite.js build --help
	npm run build -- --base=$(VITE_BASE) --outDir=target/dist$(VITE_BASE) # --mode dev
	ls -alt target/dist
EOF

cat >> ..gitignore <<EOF


.env
.env.*
target/
configs/

docker-compose.yaml
docker-compose.yml
compose.yaml
compose.yml
EOF

# git init
git add -A
git commit -m "initial commit"


#### 3. apply tailwindcss
bash ${_dir}/tailwindcss.sh

git add -A
git commit -m "apply tailwindcss"
