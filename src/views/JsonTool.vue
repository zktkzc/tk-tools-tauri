<script setup lang="ts">
import {jsonrepair} from 'jsonrepair'
import {onMounted, onUnmounted, ref} from 'vue'
import {ElMessage} from 'element-plus'
import Editor from '../components/Editor.vue'
import {Down} from '@icon-park/vue-next'
import {format} from 'prettier/standalone'
import babel from 'prettier/plugins/babel'
import estree from 'prettier/plugins/estree'
import {useDataStore} from '../store/useDataStore'
import {JsonToolDataType} from '../types'
import {eventBus} from "../utils/eventBus.ts";

const jsonStr = ref()
const result = ref()
const needTransfer = ref(false)
const needWrap = ref(true)
const {setData, getData} = useDataStore()
const editor1 = ref()
const editor2 = ref()

const copy = () => {
  navigator.clipboard.writeText(result.value)
  ElMessage.success({message: '复制成功', grouping: true, customClass: 'success'})
}

const validate = (): Promise<string> => {
  return format(jsonStr.value, {
    parser: 'json5',
    plugins: [babel, estree],
    quoteProps: 'preserve',
    trailingComma: 'none',
    tabWidth: Number(activeDropItem.value.command),
    printWidth: 1
  })
}

const beautify = (): void => {
  validate()
      .catch((e: SyntaxError) => {
        ElMessage.error({
          message: e.message,
          grouping: true,
          customClass: 'syntax-error'
        })
      })
      .then((value: string | void) => {
        if (value) {
          result.value = value
          copy()
        } else {
          result.value = ''
        }
        saveData()
        editor2.value.updateEditor(result.value)
      })
  saveData()
}

const repair = (): void => {
  try {
    if (!jsonStr.value || jsonStr.value.trim() === '') {
      ElMessage.error({message: '请输入JSON字符串', grouping: true, customClass: 'error'})
      return
    }

    jsonStr.value = jsonrepair(jsonStr.value)
    editor1.value.updateEditor(jsonStr.value)
    beautify()
  } catch (e) {
    ElMessage.error({
      message: `修复失败: ${(e as Error).message}`,
      grouping: true,
      customClass: 'error'
    })
  }
}

const minimal = (): void => {
  validate()
      .catch((e: SyntaxError) => {
        ElMessage.error({
          message: e.message,
          grouping: true,
          customClass: 'syntax-error'
        })
      })
      .then((value: string | void) => {
        if (value) {
          result.value = value.replaceAll(/\s+(?=(?:[^"]*"[^"]*")*[^"]*$)/g, '')
          copy()
        } else {
          result.value = ''
        }
        saveData()
        editor2.value.updateEditor(result.value)
      })
  saveData()
}

const transfer = (): void => {
  if (needTransfer.value) {
    result.value = result.value.replaceAll('"', '\\"')
  } else {
    result.value = result.value.replaceAll('\\"', '"')
  }
  saveData()
  editor2.value.updateEditor(result.value)
}

const autoWrap = () => {
  beautify()
  saveData()
  editor1.value.initEditor()
  editor2.value.initEditor()
}

const dropDownItems = [
  {command: '0', label: '无缩进'},
  {command: '2', label: '缩进空格 2'},
  {command: '4', label: '缩进空格 4'},
  {command: '6', label: '缩进空格 6'},
  {command: '8', label: '缩进空格 8'}
]
const activeDropItem = ref<{ command: string; label: string }>(dropDownItems[2])
const handleCommand = (command: string): void => {
  activeDropItem.value = dropDownItems.find((item) => item.command === command)!
  saveData()
}

const initData = async () => {
  const data = (await getData('json_tool')) as JsonToolDataType
  jsonStr.value = data.data?.jsonStr
  result.value = data.data?.result
  needTransfer.value = data.data?.needTransfer || false
  needWrap.value = data.data?.needWrap || true
  activeDropItem.value =
      (data.data?.activeDropItem as { command: string; label: string }) || dropDownItems[2]
}

const saveData = () => {
  setData('json_tool', {
    time: new Date().getTime(),
    data: {
      jsonStr: jsonStr.value,
      result: result.value,
      needTransfer: needTransfer.value,
      needWrap: needWrap.value,
      activeDropItem: activeDropItem.value
    }
  } as JsonToolDataType)
}

const clear = () => {
  jsonStr.value = ''
  result.value = ''
  saveData()
  editor1.value.updateEditor(jsonStr.value)
  editor2.value.updateEditor(result.value)
}

onMounted(async () => {
  await initData()

  eventBus.on('clear', clear)
})

onUnmounted(() => {
  eventBus.off('clear', clear)
  saveData()
})
</script>

<template>
  <div class="p-2 pb-1 h-full w-full flex flex-col justify-between gap-1 dark:bg-[#252525]">
    <div class="w-full flex-1 max-h-[calc(100vh-107px)]">
      <div class="w-full h-full flex items-center gap-2">
        <div
            class="h-full w-1/2 border border-[#DDDFE5] dark:border-[#4C4D4F] rounded-md p-[1px] overflow-auto"
        >
          <Editor
              ref="editor1"
              v-model="jsonStr"
              lang="json"
              placeholder-str="输入"
              :line-wrap="needWrap"
          />
        </div>
        <div
            class="h-full w-1/2 border border-[#DDDFE5] dark:border-[#4C4D4F] rounded-md p-[1px] overflow-auto"
        >
          <Editor
              ref="editor2"
              v-model="result"
              lang="json"
              placeholder-str="输出"
              :line-wrap="needWrap"
          />
        </div>
      </div>
    </div>
    <div class="w-full h-[40px]">
      <div class="h-full w-full flex items-center justify-center gap-2">
        <el-dropdown
            trigger="click"
            class="dropdown"
            popper-class="custom-dropdown"
            @command="handleCommand"
        >
          <el-button plain>
            {{ activeDropItem?.label }}
            <down class="pl-1"/>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item
                  v-for="item in dropDownItems"
                  :key="item.command"
                  :command="item.command"
              >
                {{ item.label }}
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
        <el-button type="primary" @click="beautify">格式化</el-button>
        <el-button type="primary" @click="repair">JSON修复</el-button>
        <el-button type="primary" @click="minimal">压缩</el-button>
        <el-checkbox v-model="needTransfer" @change="transfer">转义</el-checkbox>
        <el-checkbox v-model="needWrap" @change="autoWrap">自动换行</el-checkbox>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
:deep(.el-textarea__inner) {
  height: 100%;
  outline: #29a745 solid 1px;
  box-shadow: none !important;
  @apply rounded-md;

  &:focus {
    box-shadow: none !important;
  }

  @media (prefers-color-scheme: dark) {
    @apply bg-[#212123] text-slate-300;
  }
}

:deep(.el-button--primary) {
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

.dropdown {
  :deep(.el-button) {
    @apply bg-transparent font-normal text-[#515A6E] dark:text-[#B7C3CB] dark:border-[#4c4d4f];
    height: 32px !important;

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

    &:hover {
      border-color: #29a745 !important;
      color: #29a745 !important;
    }
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
</style>
