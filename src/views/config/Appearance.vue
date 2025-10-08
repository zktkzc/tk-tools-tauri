<script setup lang="ts">
import {useSettingsStore} from "../../store/useSettingsStore.ts";
import {SettingsType} from "../../types";
import {onMounted, onUnmounted, ref} from "vue";
import {computedAsync} from "@vueuse/core";
import {invoke} from "@tauri-apps/api/core";
import {set} from "../../store/AppConfigStore.ts";
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";

const {setSettings, getSettings} = useSettingsStore()
const settings = ref<SettingsType>({} as SettingsType)
const themeMode = computedAsync(async () => {
  return await invoke('get_theme')
})

const changeTheme = async (value: 'dark' | 'light' | 'system') => {
  await invoke('change_theme', {value})
  themeMode.value = await invoke('get_theme')
  settings.value!.theme = value
  await storeSettings()
}

const storeSettings = async () => {
  setSettings(settings.value)
  await set('settings', settings.value)
}

let unlisten = await getCurrentWebviewWindow().onThemeChanged(({payload: theme}) => {
  themeMode.value = theme
})

onMounted(() => {
  settings.value = getSettings()
})

onUnmounted(() => {
  storeSettings()
  unlisten()
})
</script>

<template>
  <div class="config-container">
    <div class="config-body">
      <div class="config-title">外观</div>
      <div class="config-item">
        <div class="config-wrapper">
          <div class="config-content border-none">
            <span>主题模式</span>
            <el-select v-model="settings!.theme" popper-class="custom-select" class="w-[200px]" @change="changeTheme">
              <el-option label="跟随系统" value="light"/>
              <el-option label="深色模式" value="dark"/>
              <el-option label="浅色模式" value="system"/>
            </el-select>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
</style>