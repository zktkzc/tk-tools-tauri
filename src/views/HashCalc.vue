<script setup lang="ts">
import {CheckOne, Down, UploadOne} from '@icon-park/vue-next'
import {computed, onMounted, onUnmounted, ref} from 'vue'
import {ElMessage} from 'element-plus'
import {HashCalcDataType, HashResult} from '../types'
import {useDataStore} from '../store/useDataStore'
import {invoke} from "@tauri-apps/api/core"
import {open} from '@tauri-apps/plugin-dialog'
import {eventBus} from "../utils/eventBus.ts"
import {platform} from '@tauri-apps/plugin-os'

const activeType = ref<'text' | 'file'>('text')
const originValue = ref<string>('')
const md5Value = ref<string>('')
const sha1Value = ref<string>('')
const sha256Value = ref<string>('')
const sha512Value = ref<string>('')
const sm3Value = ref<string>('')
const {getData, setData} = useDataStore()

const dropdownMenuList = [
  {command: 'text', label: '文本'},
  {command: 'file', label: '文件'}
]
const activeMenu = computed(() => {
  return dropdownMenuList.find((menu) => menu.command === activeType.value)
})

const clearResult = () => {
  md5Value.value = ''
  sha1Value.value = ''
  sha256Value.value = ''
  sha512Value.value = ''
  sm3Value.value = ''
}

const handleCommand = (type: 'text' | 'file'): void => {
  activeType.value = type
  clearResult()

  if (type === 'text') {
    file.value = ''
    initData()
  }
}

const handleInput = async (value: string): Promise<void> => {
  if (!value || value === '') {
    clearResult()
    return
  }
  const {md5, sha1, sha256, sha512, sm3} = await invoke('calculate_hash', {value}) as HashResult
  md5Value.value = md5
  sha1Value.value = sha1
  sha256Value.value = sha256
  sha512Value.value = sha512
  sm3Value.value = sm3

  setData('hash_calc', {
    time: new Date().getTime(),
    data: {originValue: value}
  } as HashCalcDataType)
}

const copy = (value: string): void => {
  if (value === '') return
  navigator.clipboard.writeText(value)
  ElMessage.success({message: '复制成功', grouping: true, customClass: 'success'})
}

const file = ref('')
const selectFile = async () => {
  const result = await open({
    title: '选择文件'
  }) as string
  try {
    invoke("calculate_file_hash", {path: result})
        .then((data: any) => {
          const {md5, sha1, sha256, sha512, sm3} = data as HashResult
          md5Value.value = md5
          sha1Value.value = sha1
          sha256Value.value = sha256
          sha512Value.value = sha512
          sm3Value.value = sm3

          if (platform() === 'windows') {
            file.value = result.substring(result.lastIndexOf('\\') + 1)
          } else {
            file.value = result.substring(result.lastIndexOf('/') + 1)
          }
        })
  } catch (e) {
    const error = e as Error
    ElMessage.error({
      message: `计算失败: ${error.message}`,
      customClass: 'error',
      grouping: true
    })
  }
}

const initData = () => {
  const data = getData('hash_calc') as HashCalcDataType
  originValue.value = data.data?.originValue
  handleInput(originValue.value)
}

const clear = () => {
  if (activeType.value === 'text') {
    originValue.value = ''
    handleInput('')
  }

  if (activeType.value === 'file') {
    file.value = ''
  }

  setData('hash_calc', {
    time: new Date().getTime(),
    data: {originValue: originValue.value}
  } as HashCalcDataType)
}

onMounted(() => {
  initData()

  eventBus.on('clear', clear)
})

onUnmounted(() => {
  setData('hash_calc', {
    time: new Date().getTime(),
    data: {originValue: originValue.value}
  } as HashCalcDataType)

  eventBus.off('clear', clear)
})
</script>

