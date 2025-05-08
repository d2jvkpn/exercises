import { createApp } from 'vue'
//import './styles/style.css'
import App from './App.vue'

import router from './router'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import './styles/style.css'

//createApp(App).mount('#app')

const app = createApp(App)

app.use(ElementPlus)
app.use(router)

app.mount('#app')
