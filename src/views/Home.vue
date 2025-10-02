<script lang="ts" setup>
import Menu from "../components/Menu.vue";
</script>

<template>
  <main class="h-full w-full flex flex-col items-center bg-white dark:bg-[#252525]">
    <div class="w-full">
      <Menu />
    </div>
    <div class="w-full flex-1">
      <router-view/>
    </div>
  </main>
</template>

<style lang="scss" scoped>
</style>
