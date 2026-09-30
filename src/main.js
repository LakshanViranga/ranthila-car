import { createApp } from 'vue'
import { createPinia } from "pinia";
import App from './App.vue'
import router from "./router"
import './assests/main.css'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import i18n from './i18n'

import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

import { debug, info, warn, error } from '@tauri-apps/plugin-log'

// ---------- Logging setup ----------

// Convert any value to a string for the log file
function toText(a) {
    if (a instanceof Error) return `${a.message}\n${a.stack ?? ''}`
    if (typeof a === 'object' && a !== null) {
        try { return JSON.stringify(a) } catch { return String(a) }
    }
    return String(a)
}

// Send console.* calls to the log file too
function forward(logFn, original) {
    return (...args) => {
        original(...args)
        logFn(args.map(toText).join(' ')).catch(() => {})
    }
}

console.log = forward(info, console.log)
console.info = forward(info, console.info)
console.warn = forward(warn, console.warn)
console.error = forward(error, console.error)
console.debug = forward(debug, console.debug)

// Catch errors that nobody handled
window.addEventListener('error', (e) => {
    error(`Uncaught error: ${e.message} at ${e.filename}:${e.lineno}:${e.colno}`).catch(() => {})
})
window.addEventListener('unhandledrejection', (e) => {
    error(`Unhandled promise rejection: ${toText(e.reason)}`).catch(() => {})
})

const vuetify = createVuetify({
    components,
    directives,
})

const app = createApp(App)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(router)
app.use(pinia)
app.use(vuetify)
app.use(i18n)
app.mount('#app')
