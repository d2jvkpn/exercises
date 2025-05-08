import { createApp } from 'vue'
import './style/style.css'
import App from './App.vue'

import router from './router'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

// createApp(App).mount('#app')

const app = createApp(App)

app.use(ElementPlus)
app.use(router)

app.mount('#app')
