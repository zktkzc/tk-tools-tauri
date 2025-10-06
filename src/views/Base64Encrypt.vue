<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref} from "vue";
import {useDataStore} from "../store/useDataStore.ts";
import {Base64EncryptDataType} from "../types";
import {eventBus} from "../utils/eventBus.ts";
import {CheckOne, Down, UploadOne} from '@icon-park/vue-next'
import {invoke} from "@tauri-apps/api/core";
import {ElMessage} from 'element-plus'
import {platform} from '@tauri-apps/plugin-os'
import {open} from '@tauri-apps/plugin-dialog'

const dropdownMenuList = [
  {command: 'text', label: '文本'},
  {command: 'file', label: '文件'}
]
const activeMenu = computed(() => {
  return dropdownMenuList.find((menu) => menu.command === activeType.value)
})

const {setData, getData} = useDataStore()
const activeType = ref<'text' | 'file'>('text')
const originValue = ref<string>('')
const result = ref<string>('')
const urlSafe = ref<boolean>(false)
const hasPad = ref<boolean>(false)
const fileCalcFlag = ref<'data-url' | 'css' | 'html' | 'null'>('null')
const filepath = ref<string>('')
const file = computed(() => {
  if (!filepath.value) return ''

  if (platform() === 'windows') {
    return filepath.value.substring(filepath.value.lastIndexOf('\\') + 1)
  } else {
    return filepath.value.substring(filepath.value.lastIndexOf('/') + 1)
  }
})

const handleInput = async (value: string) => {
  if (!value || value === '') {
    result.value = ''
    return
  }

  result.value = await invoke('calculate_text_base64', {value: value, safe: urlSafe.value, pad: hasPad.value})
  saveData()
}

const handleCommand = (type: 'text' | 'file'): void => {
  activeType.value = type
  result.value = ''

  if (type === 'text') {
    filepath.value = ''
    initData()
  }
}

const calcFileBase64 = (path: string) => {
  if (!filepath.value || !path || filepath.value === '' || path === '') return

  try {
    invoke("calculate_file_base64", {path: path, safe: urlSafe.value, pad: hasPad.value, flag: fileCalcFlag.value})
        .then((data: any) => {
          result.value = data
        })
  } catch (e) {
    const error = e as Error
    ElMessage.error({
      message: `加密失败: ${error.message}`,
      customClass: 'error',
      grouping: true
    })
  }
}

const selectFile = async () => {
  filepath.value = await open({
    title: '选择文件'
  }) as string

  calcFileBase64(filepath.value)
}

const copy = (value: string): void => {
  if (value === '') return
  navigator.clipboard.writeText(value)
  ElMessage.success({message: '复制成功', grouping: true, customClass: 'success'})
}

const initData = () => {
  const data = getData('base64_encrypt') as Base64EncryptDataType
  originValue.value = data.data?.originValue || ''
  result.value = data.data?.result || ''
  urlSafe.value = data.data?.urlSafe || false
  hasPad.value = data.data?.hasPad || false
  fileCalcFlag.value = data.data?.fileCalcFlag || 'null'
  handleInput(originValue.value)
}

const saveData = () => {
  setData('base64_encrypt', {
    time: new Date().getTime(),
    data: {
      originValue: originValue.value,
      result: result.value,
      urlSafe: urlSafe.value,
      hasPad: hasPad.value,
      fileCalcFlag: fileCalcFlag.value,
    }
  } as Base64EncryptDataType)
}

const clear = () => {
  if (activeType.value === 'text') {
    originValue.value = ''
    handleInput('')
  }

  if (activeType.value === 'file') {
    filepath.value = ''
  }

  saveData()
}

onMounted(() => {
  initData()

  eventBus.on('clear', clear)
})

onUnmounted(() => {
  saveData()
  eventBus.off('clear', clear)
})
</script>

