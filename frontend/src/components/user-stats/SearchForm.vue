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
    <Select
      v-model="selectedIndexer"
      :options="selectableIndexers"
      filter
      optionLabel="name"
      placeholder="Select an indexer"
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
import { Button, DatePicker, Select } from 'primevue'
import { onMounted, ref } from 'vue'
import { startOfWeek, endOfWeek } from 'date-fns'
import { getIndexersLite, type IndexerLite } from '@/services/api/indexerService'
import { showToast } from '@/main'

const emit = defineEmits<{
  gotResults: [UserProfileVec]
}>()

const loading = ref(false)
const selectableIndexers = ref<IndexerLite[]>([])
const selectedIndexer = ref<IndexerLite>()
const dateRange = ref<Date[]>([])
const form = ref<GetUserStatsQuery>({
  date_from: '',
  date_to: '',
  indexer_id: 0,
})

const setPresetRange = () => {
  const today = new Date()
  dateRange.value = [startOfWeek(today), endOfWeek(today)]
}
const fetchUserStats = async () => {
  if (selectedIndexer.value) {
    loading.value = true
    form.value.date_from = dateRange.value[0].toISOString().slice(0, -1)
    form.value.date_to = dateRange.value[1].toISOString().slice(0, -1)
    form.value.indexer_id = selectedIndexer.value.id
    getUserStats(form.value)
      .then((data) => emit('gotResults', data))
      .finally(() => (loading.value = false))
  }
}
onMounted(async () => {
  const indexers = await getIndexersLite(true)
  if (indexers.length === 0) {
    showToast('', 'No indexer with data available. Set them up in the settings', 'warning', 4000)
    return
  }
  selectableIndexers.value = indexers
  selectedIndexer.value = selectableIndexers.value[0]
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
