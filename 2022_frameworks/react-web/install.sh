#! /usr/bin/env bash
set -eu -o pipefail
_wd=$(pwd)
_path=$(dirname $0 | xargs -i readlink -f {})


npm install --global serve create-react-app yarn # version 5.0.1

create-react-app react-web
cd react-web

# npm install --save-dev env-cmd
yarn install --save-dev env-cmd

yarn install sprintf-js antd @aws-sdk/client-s3 ali-oss
yarn upgrade 
