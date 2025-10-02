<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { Refresh } from '@icon-park/vue-next'
import { v4 } from 'uuid'
import { GenerateUUIDDataType } from '../types'
import { useDataStore } from '../store/useDataStore'
import {eventBus} from "../utils/eventBus.ts";

let records: string[] = []
const result = ref<string>('')
const count = ref<number>(1)
const split = ref<string>(',\\n')
const needQuotes = ref<boolean>(false)
const needHyphen = ref<boolean>(true)
const needUpperCase = ref<boolean>(false)
const needToUint8 = ref<boolean>(false)
const { getData, setData } = useDataStore()

const generate = (): void => {
  records = []

  for (let i = 0; i < count.value; i++) {
    records.push(v4())
  }

  result.value = records.map((record) => '"' + record + '"').join(getSplitStr())

  handleResult()

  handleHyphenChange()

  handleToUint8Change()

  storeData()
}

const handleResult = (): void => {
  handleQuoteChange()

  handleUpperCaseChange()
}

const uuidToUint8Array = (uuid: string): string => {
  const buffer = new ArrayBuffer(16)
  const view = new DataView(buffer)
  for (let i = 0; i < 16; i++) {
    const byteValue = parseInt(uuid.replaceAll('-', '').substr(i * 2, 2), 16)
    view.setUint8(i, byteValue)
  }
  return new Uint8Array(buffer).toString()
}

const getSplitStr = (): string => {
  return split.value.replaceAll('\\n', '\n')
}

const handleToUint8Change = (): void => {
  if (!needToUint8.value) {
    result.value = records.join(getSplitStr())
    handleResult()
  } else {
    result.value = records
      .map((record) => {
        const arrayStr = uuidToUint8Array(record)
        return '[' + arrayStr + ']'
      })
      .join(getSplitStr())
    handleResult()
  }
}

const handleQuoteChange = (): void => {
  if (!needQuotes.value) result.value = result.value.replaceAll('"', '')
  else {
    result.value = result.value
      .split(getSplitStr())
      .map((record) => '"' + record + '"')
      .join(getSplitStr())
  }
}

const handleHyphenChange = (): void => {
  if (needToUint8.value) return
  if (!needHyphen.value) result.value = result.value.replaceAll('-', '')
  else {
    result.value = records.join(getSplitStr())
    handleResult()
  }
}

const handleUpperCaseChange = (): void => {
  if (needToUint8.value) return
  if (needUpperCase.value) result.value = result.value.toUpperCase()
  else result.value = result.value.toLowerCase()
}

const handleSplitInput = (newValue: string): void => {
  const oldValue = (getData('gen_uuid') as GenerateUUIDDataType).data.split
  if (newValue && oldValue && newValue !== oldValue) {
    result.value = result.value.replaceAll(
      oldValue.replaceAll('\\n', '\n'),
      newValue.replaceAll('\\n', '\n')
    )
    storeData()
  }
}

const initData = () => {
  const data = getData('gen_uuid') as GenerateUUIDDataType
  result.value = data.data?.result
  count.value = data.data?.count || 1
  split.value = data.data?.split || ',\\n'
  needQuotes.value = data.data?.needQuotes || false
  needHyphen.value = data.data?.needHyphen || true
  needUpperCase.value = data.data?.needUpperCase || false
  needToUint8.value = data.data?.needToUint8 || false
  records = data.data?.records || []
}

const storeData = () => {
  setData('gen_uuid', {
    time: new Date().getTime(),
    data: {
      result: result.value,
      count: count.value,
      split: split.value,
      needQuotes: needQuotes.value,
      needHyphen: needHyphen.value,
      needUpperCase: needUpperCase.value,
      needToUint8: needToUint8.value,
      records: records
    }
  } as GenerateUUIDDataType)
}

const clear = () => {
  result.value = ''
  storeData()
}

onMounted(() => {
  initData()

  eventBus.on('clear', clear)
})

