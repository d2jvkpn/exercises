#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


app=react-app

npm create vite@latest $app -- --template react
cd $app

npm install

npm install react-router-dom
npm install antd @ant-design/icons @ant-design/cssinjs --save


npm install vite-plugin-style-import --save-dev

npm run dev