<template>
  <div class="w-full h-full grid grid-rows-[100px_auto_30px] gap-2">
    <div class="h-full relative">
      <div class="h-full" v-if="activeType === 'text'">
        <el-input
            v-if="activeType === 'text'"
            v-model="originValue"
            type="textarea"
            placeholder="输入..."
            resize="none"
            @input="handleInput"
        />
      </div>
      <div
          v-else
          class="h-full border dark:border-[#4C4D4F] border-[#DCDFE6] hover:border-[#29a745] rounded-md"
      >
        <div class="h-full flex flex-col items-center justify-center p-2">
          <el-button class="upload-btn" @click="selectFile">
            <template #icon>
              <upload-one/>
            </template>
            上传文件
          </el-button>
          <div
              v-if="file && filepath && file !== '' && filepath !== ''"
              class="w-full flex items-center justify-center gap-1 text-xs mt-2 text-[#A8ABB2] dark:text-[#8C9095]"
              style="user-select: none"
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
    <div>
      <el-input
          v-model="result"
          type="textarea"
          placeholder="输出..."
          resize="none"
          readonly
          class="disable"
      />
    </div>
    <div class="h-full flex items-center justify-center gap-2">
      <el-button @click="copy(result)">复制</el-button>
      <el-checkbox v-model="urlSafe" @change="() => {
        if (activeType === 'text') {
          handleInput(originValue)
        } else {
          calcFileBase64(filepath)
        }
      }"
      >
        Url安全
      </el-checkbox>
      <el-checkbox v-model="hasPad" @change="() => {
        if (activeType === 'text') {
          handleInput(originValue)
        } else {
          calcFileBase64(filepath)
        }
      }">
        是否有填充
      </el-checkbox>
      <el-radio-group v-show="activeType === 'file'" v-model="fileCalcFlag"
                      @change="() => calcFileBase64(filepath)"
                      fill="#29a745">
        <el-radio-button label="无" value="null"/>
        <el-radio-button label="DataUrl" value="data-url"/>
        <el-radio-button label="CSS" value="css"/>
        <el-radio-button label="HTML" value="html"/>
      </el-radio-group>
    </div>
  </div>
</template>

<style scoped lang="scss">
:deep(.el-textarea) {
  height: 100%;
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
  --el-button-bg-color: #ffffff;
  --el-button-text-color: #515A6E;
  --el-button-border-color: #dddfe5;
  --el-button-hover-bg-color: #fff;
  --el-button-hover-border-color: #23923d;
  --el-button-hover-text-color: #23923d;
  --el-button-active-bg-color: #ffffff;
  --el-button-active-border-color: #23923d;

  @media (prefers-color-scheme: dark) {
    --el-button-bg-color: #252525;
    --el-button-border-color: #4c4d4f;
    --el-button-text-color: #bbc6ce;
    --el-button-hover-bg-color: #252525;
    --el-button-hover-border-color: #23923d;
    --el-button-hover-text-color: #23923d;
    --el-button-active-bg-color: #252525;
    --el-button-active-border-color: #23923d;
  }
}

:deep(.el-checkbox) {
  @apply border text-[#515A6E] dark:border-[#4C4D4F] dark:text-[#BBC6CE] hover:border-[#29A745] hover:text-[#29A745]
  m-0 px-2 rounded-md cursor-pointer;

  &:hover {
    .el-checkbox__inner {
      @apply border-[#29A745];
    }
  }
}

:deep(.el-checkbox__inner) {
  @apply dark:bg-[#202124] border border-[#DCDFE6] dark:border-[#4C4D4F];
}

:deep(.el-checkbox__input.is-checked .el-checkbox__inner) {
  @apply bg-[#29A745] border border-[#29A745];
}

:deep(.el-checkbox__input.is-checked + .el-checkbox__label) {
  @apply text-[#29A745];
}

.dropdown {
  :deep(.el-button) {
    @apply font-normal dark:text-[#B7C3CB] dark:border-[#4c4d4f];
    --el-button-hover-border-color: #29a745;
    --el-button-hover-text-color: #29a745;

    &:hover {
      border-color: #29a745 !important;
      color: #29a745 !important;
    }
  }
}

:deep(.el-button) {
  &.upload-btn {
    --el-button-text-color: #fff;
    --el-button-bg-color: #29a745;
    --el-button-border-color: #29a745;
    --el-button-hover-text-color: #fff;
    --el-button-hover-bg-color: #23923d;
    --el-button-hover-border-color: #23923d;
    --el-button-active-bg-color: #23923d;
    --el-button-active-border-color: #23923d;
  }
}
</style>