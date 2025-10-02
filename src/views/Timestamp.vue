<script lang="ts" setup>
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import {eventBus} from "../utils/eventBus.ts";

const padZero = (num: number) => {
  return num.toString().padStart(2, '0')
}

const padMilliseconds = (ms: number) => {
  return ms.toString().padStart(3, '0')
}

const getStandardTimeSeconds = (now: Date) => {
  const year = now.getFullYear()
  const month = padZero(now.getMonth() + 1) // 月份从0开始，所以要+1
  const day = padZero(now.getDate())
  const hours = padZero(now.getHours())
  const minutes = padZero(now.getMinutes())
  const seconds = padZero(now.getSeconds())

  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`
}

const getUnixTimestampSeconds = (now: Date) => {
  return Math.floor(now.getTime() / 1000)
}

const getStandardTimeMilliseconds = (now: Date) => {
  const milliseconds = padMilliseconds(now.getMilliseconds())
  return `${getStandardTimeSeconds(now)}.${milliseconds}`
}

const getUnixTimestampMilliseconds = (now: Date) => {
  return now.getTime()
}

const getUnixTimestampNanoseconds = (now: Date) => {
  let nanoPart = ''
  if (inputValue.value.toString().includes('.')) {
    nanoPart = inputValue.value
      .toString()
      .substring(inputValue.value.toString().lastIndexOf('.') + 4)
  }
  return now.getTime().toString() + nanoPart.padEnd(6, '0')
}

const getStandardTimeNanoseconds = (nanosecondsTimestamp: string) => {
  // 1. 使用 BigInt 来处理巨大的纳秒时间戳
  const nsBigInt = BigInt(nanosecondsTimestamp)

  // 2. 定义纳秒到秒的转换因子
  const nanosecondsPerSecond = BigInt(1000000000) // 1秒 = 1,000,000,000 纳秒

  // 3. 计算总秒数和剩余的纳秒数
  const secondsBigInt = nsBigInt / nanosecondsPerSecond
  const remainingNanoseconds = nsBigInt % nanosecondsPerSecond

  // 4. 将秒数转换为毫秒数（JavaScript Date 使用毫秒）
  // 注意：对于非常大的秒数，将其转换为 Number 类型可能会存在精度损失，但这是使用 Date 对象所必需的步骤。
  const millisecondsNumber = Number(secondsBigInt) * 1000

  // 5. 创建 Date 对象
  const date = new Date(millisecondsNumber)

  // 6. 检查 Date 对象是否有效
  if (isNaN(date.getTime())) {
    throw new Error(
      'Invalid date resulting from the timestamp. It might be out of the representable range.'
    )
  }

  // 7. 格式化 Date 对象中的日期和时间部分
  // 获取年份（可能是很大的数字）
  const year = date.getUTCFullYear()
  // 获取月份（0-11，所以+1），并补零
  const month = String(date.getUTCMonth() + 1).padStart(2, '0')
  // 获取日期，并补零
  const day = String(date.getUTCDate()).padStart(2, '0')
  // 获取小时，并补零
  const hours = String(date.getUTCHours()).padStart(2, '0')
  // 获取分钟，并补零
  const minutes = String(date.getUTCMinutes()).padStart(2, '0')
  // 获取秒，并补零
  const seconds = String(date.getUTCSeconds()).padStart(2, '0')

  // 8. 格式化剩余的纳秒部分，确保其为9位数（根据需要补前导零）
  const nanosecondsStr = String(remainingNanoseconds).padStart(9, '0')
  // 如果纳秒部分超过9位，理论上应该截断，但根据你的输入，取模后不会超过9位
  // 注意：remainingNanoseconds 是 BigInt，转换为字符串后补零

  // 9. 组合成最终的字符串格式
  // 使用 UTC 时间组件以避免时区干扰，因为输入时间戳通常被认为是 UTC

  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}.${nanosecondsStr}`
}

const tableData = ref<{ type: string; value: string | number }[]>([
  {
    type: '标准时间(秒)',
    value: getStandardTimeSeconds(new Date())
  },
  {
    type: 'Unix时间戳(秒)',
    value: getUnixTimestampSeconds(new Date())
  },
  {
    type: '标准时间(毫秒)',
    value: getStandardTimeMilliseconds(new Date())
  },
  {
    type: 'Unix时间戳(毫秒)',
    value: getUnixTimestampMilliseconds(new Date())
  }
])

const updateAllTimestamps = () => {
  tableData.value = tableData.value.map((row) => {
    const now = new Date() // 获取当前时间
    let newValue: string | number = ''
    // 根据不同的类型设置新的值
    switch (row.type) {
      case '标准时间(秒)':
        newValue = getStandardTimeSeconds(now)
        break
      case 'Unix时间戳(秒)':
        newValue = getUnixTimestampSeconds(now)
        break
      case '标准时间(毫秒)':
        newValue = getStandardTimeMilliseconds(now)
        break
      case 'Unix时间戳(毫秒)': // 毫秒级时间戳和标准时间毫秒显示都用 getTime()
        newValue = getUnixTimestampMilliseconds(now)
        break
      default:
        newValue = row.value // 默认保持不变（虽然应该不会执行到）
    }
    // 返回新的对象，确保响应式更新。如果只修改原对象的属性，可能更新不生效。
    return {
      ...row,
      value: newValue
    }
  })
}

// 声明一个变量来存储定时器 ID，用于清除
let intervalId: any = null

// 在组件挂载后启动定时器
onMounted(() => {
  // 每隔 1000 毫秒（1秒）更新一次时间
  intervalId = setInterval(updateAllTimestamps, 100)
})

