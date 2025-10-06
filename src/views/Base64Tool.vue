<script setup lang="ts">
import {onMounted, ref} from "vue";
import {useRouter} from "vue-router";

const router = useRouter()
const menuList = [
  {label: '加密', value: 'encrypt', path: '/home/encrypt/base64/encrypt'},
  {label: '解密', value: 'decrypt', path: '/home/encrypt/base64/decrypt'},
]
const activeMenu = ref<'encrypt' | 'decrypt'>('encrypt')

const handleChange = () => {
  const item = menuList.find(item => item.value === activeMenu.value)
  if (item) {
    router.push(item.path)
  }
}

onMounted(() => {
  handleChange()
})
</script>

<template>
  <div class="h-full w-full p-2 flex flex-col gap-2">
    <div class="w-full h-[40px] flex items-center justify-center">
      <el-radio-group v-model="activeMenu" fill="#29a745" @change="handleChange">
        <el-radio-button v-for="item in menuList" :key="item.value" :label="item.label" :value="item.value"/>
      </el-radio-group>
    </div>
    <div class="w-full flex-1">
      <router-view/>
    </div>
  </div>
</template>

<style scoped lang="scss">
:deep(.el-radio-button) {
  &.is-active {
    .el-radio-button__inner {
      @media (prefers-color-scheme: dark) {
        color: #fff !important;
        background-color: #29a745 !important;
      }

      &:hover {
        color: #fff !important;
        background-color: #23923d !important;
      }
    }
  }

  .el-radio-button__inner {
    @media (prefers-color-scheme: dark) {
      border-color: #4c4d4f;
      color: #dadada !important;
      background-color: #333 !important;
    }

    &:hover {
      @media (prefers-color-scheme: light) {
        color: #29A745;
      }

      @media (prefers-color-scheme: dark) {
        background-color: rgba(35, 146, 61, 0.38) !important;
      }
    }
  }
}
</style>