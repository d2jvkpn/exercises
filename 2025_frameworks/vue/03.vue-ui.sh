#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


exit
#### 1. Ant Design
npm install ant-design-vue @ant-design/icons-vue

cat <<EOF
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';

app.use(Antd);
EOF
