#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


# https://github.com/nvm-sh/nvm
# https://nodejs.org/en

curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash

nvm install --lts
node --version

npm config set registry https://registry.npm.taobao.org
# npm config set registry https://registry.npmjs.org/

mkdir -p ~/Apps
npm set --location=global prefix ~/Apps

npm config get registry

npm install -g create-react-app yarn
ls ~/Apps

# ~/.bash_aliases
export PATH=~/Apps/bin:$PATH

which create-react-app

# npm install --save react@latest
# npx browserslist@latest --update-db
