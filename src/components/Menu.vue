<script setup lang="ts">
import {onMounted, ref} from "vue";
import {useRoute, useRouter} from "vue-router";

const route = useRoute()
const router = useRouter()
const activeIndex = ref('0')
const menuItems = [
  {index: '0', label: 'JSON工具', path: '/home/json'},
  {index: '1', label: '日期工具', path: '/home/date'},
  {index: '2', label: '加解密', path: '/home/encrypt'},
  {index: '3', label: '生成工具', path: '/home/gen'},
  {index: '4', label: '文本比对', path: '/home/diff'}
]

const handleSelect = (key: string): void => {
  if (route.query?.to) {
    const toPath = route.query.to as string
    const menu = menuItems.find((menu) => {
      return toPath.includes(menu.path)
    })
    activeIndex.value = menu?.index || '0'
    if (toPath !== menu?.path) {
      router.push({path: route.query.to as string, query: {to: toPath}})
      return
    }
    router.push(route.query.to as string)
    return
  }

  const item = menuItems.find((i) => i.index === key)
  if (item) {
    if (route.path.startsWith(item.path)) return
    router.push(item.path)
  }
}

onMounted(() => {
  handleSelect(activeIndex.value)
})
</script>

<template>
  <div>
    <el-menu
        v-model="activeIndex"
        :default-active="activeIndex"
        style="user-select: none"
        mode="horizontal"
        @select="handleSelect"
    >
      <el-menu-item
          v-for="item in menuItems"
          :key="item.index"
          :index="item.index"
          class="cursor-pointer"
      >
        {{ item.label }}
      </el-menu-item>
    </el-menu>
  </div>
</template>

<style scoped lang="scss">
:deep(.el-menu--horizontal) {
  @apply border-none p-2;
  box-shadow: inset 0 1px 2px #ffffff30,
  inset 0 1px 2px #00000030,
  inset 0 2px 4px #00000015;

  --el-menu-horizontal-height: 50px;
  --el-menu-bg-color: #f7f7f7;
  --el-menu-text-color: #576173;
  --el-menu-hover-bg-color: rgba(226, 239, 235, 0.5);
  --el-menu-hover-text-color: #576173;
  --el-menu-active-color: #29a745;
  --el-menu-border-color: #dcdfe6;

  @media (prefers-color-scheme: dark) {
    --el-menu-bg-color: #333;
    --el-menu-text-color: #bdc6cd;
    --el-menu-hover-bg-color: rgba(15, 173, 142, 0.12);
    --el-menu-hover-text-color: #bdc6cd;
    --el-menu-active-color: #29a745;
    --el-menu-border-color: #4c4d4f;
  }

  .el-menu-item {
    @apply border-none rounded-md p-2;

    &.is-active {
      @apply bg-[#29a745] font-normal;
      --el-menu-active-color: #fff;
      box-shadow: inset 0 1px 2px #ffffff30,
      0 1px 2px #00000030,
      0 2px 4px #00000015;
    }

    &:hover {
      box-shadow: inset 0 1px 2px #ffffff30,
      0 1px 2px #00000030,
      0 2px 4px #00000015;
    }
  }

  .el-menu-item + .el-menu-item {
    @apply ml-2;
  }
}
</style>