<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {computedAsync} from "@vueuse/core";
import {onMounted, onUnmounted, ref} from "vue";
import {SettingsType} from "../../types";
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
import {useSettingsStore} from "../../store/useSettingsStore.ts";
import {set} from "../../store/AppConfigStore.ts";

const {setSettings, getSettings} = useSettingsStore()
const settings = ref<SettingsType>({} as SettingsType)
const themeMode = computedAsync(async () => {
  return await invoke('get_theme')
})

const changeAutoUpdate = async (value: boolean) => {
  settings.value.autoUpdate = value
  await storeSettings()
}

const checkUpdate = async () => {
  await invoke('check_update')
}

let unlisten = await getCurrentWebviewWindow().onThemeChanged(({payload: theme}) => {
  themeMode.value = theme
})

const storeSettings = async () => {
  setSettings(settings.value)
  await set('settings', settings.value)
}

onMounted(() => {
  settings.value = getSettings()
})

onUnmounted(() => {
  unlisten()
})
</script>

<template>
  <div class="config-container">
    <div class="config-body">
      <div class="config-title">更新</div>
      <div class="config-item">
        <div class="config-wrapper">
          <div class="config-content border-none">
            <span class="w-[150px]">升级配置</span>
            <div class="flex items-center gap-2">
              <el-checkbox label="启动时自动更新" v-model="settings.autoUpdate" @change="changeAutoUpdate"/>
              <el-button class="w-[170px]" @click="checkUpdate">检查更新</el-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">

</style>