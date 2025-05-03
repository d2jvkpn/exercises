#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


app=${1:-hello-vue}

npm init vite@latest $app -- --template=vue-ts
cd $app

npm install
#npm run format
npm fund


npm install -D vite-plugin-vue-devtools


cat > env <<"EOF"
# path: .env
PORT=3001

VITE_ENV=local
VITE_BASE=/
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


cat >> .gitignore <<EOF


.env
.env.*
target/
configs/

docker-compose.yaml
docker-compose.yml
compose.yaml
compose.yml
EOF
