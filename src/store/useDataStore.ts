import {defineStore} from 'pinia'
import {
    Base64DecryptDataType,
    Base64EncryptDataType,
    GenerateCharacterDataType,
    GenerateUUIDDataType,
    HashCalcDataType,
    JsonToolDataType,
    TextDiffDataType
} from '../types'

export const useDataStore = defineStore('data', {
    state: () => {
        return {
            data: {
                json_tool: {} as JsonToolDataType,
                hash_calc: {} as HashCalcDataType,
                gen_ch: {} as GenerateCharacterDataType,
                gen_uuid: {} as GenerateUUIDDataType,
                text_diff: {} as TextDiffDataType,
                base64_encrypt: {} as Base64EncryptDataType,
                base64_decrypt: {} as Base64DecryptDataType
            }
        }
    },
    actions: {
        setData(key: keyof typeof this.data, value: any): void {
            this.data[key] = {...this.data[key], ...value}
        },
        getData(key: keyof typeof this.data): any {
            return this.data[key]
        }
    },
    persist: {
        key: 'data',
        storage: localStorage
    }
})