<template>
  <div
      class="w-full h-full p-2 grid grid-rows-5 grid-cols-2 gap-2 dark:bg-[#252525] rounded-md"
  >
    <div class="h-full col-1 row-span-full relative">
      <div v-if="activeType === 'text'" class="h-full">
        <el-input
            v-model="originValue"
            type="textarea"
            placeholder="请输入文本..."
            resize="none"
            class="h-full"
            @input="handleInput"
        />
      </div>
      <div
          v-else
          class="h-full w-full border dark:border-[#4C4D4F] border-[#DCDFE6] hover:border-[#29a745] rounded-md"
      >
        <div class="h-full w-full flex flex-col items-center justify-center">
          <el-button class="button-with-bg" @click="selectFile">
            <template #icon>
              <upload-one/>
            </template>
            上传文件
          </el-button>
          <div
              v-if="file && file !== ''"
              class="flex items-center gap-1 text-xs mt-2 text-[#A8ABB2] dark:text-[#8C9095]"
          >
            <check-one theme="filled"/>
            {{ file }}
          </div>
        </div>
      </div>
      <div class="flex flex-wrap items-center absolute top-1 right-1 dropdown">
        <el-dropdown trigger="click" popper-class="custom-dropdown" @command="handleCommand">
          <el-button plain size="small">
            {{ activeMenu?.label }}
            <down class="pl-1"/>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item
                  v-for="menu in dropdownMenuList"
                  :key="menu.command"
                  :command="menu.command"
              >
                {{ menu.label }}
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>
    <div class="h-full col-2 row-1 relative">
      <el-input
          v-model="md5Value"
          type="textarea"
          placeholder="md5"
          resize="none"
          readonly
          class="h-full disable"
      />
      <el-button
          size="small"
          class="button-with-bg absolute bottom-1 right-1"
          @click="copy(md5Value)"
      >
        md5
      </el-button>
    </div>
    <div class="h-full col-2 row-2 relative">
      <el-input
          v-model="sha1Value"
          type="textarea"
          placeholder="sha1"
          resize="none"
          readonly
          class="h-full disable"
      />
      <el-button
          size="small"
          class="button-with-bg absolute bottom-1 right-1"
          @click="copy(sha1Value)"
      >
        sha1
      </el-button>
    </div>
    <div class="h-full col-2 row-3 relative">
      <el-input
          v-model="sha256Value"
          type="textarea"
          placeholder="sha256"
          resize="none"
          readonly
          class="h-full disable"
      />
      <el-button
          size="small"
          class="button-with-bg absolute bottom-1 right-1"
          @click="copy(sha256Value)"
      >
        sha256
      </el-button>
    </div>
    <div class="h-full col-2 row-4 relative">
      <el-input
          v-model="sha512Value"
          type="textarea"
          placeholder="sha512"
          resize="none"
          readonly
          class="h-full disable"
      />
      <el-button
          size="small"
          class="button-with-bg absolute bottom-1 right-1"
          @click="copy(sha512Value)"
      >
        sha512
      </el-button>
    </div>
    <div class="h-full col-2 row-5 relative">
      <el-input
          v-model="sm3Value"
          type="textarea"
          placeholder="sm3"
          resize="none"
          readonly
          class="h-full disable"
      />
      <el-button
          size="small"
          class="button-with-bg absolute bottom-1 right-1"
          @click="copy(sm3Value)"
      >
        sm3
      </el-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
:deep(.el-textarea) {
  --el-input-placeholder-color: #a8abb2;
}

:deep(.el-textarea__inner) {
  height: 100%;
  @apply text-xs dark:bg-[#212123] dark:text-slate-300;
  box-shadow: none !important;
  border: #dcdfe6 solid 1px;

  @media (prefers-color-scheme: dark) {
    border: #4c4d4f solid 1px;
  }

  &:focus {
    box-shadow: none !important;
    border: #29a745 solid 1px;
  }
}

.disable {
  :deep(.el-textarea__inner) {
    @apply cursor-default;
    outline: none !important;

    &:focus {
      border: #dcdfe6 solid 1px !important;

      @media (prefers-color-scheme: dark) {
        border: #4c4d4f solid 1px !important;
      }
    }
  }
}

:deep(.el-button) {
  @apply text-xs font-normal;
}
</style>
