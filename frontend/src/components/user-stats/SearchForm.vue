<template>
  <div class="line">
    <DatePicker
      v-model="dateRange"
      selectionMode="range"
      :manualInput="false"
      showIcon
      fluid
      :showOnFocus="false"
    />
    <Button label="Get Stats" size="small" :loading @click="fetchUserStats" />
  </div>
</template>
<script lang="ts" setup>
import {
  getUserStats,
  type GetUserStatsQuery,
  type UserProfileVec,
} from '@/services/api/userStatsService'
import { Button, DatePicker } from 'primevue'
import { onMounted, ref } from 'vue'
import { startOfWeek, endOfWeek } from 'date-fns'

const emit = defineEmits<{
  gotResults: [UserProfileVec]
}>()

const loading = ref(false)
const dateRange = ref<Date[]>([])
const form = ref<GetUserStatsQuery>({
  date_from: '',
  date_to: '',
  indexer_id: 1,
})

const setPresetRange = () => {
  const today = new Date()
  dateRange.value = [startOfWeek(today), endOfWeek(today)]
}
const fetchUserStats = async () => {
  loading.value = true
  form.value.date_from = dateRange.value[0].toISOString().slice(0, -1)
  form.value.date_to = dateRange.value[1].toISOString().slice(0, -1)
  getUserStats(form.value)
    .then((data) => emit('gotResults', data))
    .finally(() => (loading.value = false))
}
onMounted(async () => {
  setPresetRange()
  await fetchUserStats()
})
</script>
<style scoped>
.line {
  display: flex;
  justify-content: center;
  > * {
    margin: 5px;
  }
}
</style>
