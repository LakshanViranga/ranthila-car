import {defineStore} from 'pinia'
import { ref } from 'vue'

export const useAuthStore = defineStore('auth', () => {
    const username = ref<string | null>(null)
    const role = ref<string | null>(null)

    const setAuth =  (user: string, userRole: string) => {
        username.value = user
        role.value = userRole
    }

    const resetAuth = () => {
        username.value = null
        role.value = null
    }
    return {
        username,
        role,
        setAuth,
        resetAuth
    }
},{
    persist:{
        enabled: true,
        strategies: [
            {
                key: 'auth',
                storage: sessionStorage
            }
        ]
    }
})
