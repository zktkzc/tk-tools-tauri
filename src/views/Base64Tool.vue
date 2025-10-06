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
  <div class="h-full w-full p-2 grid grid-cols-1 grid-rows-[40px_1fr] gap-2">
    <div class="w-full flex flex-col items-center justify-center">
      <el-radio-group v-model="activeMenu" fill="#29a745" @change="handleChange">
        <el-radio-button v-for="item in menuList" :key="item.value" :label="item.label" :value="item.value"/>
      </el-radio-group>
    </div>
    <router-view/>
  </div>
</template>

<style scoped lang="scss">
:deep(.el-radio-button) {
  &.is-active {
    .el-radio-button__inner:hover {
      color: #fff !important;
      background-color: #23923d !important;
    }
  }

  .el-radio-button__inner:hover {
    color: #29a745 !important;
  }
}
</style>