// 在组件卸载前清除定时器，防止内存泄漏
onUnmounted(() => {
  if (intervalId !== null) {
    clearInterval(intervalId)
  }
})

const inputValue = ref('')
const result_s = ref('')
const result_ms = ref('')
const result_ns = ref('')

const copy = (value: string) => {
  navigator.clipboard.writeText(value)
  ElMessage.success({ message: '复制成功', grouping: true, customClass: 'success' })
}

const load = (value: any) => {
  inputValue.value = value
}

const handleInput = () => {
  if (inputValue.value.toString().trim() === '') {
    result_s.value = ''
    result_ms.value = ''
    result_ns.value = ''
    return
  }

  if (inputValue.value.toString().match(/^\d{1,22}$/)) {
    const date = new Date(Number(inputValue.value.toString().substring(0, 16)))
    result_s.value = getStandardTimeSeconds(date)
    result_ms.value = getStandardTimeMilliseconds(date)
    result_ns.value = getStandardTimeNanoseconds(inputValue.value.toString())
  } else if (
    inputValue.value
      .toString()
      .match(
        /^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01]) (?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(\.\d{0,9})?$/
      )
  ) {
    let date: Date | null = null
    if (inputValue.value.toString().trim().endsWith('.')) {
      const index = inputValue.value.toString().indexOf('.')
      date = new Date(inputValue.value.toString().substring(0, index - 1))
    } else {
      date = new Date(inputValue.value)
    }
    result_s.value = getUnixTimestampSeconds(date).toString()
    result_ms.value = getUnixTimestampMilliseconds(date).toString()
    result_ns.value = getUnixTimestampNanoseconds(date).toString()
  } else {
    result_s.value = '错误: 输入时间格式异常'
    result_ms.value = '错误: 输入时间格式异常'
    result_ns.value = '错误: 输入时间格式异常'
  }
}

watch(
  () => inputValue.value,
  () => {
    handleInput()
  },
  { immediate: true, deep: true }
)

const clear = () => {
  inputValue.value = ''
}

onMounted(() => {
  eventBus.on('clear', clear)
})

onUnmounted(() => {
  eventBus.off('clear', clear)
})
</script>

<template>
  <div class="h-full w-full p-2 flex flex-col gap-2">
    <el-input
      v-model="inputValue"
      placeholder="支持 YYYY-MM-DD HH:mm:ss[.\d+] 与 时间戳(秒/毫秒/纳秒)输入"
      @input="handleInput"
    >
      <template #prepend>输入</template>
      <template #suffix>
        <el-button type="primary" size="small" @click="clear">清除</el-button>
      </template>
    </el-input>
    <el-input v-model="result_s" readonly>
      <template #prepend>秒</template>
      <template #suffix>
        <el-button type="primary" size="small" @click="copy(result_s)">复制</el-button>
      </template>
    </el-input>
    <div class="flex space-x-2">
      <el-input v-model="result_ms" readonly>
        <template #prepend>毫秒</template>
        <template #suffix>
          <el-button type="primary" size="small" @click="result_ms">复制</el-button>
        </template>
      </el-input>
      <el-input v-model="result_ns" readonly>
        <template #prepend>纳秒</template>
        <template #suffix>
          <el-button type="primary" size="small" @click="result_ns">复制</el-button>
        </template>
      </el-input>
    </div>
    <el-table :data="tableData" stripe style="user-select: none">
      <el-table-column prop="type" label="格式" />
      <el-table-column prop="value" label="值" class-name="table-value">
        <template #default="scope">
          <span @click="copy(scope.row.value)">
            {{ scope.row.value }}
          </span>
        </template>
      </el-table-column>
      <el-table-column label="操作" align="center" width="100px">
        <template #default="scope">
          <el-button type="primary" size="small" @click="load(scope.row.value)">加载</el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<style lang="scss" scoped>
// 表头
:deep(.el-table__header-wrapper th) {
  background-color: #f7f7f7;
  color: #535a6c;
  font-weight: bold;
  border: none !important;

  @media (prefers-color-scheme: dark) {
    background-color: #333;
    color: #bdc6cd;
  }
}

// 斑马条纹样式（需配合 stripe 属性）
:deep(.el-table__row--striped td) {
  background-color: #f6f8f9 !important;

  @media (prefers-color-scheme: dark) {
    background-color: #333 !important;
  }
}

// 表体
:deep(.el-table__body tr > td) {
  background-color: #ffffff;
  color: #535a6c;

  @media (prefers-color-scheme: dark) {
    background-color: #282929;
    color: #bdc6cd;
  }
}

// 行 hover 效果
:deep(.el-table__body tr:hover > td) {
  background-color: #eaf5f1 !important;

  @media (prefers-color-scheme: dark) {
    background-color: #2a3531 !important;
  }
}

:deep(.el-table) {
  td {
    border: none;
  }
}

:deep(.el-table__inner-wrapper::before) {
  height: 0 !important;
}

:deep(.el-table__body .table-value) {
  @media (prefers-color-scheme: dark) {
    &:hover {
      color: #29a745 !important;
      text-decoration: underline;
      cursor: pointer;
    }
  }

  &:hover {
    color: #29a745 !important;
    text-decoration: underline;
    cursor: pointer;
  }
}

:deep(.el-button--primary) {
  --el-button-bg-color: #ffffff;
  --el-button-text-color: #333;
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

:deep(.el-input) {
  --el-input-placeholder-color: #898b8f;
}
</style>
