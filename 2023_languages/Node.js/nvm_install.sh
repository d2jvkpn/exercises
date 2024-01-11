#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

# https://github.com/nvm-sh/nvm
# https://nodejs.org/en

# curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash

cat >> ~/.bashrc <<'EOF'
#### NVM
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
EOF

nvm install --lts
version=$(node --version | sed 's/^v//') # 20.11.0
# nvm ls
# nvm use $version
nvm use --delete-prefix $version
nvm alias default $version
# nvm unalias default

npm config set registry https://registry.npm.taobao.org
# npm config set registry https://registry.npmjs.org/
npm config get registry

mkdir -p ~/Apps
npm set --location=global prefix ~/Apps

npm install -g create-react-app yarn
ls -al ~/Apps/npm/bin

# npm install --save react@latest
# npx browserslist@latest --update-db