onUnmounted(() => {
  storeData()
  eventBus.off('clear', clear)
})
</script>

<template>
  <div class="h-full flex flex-col justify-center items-center gap-2">
    <div class="w-full h-full flex-1">
      <div class="h-full relative">
        <el-input
          v-model="result"
          type="textarea"
          resize="none"
          class="h-full font-mono"
          readonly
          placeholder="输出..."
        />
        <div class="absolute bottom-1 right-1 flex items-center gap-1">
          <el-checkbox
            v-model="needQuotes"
            style="height: 25px !important"
            label="添加引号"
            size="small"
            @change="handleResult"
          />
          <el-input v-model="split" class="split-input" @input="handleSplitInput">
            <template #prepend>分隔符</template>
          </el-input>
        </div>
      </div>
    </div>
    <div
      class="h-[42px] w-full border border-[#DCDFE6] dark:border-[#4c4d4f] flex items-center justify-center rounded-md"
    >
      <div class="w-[60%] flex items-center justify-center gap-1 setting-panel">
        <div class="w-[130px]">
          <el-input v-model="count" type="number" min="0" @change="generate">
            <template #prepend>数量</template>
          </el-input>
        </div>
        <el-checkbox
          v-model="needHyphen"
          label="连接符(-)"
          size="small"
          @change="handleHyphenChange"
        />
        <el-checkbox
          v-model="needUpperCase"
          label="大写"
          size="small"
          @change="handleUpperCaseChange"
        />
        <el-checkbox
          v-model="needToUint8"
          label="Uint8数组"
          size="small"
          @change="handleToUint8Change"
        />
        <el-button class="w-[32px]" @click="generate">
          <refresh theme="outline" size="18" />
        </el-button>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
:deep(.el-textarea__inner) {
  height: 100%;
  box-shadow: none !important;
  @apply dark:bg-[#212123] dark:text-[#BDC6CD];
  outline: none;
  border: #dcdfe6 solid 1px;

  @media (prefers-color-scheme: dark) {
    border: #4c4d4f solid 1px;
  }

  &:focus {
    box-shadow: none !important;
    outline: none !important;
  }
}

.split-input {
  height: 25px;
  width: 120px;
  font-size: 12px;

  :deep(.el-input__inner) {
    @apply h-full;
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

:deep(.el-button) {
  --el-button-bg-color: #ffffff;
  --el-button-border-color: #dcdfe6;
  --el-button-hover-bg-color: #ffffff;
  --el-button-hover-border-color: #29a745;
  --el-button-hover-text-color: #29a745;
  --el-button-active-border-color: #29a745;

  @media (prefers-color-scheme: dark) {
    --el-button-bg-color: #202124;
    --el-button-border-color: #4c4d4f;
    --el-button-hover-bg-color: #202124;
    --el-button-hover-border-color: #29a745;
    --el-button-hover-text-color: #29a745;
    --el-button-active-border-color: #29a745;
  }
}

:deep(.el-checkbox) {
  @apply border text-[#515A6E] hover:border-[#29A745] dark:border-[#4C4D4F] dark:text-[#BBC6CE] dark:hover:border-[#29A745] dark:hover:text-[#29A745]
  m-0 px-2 rounded-md cursor-pointer;

  --el-checkbox-input-border-color-hover: #29a745;
  height: 32px !important;
}

:deep(.el-checkbox__inner) {
  @apply dark:bg-[#202124] border dark:border-[#4C4D4F] dark:hover:border-[#29A745];
}

:deep(.el-checkbox__input.is-checked .el-checkbox__inner) {
  @apply bg-[#29A745] border border-[#29A745];
}

:deep(.el-checkbox__input.is-checked + .el-checkbox__label) {
  @apply text-[#29A745];
}

.setting-panel {
  :deep(.el-input__wrapper) {
    @apply h-[32px];
  }

  :deep(.el-input-group__prepend) {
    @apply h-[32px];
  }
}
</style>
