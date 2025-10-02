<script lang="ts" setup>
import Card from '../components/Card.vue'
import { ref } from 'vue'
import dayjs from 'dayjs'
import { ElMessage } from 'element-plus'

const padZero = (num: number) => {
  return num.toString().padStart(2, '0')
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

const initDate = new Date()
const date1_1 = ref(getStandardTimeSeconds(initDate))
const date1_2 = ref(getStandardTimeSeconds(new Date(initDate.getTime() + 24 * 60 * 60 * 1000)))
const result1 = ref<number>()
const unit1 = ref<string>('second')
const date2 = ref<string>(getStandardTimeSeconds(initDate))
const sign = ref<string>('+')
const diffValue = ref<number>(1)
const unit2 = ref<string>('day')
const result2 = ref<string>()
const date3 = ref(getStandardTimeSeconds(initDate))
const unit3 = ref<string>('year')
const result3 = ref<string>()

const diffBetween = () => {
  const startDate = dayjs(date1_1.value)
  const endDate = dayjs(date1_2.value)
  result1.value = endDate.diff(startDate, unit1.value as any)
}

diffBetween()

const calculate = () => {
  const start = dayjs(date2.value)
  let end: dayjs.Dayjs
  if (sign.value === '+') {
    end = start.add(diffValue.value, unit2.value as any)
  } else {
    end = start.subtract(diffValue.value, unit2.value as any)
  }
  result2.value = end.format('YYYY-MM-DD HH:mm:ss')
}

calculate()

const copy = (value: string | undefined) => {
  if (!value) return
  navigator.clipboard.writeText(value)
  ElMessage.success({ message: '复制成功', grouping: true, customClass: 'success' })
}

const getQuarter = () => {
  const month = dayjs(date3.value).month()
  let quarterInfo: { quarter: number; quarterStartMonth: number }
  if (month <= 2) {
    // 第一季度: 0,1,2月
    quarterInfo = { quarter: 1, quarterStartMonth: 0 }
  } else if (month <= 5) {
    // 第二季度: 3,4,5月
    quarterInfo = { quarter: 2, quarterStartMonth: 3 }
  } else if (month <= 8) {
    // 第三季度: 6,7,8月
    quarterInfo = { quarter: 3, quarterStartMonth: 6 }
  } else {
    // 第四季度: 9,10,11月
    quarterInfo = { quarter: 4, quarterStartMonth: 9 }
  }
  return quarterInfo
}

const analyzeDiffBetweenTowDate = (start: dayjs.Dayjs, target: dayjs.Dayjs) => {
  const totalSeconds = target.diff(start, 'second')
  const totalMinutes = Math.ceil(totalSeconds / 60)
  const totalHours = Math.ceil(totalMinutes / 60)
  const totalDays = Math.ceil(totalHours / 24)
  const totalWeeks = Math.ceil(totalDays / 7)
  return {
    totalSeconds,
    totalMinutes,
    totalHours,
    totalDays,
    totalWeeks
  }
}

const analyzeForYear = () => {
  const target = dayjs(date3.value)
  const year = target.year()
  const start = dayjs(new Date(year, 0, 1))
  const { quarter } = getQuarter()
  const { totalSeconds, totalMinutes, totalHours, totalDays, totalWeeks } =
    analyzeDiffBetweenTowDate(start, target)
  result3.value = `${year}年: ${quarter}季度, ${totalWeeks}周, ${totalDays}天, ${totalHours}小时, ${totalMinutes}分钟, ${totalSeconds}秒`
}

const analyzeForQuarter = () => {
  const target = dayjs(date3.value)
  const { quarter, quarterStartMonth } = getQuarter()
  const start = dayjs(new Date(target.year(), quarterStartMonth, 1))
  const { totalSeconds, totalMinutes, totalHours, totalDays, totalWeeks } =
    analyzeDiffBetweenTowDate(start, target)
  result3.value = `${quarter}季度: ${totalWeeks}周, ${totalDays}天, ${totalHours}小时, ${totalMinutes}分钟, ${totalSeconds}秒`
}

const analyzeForMonth = () => {
  const target = dayjs(date3.value)
  const start = dayjs(new Date(target.year(), target.month(), 1))
  const { totalSeconds, totalMinutes, totalHours, totalDays, totalWeeks } =
    analyzeDiffBetweenTowDate(start, target)
  result3.value = `${target.month() + 1}月: ${totalWeeks}周, ${totalDays}天, ${totalHours}小时, ${totalMinutes}分钟, ${totalSeconds}秒`
}

const analyzeDate = () => {
  switch (unit3.value) {
    case 'year':
      analyzeForYear()
      break
    case 'quarter':
      analyzeForQuarter()
      break
    case 'month':
      analyzeForMonth()
      break
  }
}
analyzeDate()
</script>

<template>
  <div class="h-full w-full p-2 flex flex-col justify-start gap-2 overflow-auto">
    <Card title="差值计算器" class="dark:text-[#AEB9C0]">
      <div class="flex items-center gap-2 flex-wrap text-[#515A6E] dark:text-[#AEB9C0]">
        <el-input v-model="date1_1" style="width: 200px" @input="diffBetween" />
        与
        <el-input v-model="date1_2" style="width: 200px" @input="diffBetween" />
        相差
        <div class="input-with-select">
          <el-input
            v-model="result1"
            readonly
            style="width: 140px"
            class="result-input"
            @click="copy(result1?.toString())"
          />
          <el-select
            v-model="unit1"
            style="width: 80px"
            popper-class="custom-select"
            @change="diffBetween"
          >
            <el-option label="年" value="year" />
            <el-option label="月" value="month" />
            <el-option label="周" value="week" />
            <el-option label="天" value="day" />
            <el-option label="小时" value="hour" />
            <el-option label="分钟" value="minute" />
            <el-option label="秒" value="second" />
          </el-select>
        </div>
      </div>
    </Card>
    <Card title="时间操作" class="dark:text-[#AEB9C0]">
      <div class="flex items-center gap-2 flex-wrap">
        <el-input v-model="date2" style="width: 200px" @input="calculate" />
        <el-select
          v-model="sign"
          style="width: 60px"
          popper-class="custom-select"
          @change="calculate"
        >
          <el-option label="加" value="+" />
          <el-option label="减" value="-" />
        </el-select>
        <div class="input-with-select">
          <el-input
            v-model="diffValue"
            :min="0"
            type="number"
            style="width: 140px"
            @change="calculate"
          />
          <el-select
            v-model="unit2"
            style="width: 80px"
            popper-class="custom-select"
            @change="calculate"
          >
            <el-option label="年" value="year" />
            <el-option label="月" value="month" />
            <el-option label="周" value="week" />
            <el-option label="天" value="day" />
            <el-option label="小时" value="hour" />
            <el-option label="分钟" value="minute" />
            <el-option label="秒" value="second" />
          </el-select>
        </div>
        后，为
        <el-input
          v-model="result2"
          readonly
          style="width: 170px"
          class="result-input"
          @click="copy(result2?.toString())"
        />
      </div>
    </Card>
    <Card title="时间分析" class="dark:text-[#AEB9C0]">
      <div class="flex items-center gap-2 flex-wrap">
        <el-input v-model="date3" style="width: 200px" @input="analyzeDate" />
        <el-select
          v-model="unit3"
          style="width: 80px"
          popper-class="custom-select"
          @change="analyzeDate"
        >
          <el-option label="年" value="year" />
          <el-option label="季度" value="quarter" />
          <el-option label="月" value="month" />
        </el-select>
        <el-input
          v-model="result3"
          readonly
          style="width: 440px"
          class="result-input"
          @click="copy(result3?.toString())"
        />
      </div>
    </Card>
  </div>
</template>

<style lang="scss" scoped>
:deep(.el-select__wrapper) {
  @apply text-[#515A6E] dark:bg-[#252525] shadow-none hover:shadow-none border border-[#DCDFE6] dark:border-[#4C4D4F];
}

:deep(.el-select__placeholder) {
  @apply text-[#515A6E] dark:text-[#bbc6ce] hover:text-[#29a745];
}

:deep(.el-form-item__label) {
  @apply text-[#515A6E] dark:text-[#bbc6ce];
}

:deep(.el-input__inner) {
  @apply text-center;
}

.input-with-select {
  :deep(.el-input__wrapper) {
    @apply rounded-tr-none rounded-br-none h-[32px] cursor-default;
  }

  :deep(.el-input__inner) {
    @apply cursor-default;
  }

  :deep(.el-select__wrapper) {
    @apply border-l-0 rounded-tl-none rounded-bl-none;
  }
}

:deep(.el-input) {
  &.result-input {
    .el-input__wrapper {
      @apply dark:bg-[#333333] cursor-pointer border-[#DCDFE6] dark:border-[#4C4D4F] hover:border-[#29a745] border-dashed;

      &:hover {
        .el-input__inner {
          @apply text-[#29a745];
        }
      }
    }

    .el-input__inner {
      @apply cursor-pointer text-[#545C70] dark:text-[#bbc6ce] hover:text-[#29a745];
    }
  }
}
</style>
