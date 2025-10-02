<script lang="ts" setup>
defineProps({
  title: {
    type: String,
    default: ''
  }
})
</script>

<template>
  <div class="border dark:border-[#4C4D4F] rounded-md">
    <div
      class="text-[#515A6E] dark:text-[#AEB9C0] dark:bg-[#333333] border-b dark:border-[#4C4D4F] p-2 text-sm"
      style="user-select: none"
    >
      {{ title }}
    </div>
    <div class="p-2" style="user-select: none">
      <slot></slot>
    </div>
  </div>
</template>

<style lang="scss" scoped></style>
