#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})

env_file=$1

# export REACT_APP_BuildTime=$(date +'%FT%T%:z') # date doesn't support %:z in alpine
export REACT_APP_BuildTime=$(printenv REACT_APP_BuildTime || true)

# if [[ -f /etc/alpine-release ]]; then
# sed '/=/s/^/export /' $env_file > ./config.env
awk '!/^#/{sub(" += +", "=", $0); print "export "$0}' $env_file > ./config.env
. ./config.env

echo ">>> REACT_APP_ENV: $REACT_APP_ENV, REACT_APP_BuildTime: $REACT_APP_BuildTime, \
PORT: $PORT, PUBLIC_URL: $PUBLIC_URL"

npm run-script build
# sed -i "s#%REACT_APP_ENV%#$REACT_APP_ENV#" build/index.html

PUBLIC_URL=$(printenv PUBLIC_URL | sed 's#^/*##; s#/*$##')
if [ ! -z "$PUBLIC_URL" ]; then
  rand_string=.build-$(echo $RANDOM | md5sum | head -c 20)
  mv build $rand_string
  mkdir -p build/$PUBLIC_URL
  mv $rand_string/* build/$PUBLIC_URL
  rm -r $rand_string
  cp build/$PUBLIC_URL/index.html build/ # make sure "serve -s build" works
fi

# PORT=$PORT serve -s ./build"
echo "~~~ $ serve -l $PORT -s ./build"

exit
npm install --global yarn
yarn install
