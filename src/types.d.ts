export type HashResult = {
    md5: string
    sha1: string
    sha256: string
    sha512: string
    sm3: string
}

export type LangType = 'json'

export type SettingsType = {
    theme: 'dark' | 'light' | 'system'
    autoUpdate: boolean
}

export type JsonToolDataType = {
    time: number
    data: {
        jsonStr: string
        result: string
        needTransfer: boolean
        needWrap: boolean
        activeDropItem: object
    }
}

export type HashCalcDataType = {
    time: number
    data: {
        originValue: string
    }
}

export type GenerateCharacterDataType = {
    time: number
    data: {
        result: string
        length: number
        count: number
        characters: string
        split: string
        needQuotes: boolean
        checkList: string[]
        records: string[]
    }
}

export type GenerateUUIDDataType = {
    time: number
    data: {
        result: string
        count: number
        split: string
        needQuotes: boolean
        needHyphen: boolean
        needUpperCase: boolean
        needToUint8: boolean
        records: string[]
    }
}

export type TextDiffDataType = {
    time: number
    data: {
        a: string
        b: string
        options: {
            lineWrap: boolean
            revertControl: string
        }
    }
}
