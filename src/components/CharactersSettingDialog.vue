<script setup lang="ts">
import {Close} from '@icon-park/vue-next'
import {onMounted, ref} from 'vue'

defineProps({
  showDialog: {
    type: Boolean,
    default: false
  }
})

const inputValue = ref('')
const checkList = ref<string[]>([])
let oldCheckList = [] as string[]
const isCancel = ref(true)
const emit = defineEmits(['close-dialog', 'submit'])
const closeDialog = (): void => {
  emit('close-dialog')
  if (isCancel.value) checkList.value = oldCheckList
}
const reset = (): void => {
  checkList.value = ['number', 'slow', 'up']
  checkChange()
}

const submit = (): void => {
  isCancel.value = false
  emit('submit', inputValue.value, checkList.value)
}

const checkChange = (): void => {
  inputValue.value = ''
  if (checkList.value.includes('number')) {
    inputValue.value = inputValue.value.concat('0123456789')
  }
  if (checkList.value.includes('slow')) {
    inputValue.value = inputValue.value.concat('abcdefghijklmnopqrstuvwxyz')
  }
  if (checkList.value.includes('up')) {
    inputValue.value = inputValue.value.concat('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
  }
  if (checkList.value.includes('special')) {
    inputValue.value = inputValue.value.concat("`~!@#$%^&*()-_=+[{]}|;:',<.>/?")
  }
}

onMounted(() => {
  isCancel.value = true
  checkChange()
})

const setPreset = (preset: string[]): string => {
  checkList.value = preset
  oldCheckList = preset
  checkChange()
  return inputValue.value
}

defineExpose({
  setPreset
})
</script>

<template>
  <el-dialog :model-value="showDialog" :show-close="false" class="my-dialog" @close="closeDialog">
    <template #default>
      <div class="h-[50vh] flex flex-col">
        <div
            class="h-[50px] flex items-center border-b border-[#EDF0F3] dark:border-none justify-between p-4 text-lg bg-[#F7F7F7] text-[#676F80] dark:bg-[#333] dark:text-[#BBC6CE] rounded-tl-md rounded-tr-md"
            style="user-select: none"
        >
          设置字符
          <close
              theme="outline"
              size="12"
              :stroke-width="10"
              class="cursor-pointer dark:text-[#555B62] dark:hover:text-[#73828C]"
              @click="closeDialog"
          />
        </div>
        <div class="flex-1 h-full p-2 flex flex-col gap-2 bg-[#FFFFFF] dark:bg-[#252525]">
          <el-input
              v-model="inputValue"
              type="textarea"
              resize="none"
              class="h-full"
              placeholder="请选择预设或输入字符..."
          />
          <div class="w-full">
            <el-checkbox-group
                v-model="checkList"
                class="w-full flex items-center justify-center gap-2 flex-wrap"
                @change="checkChange"
            >
              <el-checkbox class="check-box-with-border" label="数字" size="small" value="number"/>
              <el-checkbox class="check-box-with-border" label="小写字母" size="small" value="slow"/>
              <el-checkbox class="check-box-with-border" label="大写字母" size="small" value="up"/>
              <el-checkbox class="check-box-with-border" label="特殊符号" size="small" value="special"/>
              <el-button class="button-no-bg" size="small" @click="reset">重置</el-button>
            </el-checkbox-group>
          </div>
        </div>
        <div
            class="h-[50px] flex items-center border-t border-[#EDF0F3] dark:border-none p-2 bg-[#F7F7F7] dark:bg-[#333] rounded-bl-md rounded-br-md"
        >
          <el-button class="button-with-bg w-full" @click="submit"> 确认</el-button>
        </div>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">
:deep(.el-textarea__inner) {
  height: 100%;
  box-shadow: none !important;
  @apply dark:bg-[#212123] dark:text-[#BDC6CD];
  outline: none;
  border: 1px solid #dcdfe6;

  @media (prefers-color-scheme: dark) {
    border: #4c4d4f solid 1px;
  }

  &:focus {
    box-shadow: none !important;
    outline: none;
    border: 1px solid #29a745;
  }
}
</style>
