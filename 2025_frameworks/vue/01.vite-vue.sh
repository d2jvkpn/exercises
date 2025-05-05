#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


app=${1:-"hello-vue"}

#### 1. create the project
npm init vite@latest $app -- --template=vue-ts
cd $app

npm install
#npm run format
npm fund

#### 2. add packages
npm install --save-dev @types/node
npm install --save-dev vite-plugin-vue-devtools

# https://tailwindcss.com/docs/installation/using-vite
npm install vue-router
npm install tailwindcss @tailwindcss/vite

#### 3. setup env
cat > env <<"EOF"
# path: .env
PORT=3001

VITE_ENV=local
VITE_BASE_PATH=/
VITE_API_URL=localhost:3011
EOF

cp env .env

#### 4. setup makefile
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
	npm run build -- --base=$(VITE_BASE_PATH) --outDir=target/dist$(VITE_BASE_PATH) # --mode dev
	ls -alt target/dist
EOF

#### 5. setup gitignore
cat >> .gitignore <<EOF

####
.env
.env.*
target/

docker-compose.yaml
docker-compose.yml
compose.yaml
compose.yml
EOF
