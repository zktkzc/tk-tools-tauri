<script setup lang="ts">
import {ElMessage} from 'element-plus'
import {onMounted, onUnmounted, ref} from "vue";
import {useDataStore} from "../store/useDataStore.ts";
import {invoke} from "@tauri-apps/api/core";
import {Base64DecryptDataType} from "../types";
import {eventBus} from "../utils/eventBus.ts";
import {DamageMap} from '@icon-park/vue-next'
import dayjs from "dayjs";

const {getData, setData} = useDataStore()
const originValue = ref<string>('')
const result = ref<string>('')
const urlSafe = ref<boolean>(false)
const hasPad = ref<boolean>(false)
const resultType = ref<'text' | 'file' | 'image'>('text')
const previewRef = ref<HTMLImageElement | null>(null)
const showDownload = ref<boolean>(false)
const showPreview = ref<boolean>(false)
const filename = ref<string>('')
let mimeType = ''

const base64Decode = async () => {
  showDownload.value = false
  showPreview.value = false
  filename.value = ''
  mimeType = ''

  if (!originValue.value || originValue.value === '') {
    result.value = ''
    return
  }

  if (resultType.value === 'text') {
    invoke('base64_decode', {value: originValue.value, safe: urlSafe.value, pad: hasPad.value}).then(data => {
      if (data) {
        const uint8Array = new Uint8Array(data as any)
        const decoder = new TextDecoder('utf-8')
        result.value = decoder.decode(uint8Array)
      }
    }).catch(e => {
      ElMessage.error({
        message: `转换失败: ${e}`,
        grouping: true,
        customClass: 'error'
      })
    });
    saveData()
  } else if (resultType.value === 'image' || resultType.value === 'file') {
    invoke("get_mime_type_from_base64_str", {
      value: originValue.value,
      safe: urlSafe.value,
      pad: hasPad.value
    }).then(data => {
      mimeType = data as string
      if ((data as string) === '') mimeType = 'text/plain'

      if (resultType.value === 'image') {
        if (!(data as string).includes('image')) {
          ElMessage.error({
            message: '该Base64不是一个图片文件',
            grouping: true,
            customClass: 'error'
          })
          return
        } else {
          showPreview.value = true
          previewRef.value!.src = `data:${data};base64,${originValue.value}`
        }
      }

      showDownload.value = true
      filename.value = `TK-${dayjs(new Date()).format('YYYY-MM-DD-HH-mm-ss')}`
    })
  }
}

const download = () => {
  // 去掉base64中的特殊字符，解码为二进制数据
  const byteCharacters = atob(originValue.value.replace(/\s/g, ''))
  const byteLength = byteCharacters.length
  const byteArray = new Uint8Array(byteLength)

  // 遍历二进制数据，填充到Uint8Array中
  for (let i = 0; i < byteLength; i++) {
    byteArray[i] = byteCharacters.charCodeAt(i)
  }

  // 创建Blob对象（基于二进制数据和MIME类型）
  const blob = new Blob([byteArray], {type: mimeType})

  // 生成Blob临时URL（浏览器会自动管理此URL的生命周期）
  const blobUrl = URL.createObjectURL(blob);

  let link = document.createElement('a')
  link.href = blobUrl
  link.download = filename.value
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}

const copy = (value: string): void => {
  if (value === '') return
  navigator.clipboard.writeText(value)
  ElMessage.success({message: '复制成功', grouping: true, customClass: 'success'})
}

const initData = () => {
  const data = getData('base64_decrypt') as Base64DecryptDataType
  originValue.value = data.data?.originValue || ''
  result.value = data.data?.result || ''
  urlSafe.value = data.data?.urlSafe || false
  hasPad.value = data.data?.hasPad || false
}

const saveData = () => {
  setData('base64_decrypt', {
    time: new Date().getTime(),
    data: {
      originValue: originValue.value,
      result: result.value,
      urlSafe: urlSafe.value,
      hasPad: hasPad.value,
    }
  } as Base64DecryptDataType)
}

const clear = () => {
  originValue.value = ''
  result.value = ''
  mimeType = ''

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
  <div class="w-full h-full grid grid-rows-[100px_30px_auto] gap-2">
    <div class="h-full">
      <el-input
          v-model="originValue"
          type="textarea"
          placeholder="输入..."
          resize="none"
      />
    </div>
    <div class="h-full flex items-center gap-2">
      <el-button @click="async () => await base64Decode()">转换</el-button>
      <el-checkbox v-model="urlSafe" @change="saveData()">
        Url安全
      </el-checkbox>
      <el-checkbox v-model="hasPad" @change="saveData()">
        是否有填充
      </el-checkbox>
      <el-button @click="copy(result)">复制</el-button>
      <div class="flex items-center">
        <div
            class="w-[60px] h-[32px] font-normal text-[15px] flex items-center justify-center rounded-tl-md rounded-bl-md border border-r-0 text-[#A1A3A9] dark:text-[#bdc6cd] dark:border-[#4C4D4F] bg-[#F5F7FA] dark:bg-[#333]"
            style="user-select: none; cursor: default"
        >
          类型:
        </div>
        <div class="h-full w-[85px] text-center">
          <el-select
              v-model="resultType"
              popper-class="custom-select"
              @change="(value: 'text' | 'file' | 'image') => (resultType = value)"
          >
            <el-option label="文本" value="text"/>
            <el-option label="文件" value="file"/>
            <el-option label="图片" value="image"/>
          </el-select>
        </div>
      </div>
    </div>
    <div class="h-full">
      <el-input
          v-if="resultType === 'text'"
          v-model="result"
          type="textarea"
          placeholder="输出..."
          resize="none"
          readonly
          class="disable"
      />
      <div v-else-if="resultType === 'image' || resultType === 'file'"
           class="w-full h-full border rounded-md">
        <div v-show="showDownload" class="w-full h-full flex md:flex-col items-center justify-center gap-2"
             style="user-select: none; cursor: default">
          <img v-show="showPreview" ref="previewRef" src="" alt=""
               class="w-[30vmin] border border-dashed border-black"/>
          <div class="text-sm text-[#A8ABB2]">{{ filename }}</div>
          <div class="text-sm text-[#A8ABB2]">文件类型:&nbsp;{{ mimeType }}</div>
          <el-button @click="download">下载</el-button>
        </div>
        <div v-show="!showDownload" class="w-full h-full flex flex-col items-center justify-center gap-1">
          <damage-map v-show="resultType === 'image'" size="100" :stroke-width="2" class="text-[#919399]"/>
          <div class="text-[#919399]">无数据</div>
        </div>
      </div>
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

.el-button + .el-button {
  margin-left: 0;
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

:deep(.el-select__wrapper) {
  @apply dark:bg-[#252525] shadow-none hover:shadow-none border rounded-tr-md rounded-br-md rounded-tl-none rounded-bl-none border-[#DCDFE6] dark:border-[#4C4D4F] hover:border-[#29a745];
  height: 32px;
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