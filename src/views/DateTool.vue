<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const menuList = [
  {
    value: 0,
    label: '时间戳',
    path: '/home/date/timestamp'
  },
  {
    value: 1,
    label: '计算器',
    path: '/home/date/calculator'
  }
]
const activeMenu = ref(0)
const router = useRouter()
const route = useRoute()
const handleChange = (value: number): void => {
  if (route.query?.to) {
    const toPath = route.query.to as string
    const menu = menuList.find((menu) => {
      return toPath.includes(menu.path)
    })
    activeMenu.value = menu?.value || 0
    if (toPath !== menu?.path) {
      router.push({ path: route.query.to as string, query: { to: toPath } })
      return
    }
    router.push(route.query.to as string)
    return
  }

  const item = menuList.find((item) => item.value === value)
  router.push(item!.path)
}

onMounted(() => {
  handleChange(activeMenu.value)
})
</script>

<template>
  <div class="w-full h-[calc(100vh-88px)] px-2 py-0 broder border-gray-500 flex flex-col justify-center">
    <div class="w-full h-[35px] flex items-center justify-between flex-wrap">
      <el-radio-group v-model="activeMenu" @change="handleChange">
        <el-radio v-for="item in menuList" :key="item.value" :value="item.value">
          {{ item.label }}
        </el-radio>
      </el-radio-group>
    </div>
    <div class="flex-1 w-full border dark:border-[#4C4D4F] rounded-md overflow-hidden">
      <router-view />
    </div>
  </div>
</template>

<style lang="scss" scoped>
:deep(.el-radio) {
  margin-right: 10px;
  --el-radio-text-color: #515a6e;
  --el-radio-input-border-color-hover: #29a745;

  @media (prefers-color-scheme: dark) {
    --el-radio-text-color: #bdc6cd;
    --el-radio-input-bg-color: #252525;
  }
}

:deep(.el-radio__inner) {
  @apply dark:border-[#4C4D4F];
}

:deep(.el-radio__input.is-checked + .el-radio__label) {
  color: #29a745 !important;
}

:deep(.el-radio__input.is-checked .el-radio__inner) {
  background-color: transparent;
  border-color: #29a745;

  &::after {
    background-color: #29a745 !important;
    transform: translate(-50%, -50%) scale(2);
  }
}

:deep(.el-input__inner) {
  @apply dark:text-[#BDC6CD];
}

:deep(.el-input__wrapper) {
  @apply dark:bg-[#202124] border dark:border-[#4C4D4F];
  box-shadow: none !important;
}

:deep(.el-input-group__prepend) {
  @apply dark:bg-[#333] border dark:border-[#4C4D4F] dark:text-[#919398] px-2;
  box-shadow: none !important;
  user-select: none;
}
</style>
