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


exit
#### 2. Element Plus
npm install npm install element-plus @element-plus/icons-vue

cat <<EOF
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';

app.use(ElementPlus);
EOF
