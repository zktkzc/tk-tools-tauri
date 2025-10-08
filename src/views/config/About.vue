<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";

const checkUpdate = async () => {
  await invoke('check_update')
}
</script>

<template>
  <div class="">
  </div>
</template>

<style scoped lang="scss">
</style>