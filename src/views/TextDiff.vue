<template>
  <div class="h-[calc(100vh-60px)] w-full flex flex-col items-center justify-between p-2 gap-2">
    <div
      class="flex-1 w-full dark:text-[#ccc] border rounded-md dark:border-[#4C4D4F] border-[#e5e7eb] overflow-auto"
    >
      <div>
        <div ref="container" class="h-full w-full" />
      </div>
    </div>
    <div
      class="h-[40px] w-full border border-[#e5e7eb] dark:border-[#4C4D4F] rounded-md flex items-center justify-center gap-2 p-1"
    >
      <el-checkbox v-model="data.options.lineWrap" @change="autoWrap">自动换行</el-checkbox>
      <el-select
        v-model="data.options.revertControl"
        popper-class="custom-select"
        class="revert-control-select"
        @change="changeRevertControl"
      >
        <el-option label="左 => 右" value="a-to-b" />
        <el-option label="左 <= 右" value="b-to-a" />
      </el-select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { MergeView } from '@codemirror/merge'
import { EditorView, placeholder } from '@codemirror/view'
import { basicSetup } from 'codemirror'
import { useDataStore } from '../store/useDataStore'
import { TextDiffDataType } from '../types'
import {eventBus} from "../utils/eventBus.ts";

const { setData, getData } = useDataStore()

type DataType = {
  a: string
  b: string
  options: {
    lineWrap: boolean
    revertControl: string
  }
}

const data = ref<DataType>({
  a: '',
  b: '',
  options: {
    lineWrap: true,
    revertControl: 'b-to-a'
  }
})

const container = ref<HTMLDivElement | null>(null)
const mergeView = ref<MergeView | null>(null)

const autoWrap = () => {
  initEditor(container.value as HTMLDivElement)
  saveData()
}

const changeRevertControl = () => {
  initEditor(container.value as HTMLDivElement)
  saveData()
}

const initData = () => {
  if (!data.value) return

  const oldData = getData('text_diff') as TextDiffDataType
  data.value.a = oldData.data?.a
  data.value.b = oldData.data?.b
  data.value.options.lineWrap = oldData.data?.options?.lineWrap || true
  data.value.options.revertControl = oldData.data?.options?.revertControl || 'b-to-a'
}

const saveData = async () => {
  setData('text_diff', {
    time: new Date().getTime(),
    data: data.value
  } as TextDiffDataType)
}

const getExtensions = () => {
  return [
    basicSetup,
    ...(data.value.options.lineWrap ? [EditorView.lineWrapping] : []),
    placeholder('输入'),
    EditorView.updateListener.of((update) => {
      if (!mergeView.value) return

      if (update.docChanged) {
        if (update.view.dom.contains(mergeView.value.a.dom as any)) {
          if (update.state.doc.toString() !== data.value.a) {
            data.value.a = update.state.doc.toString()
            saveData()
          }
        }

        if (update.view.dom.contains(mergeView.value.b.dom as any)) {
          if (update.state.doc.toString() !== data.value.b) {
            data.value.b = update.state.doc.toString()
            saveData()
          }
        }
      }
    })
  ]
}

const initEditor = async (element: HTMLDivElement) => {
  if (mergeView.value) {
    mergeView.value.destroy()
  }

  mergeView.value = new MergeView({
    a: {
      doc: data.value.a,
      extensions: getExtensions()
    },
    b: {
      doc: data.value.b,
      extensions: getExtensions()
    },
    parent: element,
    revertControls: data.value.options.revertControl as any,
    highlightChanges: true,
    gutter: true
  })
}

const clear = () => {
  data.value.a = ''
  data.value.b = ''
  initEditor(container.value as HTMLDivElement)
  saveData()
}

onMounted(() => {
  initData()
  setTimeout(() => initEditor(container.value as HTMLDivElement), 100)

  eventBus.on('clear', clear)
})

onUnmounted(() => {
  mergeView.value?.destroy()
  eventBus.off('clear', clear)
  saveData()
})
</script>

<style lang="scss" scoped>
:deep(.cm-mergeView) {
  height: 100% !important;

  .cm-mergeViewEditors {
    height: 100%;

    .cm-editor {
      height: 100% !important;

      .cm-scroller {
        height: 100% !important;
      }
    }
  }
}

:deep(.cm-content) {
  @apply dark:bg-[#212123] dark:-z-30;
}

:deep(.ͼ2.cm-merge-a .cm-changedLineGutter) {
  @apply dark:bg-[#fa9] bg-[#e43];
}

:deep(.ͼ2.cm-merge-a .cm-changedText) {
  background: linear-gradient(#ee443366, #ee443366) bottom/100% 2px no-repeat;

  @media (prefers-color-scheme: dark) {
    background: linear-gradient(#ffaa9966, #ffaa9966) bottom/100% 2px no-repeat;
  }
}

:deep(.ͼ2.cm-merge-b .cm-changedLineGutter) {
  @apply dark:bg-[#8f8] bg-[#2b2];
}

:deep(.ͼ2.cm-merge-b .cm-changedText) {
  background: linear-gradient(#22bb2266, #22bb2266) bottom/100% 2px no-repeat;

  @media (prefers-color-scheme: dark) {
    background: linear-gradient(#88ff8866, #88ff8866) bottom/100% 2px no-repeat;
  }
}

:deep(.cm-gutters) {
  @apply dark:bg-[#333] dark:text-[#ccc] border-none;
}

:deep(.cm-activeLineGutter) {
  @apply dark:bg-transparent;
}

:deep(.cm-activeLine) {
  @apply dark:bg-transparent;
}

:deep(.cm-cursor) {
  @media (prefers-color-scheme: dark) {
    border-left-color: #ccc;
  }
}

:deep(.ͼ2 .cm-selectionBackground) {
  @media (prefers-color-scheme: dark) {
    background: rgba(204, 204, 204, 0.25) !important;
  }
}

:deep(.revert-control-select) {
  width: 110px;

  &:hover {
    .el-select__placeholder {
      @apply text-[#29A745];
    }
  }
}

:deep(.el-checkbox) {
  @apply border text-[#515A6E] dark:border-[#4C4D4F] dark:text-[#BBC6CE] hover:border-[#29A745] hover:text-[#29A745]
  m-0 px-2 rounded-md cursor-pointer;
}

:deep(.el-checkbox__inner) {
  @apply dark:bg-[#202124] border border-[#DCDFE6] dark:border-[#4C4D4F] hover:border-[#29A745];
}

:deep(.el-checkbox__input.is-checked .el-checkbox__inner) {
  @apply bg-[#29A745] border border-[#29A745];
}

:deep(.el-checkbox__input.is-checked + .el-checkbox__label) {
  @apply text-[#29A745];
}

:deep(.el-select__wrapper) {
  @apply dark:bg-[#252525] shadow-none hover:shadow-none border border-[#DCDFE6] dark:border-[#4C4D4F] hover:border-[#29a745];
}

:deep(.el-select__placeholder) {
  @apply text-[#545C70] dark:text-[#bbc6ce] hover:text-[#29a745];
}

:deep(.el-form-item__label) {
  @apply text-[#545C70] dark:text-[#bbc6ce];
}
</style>
