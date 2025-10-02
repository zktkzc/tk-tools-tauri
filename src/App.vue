<script setup lang="ts">
import Footer from './components/Footer.vue'
import {get} from "./store/AppConfigStore.ts";
import {SettingsType} from "./types";
import {onMounted} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {eventBus} from "./utils/eventBus.ts";

onMounted(async () => {
  const theme = (await get('settings') as SettingsType)?.theme
  if (theme) {
    await invoke('change_theme', {value: theme})
    eventBus.emit('change_theme')
  }
})
</script>

<template>
  <Suspense>
    <div class="h-screen w-screen flex flex-col">
      <div class="w-full flex-1 overflow-hidden">
        <router-view/>
      </div>
      <div class="w-full h-[31px]">
        <Footer/>
      </div>
    </div>
  </Suspense>
</template>
