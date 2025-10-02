import { createApp } from "vue";
import App from "./App.vue";
import {createPinia} from "pinia";
import piniaPluginPersistedState from 'pinia-plugin-persistedstate'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import router from './router'

import './assets/main.scss'

const app = createApp(App)

app.use(router)
app.use(ElementPlus)

const pinia = createPinia()
pinia.use(piniaPluginPersistedState)
app.use(pinia)

app.mount('#app')