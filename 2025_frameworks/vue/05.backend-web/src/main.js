import { createApp } from 'vue'
//import './style.css'
import App from './App.vue'

import ElementPlus from 'element-plus'
import './styles/style.css'
import router from './router'


//createApp(App).mount('#app')

const app = createApp(App)

app.use(ElementPlus)

app.use(router)

app.mount('#app')
