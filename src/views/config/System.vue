<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {computedAsync} from "@vueuse/core";
import {onMounted, onUnmounted, ref} from "vue";
import {SettingsType} from "../../types";
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
import {useSettingsStore} from "../../store/useSettingsStore.ts";
import {set} from "../../store/AppConfigStore.ts";

const {setSettings, getSettings} = useSettingsStore()
const updateBtnText = ref<string>('检查更新')
const settings = ref<SettingsType>({} as SettingsType)
const themeMode = computedAsync(async () => {
  return await invoke('get_theme')
})

const changeAutoUpdate = async (value: boolean) => {
  settings.value.autoUpdate = value
  await storeSettings()
}

const checkUpdate = async () => {
  updateBtnText.value = '检查中...'
  const {status, version} = await invoke('check_update') as { status: boolean, version: string }
  console.log(status, version)
  if (!status) {
    updateBtnText.value = '已经是最新版本'
  } else {
    updateBtnText.value = `发现新版本: ${version}!`
  }

  setTimeout(() => {
    updateBtnText.value = '检查更新'
  }, 5000)
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
            <div class="flex items-center gap-4">
              <el-checkbox class="check-box-no-border" label="启动时自动更新" v-model="settings.autoUpdate" @change="changeAutoUpdate"/>
              <el-button class="button-with-bg w-[170px]" @click="checkUpdate">{{ updateBtnText }}</el-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">

</style>