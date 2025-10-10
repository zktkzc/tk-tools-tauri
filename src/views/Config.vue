<script lang="ts" setup>
import {SettingsType} from '../types'
import {onMounted, onUnmounted, ref} from 'vue'
import {useSettingsStore} from '../store/useSettingsStore'
import {computedAsync} from '@vueuse/core'
import {invoke} from "@tauri-apps/api/core"
import {get, set} from "../store/AppConfigStore.ts"
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";
import {Info, Platte, SettingConfig} from '@icon-park/vue-next'
import {useRouter} from "vue-router";

const indexPathMap = [
  {index: '1', path: '/config/system'},
  {index: '2', path: '/config/appearance'},
  {index: '3', path: '/config/about'},
]

const router = useRouter()
const {setSettings, getSettings} = useSettingsStore()
const settings = ref<SettingsType>({} as SettingsType)
const themeMode = computedAsync(async () => {
  return await invoke('get_theme')
})
const activeMenuIndex = ref<string>('1')

const handleSelect = (key: string, _keyPath: string[]) => {
  activeMenuIndex.value = key
  const menu = indexPathMap.find(item => item.index === key)
  if (menu) {
    router.push(menu.path)
  }
}

const initSettings = async () => {
  let oldSettings = await get('settings') as SettingsType
  if (!oldSettings) {
    oldSettings = getSettings()
  }
  if (!oldSettings.theme) oldSettings.theme = 'system'
  if (oldSettings.autoUpdate === undefined) oldSettings.autoUpdate = true
  settings.value = oldSettings
  await set('settings', settings.value)
  await storeSettings()
}

const storeSettings = async () => {
  await set('settings', settings.value)
  setSettings(settings.value)
}

let unlisten = await getCurrentWebviewWindow().onThemeChanged(({payload: theme}) => {
  themeMode.value = theme
})

onMounted(() => {
  initSettings()
  handleSelect(activeMenuIndex.value, [])
})

onUnmounted(() => {
  storeSettings()
  unlisten()
})
</script>

<template>
  <div
      class="h-full w-full bg-white overflow-hidden dark:bg-[#252525] flex flex-col justify-center"
  >
    <div class="w-full flex-1 overflow-y-auto" style="user-select: none">
      <el-container
          class="w-full h-full dark:text-[#bdc6cd] overflow-y-auto relative z-[2]"
      >
        <el-aside width="150px">
          <el-scrollbar>
            <el-menu class="h-full pt-2 border-none dark:bg-[#1B1B1B]" default-active="1" @select="handleSelect">
              <el-menu-item index="1">
                <template #title>
                  <div class="content">
                    <span><setting-config size="20"/></span>
                    系统
                  </div>
                </template>
              </el-menu-item>
              <el-menu-item index="2">
                <template #title>
                  <div class="content">
                    <span><platte size="20"/></span>
                    外观
                  </div>
                </template>
              </el-menu-item>
              <el-menu-item index="3">
                <template #title>
                  <div class="content">
                    <span><info size="20"/></span>
                    关于
                  </div>
                </template>
              </el-menu-item>
            </el-menu>
          </el-scrollbar>
        </el-aside>
        <el-main class="p-0 bg-[#f3f3f3] dark:bg-[#111]">
          <router-view/>
        </el-main>
      </el-container>
    </div>
  </div>
</template>

<style lang="scss" scoped>
:deep(.el-scrollbar__view) {
  @apply h-full;
}

:deep(.el-menu-item) {
  @apply h-[40px] w-full hover:bg-transparent;
  padding: 0 !important;
  --el-menu-active-color: #424242;

  @media (prefers-color-scheme: dark) {
    --el-menu-active-color: #e4e4e4;
  }

  .content {
    @apply w-full flex items-center justify-start pl-5 mx-2 py-1.5 rounded-md text-base
    dark:text-[#fff] hover:bg-[#EBEBEB] dark:hover:bg-[#363636];

    &:hover {
      box-shadow: inset 0 1px 2px #ffffff30,
      0 1px 2px #00000030,
      0 2px 4px #00000015;
    }

    span {
      @apply flex items-center justify-center mr-1.5 text-[#424242] dark:text-[#E4E4E4];
    }
  }

  &.is-active {
    .content {
      @apply bg-[#29a745] hover:bg-[#29a745] text-[#fff];
      box-shadow: inset 0 1px 2px #ffffff30,
      0 1px 2px #00000030,
      0 2px 4px #00000015;

      span {
        @apply text-[#fff];
      }
    }
  }
}

:deep(.config-container) {
  @apply h-full w-full px-5 py-2;

  .config-body {
    .config-title {
      @apply text-[22px] mb-3 text-black dark:text-white;
    }

    .config-item {
      @apply rounded-md overflow-hidden;

      .config-wrapper {
        @apply px-2 bg-[#fff] dark:bg-[#262626];
        box-shadow: inset 0 1px 2px #ffffff30,
        0 1px 5px #00000030,
        0 2px 10px #00000015;

        .config-content {
          @apply grid grid-cols-[auto_auto] py-3 border-b;

          > span {
            @apply text-[15px] flex items-center text-black dark:text-white;
          }
        }
      }
    }
  }
}

:deep(.el-select) {
  .el-select__wrapper {
    @apply dark:bg-[#373737] shadow-none hover:shadow-none border border-[#DCDFE6]
    dark:border-[#4C4D4F] hover:border-[#29a745];
  }

  .el-select__placeholder {
    @apply text-[15px] text-black dark:text-[#fff];
  }
}
</style>
