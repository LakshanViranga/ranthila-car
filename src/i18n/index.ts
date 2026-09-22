import { createI18n } from 'vue-i18n'
import en from './en'
import si from './si'

const i18n = createI18n({
    legacy: true,

    locale: localStorage.getItem('language') || 'en',

    fallbackLocale: 'en',

    messages: {
        en,
        si
    }
})

export default i18n
