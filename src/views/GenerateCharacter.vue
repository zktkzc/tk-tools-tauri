<script lang="ts" setup>
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { Refresh, Setting } from '@icon-park/vue-next'
import CharactersSettingDialog from '../components/CharactersSettingDialog.vue'
import { computedAsync } from '@vueuse/core'
import { useDataStore } from '../store/useDataStore'
import { GenerateCharacterDataType } from '../types'
import {invoke} from "@tauri-apps/api/core";
import {eventBus} from "../utils/eventBus.ts";
import {getCurrentWindow} from "@tauri-apps/api/window";

const { getData, setData } = useDataStore()
let records: string[] = []
const result = ref<string>('')
const length = ref<number>(32)
const count = ref<number>(1)
const characters = ref<string>('')
const split = ref<string>(',\\n')
const showDialog = ref<boolean>(false)
const needQuotes = ref<boolean>(false)
const checkList = ref<string[]>(['number', 'slow', 'up'])
const charactersSettingDialogRef = ref()
const themeMode = computedAsync(async () => {
  return await invoke('get_theme')
})
const handleSubmit = (value: string, checkValue: string[]): void => {
  characters.value = value
  checkList.value = checkValue
  showDialog.value = false
  storeData()
  if (result.value !== '') generate()
}

const getRandomInt = (min: number, max: number): number => {
  return Math.floor(Math.random() * (max - min + 1)) + min
}

const generate = (): void => {
  records = []

  for (let i = 0; i < count.value; i++) {
    let record = '"'
    for (let j = 0; j < length.value; j++) {
      if (characters.value.length === 0) {
        record += '"'
        break
      }

      const index = getRandomInt(0, characters.value.length - 1)
      record += characters.value[index]

      if (j === length.value - 1) record += '"'
    }
    records.push(record)
  }

  handleResult()

  storeData()
}

const handleResult = (): void => {
  result.value = ''
  const splitStr = split.value.replaceAll('\\n', '\n')
  if (needQuotes.value) result.value = records.join(splitStr)
  else result.value = records.join(splitStr).replaceAll('"', '')
}

const initData = () => {
  const data = getData('gen_ch') as GenerateCharacterDataType
  result.value = data.data?.result
  length.value = data.data?.length || 32
  count.value = data.data?.count || 1
  characters.value = data.data?.characters
  split.value = data.data?.split || ',\\n'
  needQuotes.value = data.data?.needQuotes || false
  checkList.value = data.data?.checkList || ['number', 'slow', 'up']
  records = data.data?.records || []
}

const storeData = () => {
  setData('gen_ch', {
    time: new Date().getTime(),
    data: {
      result: result.value,
      length: length.value,
      count: count.value,
      characters: characters.value,
      split: split.value,
      needQuotes: needQuotes.value,
      checkList: checkList.value,
      records: records
    }
  } as GenerateCharacterDataType)
}

const clear = () => {
  result.value = ''
  storeData()
}

const unlisten = await getCurrentWindow().onThemeChanged(({payload: theme}) => {
  themeMode.value = theme
})

onMounted(() => {
  initData()
  characters.value = charactersSettingDialogRef.value.setPreset(checkList.value)

  eventBus.on('clear', clear)
})

onUnmounted(() => {
  storeData()
  eventBus.off('clear', clear)
  unlisten()
})

watch(
  () => split.value,
  (): void => {
    handleResult()
  },
  { immediate: true, deep: true }
)
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
          <el-checkbox class="check-box-with-border" v-model="needQuotes" label="添加引号" size="small" @change="handleResult" />
          <el-input v-model="split" class="split-input">
            <template #prepend>分隔符</template>
          </el-input>
        </div>
      </div>
    </div>
    <div
      class="h-[42px] w-full border border-[#DCDFE6] dark:border-[#4c4d4f] flex items-center justify-center rounded-md"
    >
      <div class="w-[60%] flex items-center justify-center gap-1 setting-panel">
        <div class="flex w-[60%] h-full items-center characters-setting">
          <el-input
            v-model="characters"
            class="flex-1"
            readonly
            placeholder="请点击设置来配置字符..."
          />
          <el-button
            class="button-no-bg w-[32px]"
            @click="
              () => {
                showDialog = true
                characters = charactersSettingDialogRef.setPreset(checkList)
              }
            "
          >
            <el-tooltip content="设置" placement="top" :effect="themeMode">
              <setting theme="outline" size="18" />
            </el-tooltip>
          </el-button>
        </div>
        <div class="w-[200px]">
          <el-input v-model="length" type="number" min="0" @change="generate">
            <template #prepend>长度</template>
          </el-input>
        </div>
        <div class="w-[200px]">
          <el-input v-model="count" type="number" min="0" @change="generate">
            <template #prepend>数量</template>
          </el-input>
        </div>
        <el-button class="button-no-bg w-[32px]" @click="generate">
          <refresh theme="outline" size="18" />
        </el-button>
      </div>
    </div>
  </div>
  <characters-setting-dialog
    ref="charactersSettingDialogRef"
    :show-dialog="showDialog"
    @close-dialog="showDialog = false"
    @submit="handleSubmit"
  />
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

.characters-setting {
  :deep(.el-input__wrapper) {
    @apply rounded-tr-none rounded-br-none h-[32px] border-r-0 cursor-default;
  }

  :deep(.el-input__inner) {
    @apply cursor-default;
  }

  :deep(.el-button) {
    @apply rounded-tl-none rounded-bl-none;
  }
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
