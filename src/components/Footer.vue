<script setup lang="ts">
import {Browser, ClearFormat, Pin, SettingConfig} from '@icon-park/vue-next'
import config from '../../package.json'
import {computedAsync} from '@vueuse/core'
import {onMounted, onUnmounted} from 'vue'
import {invoke} from "@tauri-apps/api/core";
import {listen} from '@tauri-apps/api/event'
import {WebviewWindow, getCurrentWebviewWindow, getAllWebviewWindows} from '@tauri-apps/api/webviewWindow'
import {eventBus} from "../utils/eventBus.ts";
import {useRoute} from "vue-router";

const route = useRoute()
const onTop = computedAsync(async () => {
  return await invoke('get_window_always_on_top')
})
const themeMode = computedAsync(async () => {
  return await invoke('get_theme')
})

const openDevTools = async () => {
  await invoke('open_dev_tools')
}

const unlisten = await getCurrentWebviewWindow().onThemeChanged(({payload: theme}) => {
  themeMode.value = theme
})

const handleClear = () => {
  eventBus.emit('clear')
}

const top = async () => {
  await invoke('set_window_always_on_top', {value: !onTop.value})
  eventBus.emit('change_on_top')
}

listen('theme-changed', async () => {
  themeMode.value = await invoke('get_theme')
})

const onTopChanged = async () => {
  onTop.value = await invoke('get_window_always_on_top')
}

const openConfigWindow = async () => {
  const windows = await getAllWebviewWindows()
  const window = windows.find(window => window.label === 'config')

  window!.onCloseRequested((event) => {
    event.preventDefault()
    window!.hide()
  }).catch(error => console.log(error))

  await window!.center()
  await window!.show()
}

onMounted(() => {
  eventBus.on('change_on_top', onTopChanged)
})

onUnmounted(() => {
  unlisten()
  eventBus.off('change_on_top', onTopChanged)
})
</script>

<template>
  <div
      class="w-full h-full text-sm leading-[30px] text-center border-t border-[#dcdfe6] dark:border-[#4c4d4f] bg-[#F7F7F7] text-[#515A6E] dark:bg-[#333] dark:text-[#BDC6CD] relative"
      style="user-select: none"
  >
    tkzc00作品&nbsp;v{{ config.version }}
    <div class="h-full absolute top-0 right-2 flex items-center justify-center gap-2 mx-2">
      <el-tooltip v-if="!route.path.includes('config')" content="清空输入" :effect="themeMode">
        <clear-format
            theme="outline"
            size="24"
            :stroke-width="4"
            class="cursor-pointer text-[#515A6E] dark:text-[#BDC6CD] hover:text-[#29a745]"
            @click="handleClear"
        />
      </el-tooltip>
      <el-tooltip :content="onTop ? '取消置顶' : '置顶'" :effect="themeMode">
        <Pin
            v-if="onTop"
            class="cursor-pointer text-[#29a745]"
            size="24"
            :stroke-width="4"
            theme="filled"
            @click="top()"
        />
        <Pin
            v-else
            class="cursor-pointer hover:text-[#29a745] text-[#515A6E] dark:text-[#BDC6CD]"
            size="24"
            :stroke-width="4"
            @click="top()"
        />
      </el-tooltip>
      <el-tooltip content="打开开发者工具" :effect="themeMode">
        <browser
            theme="outline"
            size="24"
            :stroke-width="4"
            class="cursor-pointer text-[#515A6E] dark:text-[#BDC6CD] hover:text-[#29a745]"
            @click="openDevTools"
        />
      </el-tooltip>
      <el-tooltip v-if="!route.path.includes('config')" content="设置" :effect="themeMode">
        <setting-config
            theme="filled"
            size="24"
            :stroke-width="4"
            class="text-[#515A6E] dark:text-[#BDC6CD] hover:text-[#29a745] cursor-pointer flex items-center"
            @click="openConfigWindow"
        />
      </el-tooltip>
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
