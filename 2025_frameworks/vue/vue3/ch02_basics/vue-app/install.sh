#!/bin/bash
set -eu -o pipefail; _wd=$(pwd); _dir=$(readlink -f `dirname "$0"`)


npm install bootstrap bootstrap-vue-3


cat <<EOF 
import { createApp } from 'vue'
// import './style.css'
import App from './App.vue'

// createApp(App).mount('#app')

import BootstrapVue3 from 'bootstrap-vue-3'
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue-3/dist/bootstrap-vue-3.css'

const app = createApp(App)
app.use(BootstrapVue3)

app.mount('#app')
EOF


exit
https://www.bilibili.com/video/BV18HzGYREWm
