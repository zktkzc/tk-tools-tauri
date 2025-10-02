<script setup lang="ts">
import { ref } from 'vue'
import { CheckOne, UploadOne } from '@icon-park/vue-next'
import { ElMessage } from 'element-plus'
import {invoke} from "@tauri-apps/api/core";
import {platform} from "@tauri-apps/plugin-os";
import {HashResult} from "../types";
import {open} from '@tauri-apps/plugin-dialog'

const calcValue = ref<string>('')
const targetValue = ref<string>('')
const algorithmValue = ref<string>('md5')
const result = {
  md5: '',
  sha1: '',
  sha256: '',
  sha512: '',
  sm3: ''
}

const file = ref('')
const selectFile = async () => {
  const path = await open({
    title: '选择文件'
  }) as string
  try {
    invoke("calculate_file_hash", {path})
        .then((data: any) => {
          const {md5, sha1, sha256, sha512, sm3} = data as HashResult
          result.md5 = md5
          result.sha1 = sha1
          result.sha256 = sha256
          result.sha512 = sha512
          result.sm3 = sm3

          if (platform() === 'windows') {
            file.value = path.substring(path.lastIndexOf('\\') + 1)
          } else {
            file.value = path.substring(path.lastIndexOf('/') + 1)
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

const check = () => {
  if (file.value.length === 0) {
    ElMessage.error({ message: '请选择文件', grouping: true, customClass: 'error' })
    return
  }

  if (targetValue.value.length === 0) {
    ElMessage.error({ message: '请输入目标值', grouping: true, customClass: 'error' })
    return
  }

  calcValue.value = result[algorithmValue.value as keyof typeof result]
  if (calcValue.value === targetValue.value) {
    ElMessage.success({ message: '计算值与目标值一致', grouping: true, customClass: 'success' })
  } else {
    ElMessage.error({ message: '计算值与目标值不一致', grouping: true, customClass: 'error' })
  }
}
</script>

<template>
  <div
    class="w-full h-full p-2 flex flex-col justify-center gap-2 dark:bg-[#252525]"
    style="user-select: none"
  >
    <div class="w-full h-[200px]">
      <div
        class="h-full w-full border dark:border-[#4C4D4F] border-[#DCDFE6] hover:border-[#29a745] rounded-md"
      >
        <div class="h-full w-full flex flex-col items-center justify-center">
          <el-button class="upload-btn" @click="selectFile">
            <template #icon>
              <upload-one />
            </template>
            上传文件
          </el-button>
          <div
            v-if="file && file !== ''"
            class="flex items-center gap-1 text-xs mt-2 text-[#A8ABB2] dark:text-[#8C9095]"
          >
            <check-one theme="filled" />
            {{ file }}
          </div>
        </div>
      </div>
    </div>
    <div class="w-full flex-1">
      <el-input
        v-model="calcValue"
        type="textarea"
        placeholder="计算值"
        resize="none"
        readonly
        class="h-full disable"
      />
    </div>
    <div class="w-full flex-1">
      <el-input
        v-model="targetValue"
        type="textarea"
        placeholder="目标值"
        resize="none"
        class="h-full"
      />
    </div>
    <div class="h-[50px] border border-[#dcdfe6] dark:border-[#4c4d4f] rounded-md overflow-auto">
      <div class="h-full flex items-center gap-2 p-1">
        <div class="col-span-4 row-1 flex items-center">
          <div
            class="w-[60px] h-[40px] flex items-center justify-center rounded-tl-md rounded-bl-md border border-r-0 text-[#A1A3A9] dark:text-[#bdc6cd] dark:border-[#4C4D4F] bg-[#F5F7FA] dark:bg-[#333]"
          >
            算法:
          </div>
          <div class="h-full w-[100px] text-center">
            <el-select
              v-model="algorithmValue"
              popper-class="custom-select"
              @change="(value: string) => (algorithmValue = value)"
            >
              <el-option label="MD5" value="md5" />
              <el-option label="SHA1" value="sha1" />
              <el-option label="SHA256" value="sha256" />
              <el-option label="SHA512" value="sha512" />
              <el-option label="SM3" value="sm3" />
            </el-select>
          </div>
        </div>
        <div class="h-[40px] flex-1">
          <el-button class="w-full h-full" style="height: 100%" @click="check">比对</el-button>
        </div>
      </div>
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
  outline: none;
  border: #dcdfe6 solid 1px;

  @media (prefers-color-scheme: dark) {
    border: #4c4d4f solid 1px;
  }

  &:focus {
    box-shadow: none !important;
    outline: none !important;
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
  --el-button-text-color: #fff;
  --el-button-bg-color: #29a745;
  --el-button-border-color: #29a745;
  --el-button-hover-text-color: #fff;
  --el-button-hover-bg-color: #23923d;
  --el-button-hover-border-color: #23923d;
  --el-button-active-bg-color: #23923d;
  --el-button-active-border-color: #23923d;
}

:deep(.el-select__wrapper) {
  @apply dark:bg-[#252525] shadow-none hover:shadow-none border rounded-tr-md rounded-br-md rounded-tl-none rounded-bl-none border-[#DCDFE6] dark:border-[#4C4D4F] hover:border-[#29a745];
  height: 40px;
}

:deep(.el-select__placeholder) {
  @apply text-[#545C70] dark:text-[#bbc6ce] hover:text-[#29a745];
}


:deep(.el-select) {
  &:hover {
    .el-select__placeholder {
      @apply text-[#29A745];
    }
  }
}
</style>
