import {defineStore} from 'pinia'
import {SettingsType} from '../types'

export const useSettingsStore = defineStore('settings', {
    state: () => {
        return {
            settings: {} as SettingsType
        }
    },
    actions: {
        setSettings(value: SettingsType): void {
            this.settings = value
        },
        getSettings(): SettingsType {
            return this.settings
        }
    },
    persist: {
        key: 'settings',
        storage: localStorage
    }
})
