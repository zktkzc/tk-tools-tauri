import {LazyStore} from "@tauri-apps/plugin-store";

const store = new LazyStore("config.json")

const set = async (key: string, value: any) => {
    await store.set(key, value)
    await store.save()
}

const get = async (key: string) => {
    return await store.get(key)
}

export { store, set, get }