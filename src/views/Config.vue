<script lang="ts" setup>
import {ArrowLeft, Setting} from '@icon-park/vue-next'
import {useRoute, useRouter} from 'vue-router'
import {SettingsType} from '../types'
import {onMounted, onUnmounted, ref} from 'vue'
import {useSettingsStore} from '../store/useSettingsStore'
import {computedAsync} from '@vueuse/core'
import {invoke} from "@tauri-apps/api/core"
import {getCurrentWindow} from '@tauri-apps/api/window'
import {eventBus} from "../utils/eventBus.ts"
import {set, get} from "../store/AppConfigStore.ts"
import config from '../../package.json'

const {setSettings, getSettings} = useSettingsStore()
const router = useRouter()
const route = useRoute()
const settings = ref<SettingsType>({} as SettingsType)
const themeMode = computedAsync(async () => {
  return await invoke('get_theme')
})

const changeTheme = async (value: 'dark' | 'light' | 'system') => {
  await invoke('change_theme', {value})
  themeMode.value = await invoke('get_theme')
  settings.value!.theme = value
  await storeSettings()
  eventBus.emit('change_theme')
}

const changeAutoUpdate = async (value: boolean) => {
  settings.value.autoUpdate = value
  await storeSettings()
}

const checkUpdate = async () => {
  const result = await invoke('check_update')
  console.log(result)
}

const initSettings = async () => {
  let oldSettings = await get('settings') as SettingsType
  if (!oldSettings) {
    oldSettings = getSettings()

    if (oldSettings.theme) {
      await invoke('change_theme', {value: oldSettings.theme})
      eventBus.emit('change_theme')
    }
  }
  if (!oldSettings.theme) oldSettings.theme = 'system'
  if (!oldSettings.autoUpdate) oldSettings.autoUpdate = true
  settings.value = oldSettings
  await set('settings', settings.value)
}

const storeSettings = async () => {
  setSettings(settings.value)
  await set('settings', settings.value)
}

let unlisten = await getCurrentWindow().onThemeChanged(({payload: theme}) => {
  themeMode.value = theme
})

onMounted(() => {
  initSettings()
})

onUnmounted(() => {
  storeSettings()
  unlisten()
})
</script>

<template>
  <div
      class="h-full w-full bg-white overflow-hidden dark:bg-[#252525] flex flex-col justify-center space-y-2 overflow-y-hidden"
  >
    <div
        class="h-[32px] w-full bg-[#F7F7F7] dark:text-[#bdc6cd] dark:bg-[#333] px-2 flex items-center justify-center text-center relative"
        style="user-select: none"
    >
      <div
          class="h-[32px] w-[32px] absolute top-0 left-2 flex items-center justify-center hover:bg-[#eaf5f1] dark:text-[#bdc6cd] dark:hover:bg-[#2a3531] dark:hover:text-[#29a745] cursor-pointer"
          @click="
          () => {
            const to = route.query.from ? (route.query.from as string) : '/home/json'
            router.push({ path: '/home', query: { to } })
          }
        "
      >
        <el-tooltip content="返回" :effect="themeMode">
          <arrow-left theme="outline" size="24" :stroke-width="4"/>
        </el-tooltip>
      </div>
      软件设置
    </div>
    <div class="w-full flex-1 px-2 pb-2 overflow-y-auto" style="user-select: none">
      <div
          class="w-full h-full dark:text-[#bdc6cd] border dark:border-[#4C4D4F] rounded-md p-2 overflow-y-auto relative"
      >
        <el-form :model="settings" label-width="100px">
          <el-form-item label="主题" class="w-[180px]">
            <el-select v-model="settings!.theme" popper-class="custom-select" @change="changeTheme">
              <el-option label="亮色" value="light"/>
              <el-option label="暗色" value="dark"/>
              <el-option label="自动" value="system"/>
            </el-select>
          </el-form-item>
          <el-form-item label="软件更新">
            <el-checkbox label="启动时自动更新" v-model="settings.autoUpdate" @change="changeAutoUpdate"/>
          </el-form-item>
          <el-form-item label="当前版本">
            <div>v&nbsp;{{ config.version }}</div>
            <div class="pl-5">
              <el-button @click="checkUpdate">检测更新</el-button>
            </div>
          </el-form-item>
        </el-form>
        <div
            class="flex items-center justify-center text-slate-100 dark:text-[#515A6E] dark:opacity-15 bg-transparent absolute top-0 left-0 h-full w-full pointer-events-none"
        >
          <setting theme="outline" size="240" :stroke-width="1"/>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
:deep(.el-select__wrapper) {
  @apply dark:bg-[#252525] shadow-none hover:shadow-none border border-[#DCDFE6] dark:border-[#4C4D4F] hover:border-[#29a745];
}

:deep(.el-select__placeholder) {
  @apply text-[#545C70] dark:text-[#bbc6ce] hover:text-[#29a745];
}

:deep(.el-select) {
  &:hover {
    .el-select__placeholder {
      @apply text-[#29A745];
    }
  }
}

:deep(.el-form-item__label) {
  @apply text-[#545C70] dark:text-[#bbc6ce];
}

:deep(.el-checkbox) {
  @apply text-[#515A6E] dark:text-[#BBC6CE] hover:text-[#29A745] cursor-pointer;

  &:hover {
    .el-checkbox__inner {
      @apply border-[#29A745];
    }
  }
}

:deep(.el-checkbox__inner) {
  @apply dark:bg-[#202124] border border-[#DCDFE6] dark:border-[#4C4D4F];
}

:deep(.el-checkbox__input.is-checked .el-checkbox__inner) {
  @apply bg-[#29A745] border border-[#29A745];
}

:deep(.el-checkbox__input.is-checked + .el-checkbox__label) {
  @apply text-[#29A745];
}

:deep(.el-button) {
  --el-button-bg-color: #ffffff;
  --el-button-text-color: #515A6E;
  --el-button-border-color: #dddfe5;
  --el-button-hover-bg-color: #fff;
  --el-button-hover-border-color: #23923d;
  --el-button-hover-text-color: #23923d;
  --el-button-active-bg-color: #ffffff;
  --el-button-active-border-color: #23923d;

  @media (prefers-color-scheme: dark) {
    --el-button-bg-color: #252525;
    --el-button-border-color: #4c4d4f;
    --el-button-text-color: #bbc6ce;
    --el-button-hover-bg-color: #252525;
    --el-button-hover-border-color: #23923d;
    --el-button-hover-text-color: #23923d;
    --el-button-active-bg-color: #252525;
    --el-button-active-border-color: #23923d;
  }
}
</style>
