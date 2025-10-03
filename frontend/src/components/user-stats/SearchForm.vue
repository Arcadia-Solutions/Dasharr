<template>
  <div class="line">
    <DatePicker
      v-model="dateRange"
      dateFormat="d M yy"
      selectionMode="range"
      :manualInput="false"
      showIcon
      fluid
      :showOnFocus="false"
      @update:modelValue="fetchUserStats"
    />
    <Select v-model="selectedIndexer" :options="selectableIndexers" filter optionLabel="name" placeholder="Select an indexer" @change="fetchUserStats" />
  </div>
  <div class="wrapper-center">
    <MultiSelect
      v-model="selectedValues"
      :options="displayableValues"
      @update:modelValue="(val) => emit('selectedValuesUpdated', val)"
      filter
      display="chip"
      placeholder="Charts to display"
    />
  </div>
  <div class="wrapper-center" style="margin-top: 5px">
    <Button label="Set as default" size="small" @click="setDefaultForm" />
  </div>
</template>
<script lang="ts" setup>
import { getUserStats, type GetUserStatsQuery, type UserProfileScrapedVec, type UserProfileVec } from '@/services/api/userStatsService'
import { DatePicker, Select, MultiSelect, Button } from 'primevue'
import { onMounted, ref } from 'vue'
import { startOfMonth, endOfMonth } from 'date-fns'
import { getIndexersEnriched, type IndexerEnriched } from '@/services/api/indexerService'
import { showToast } from '@/main'

const emit = defineEmits<{
  gotResults: [UserProfileVec]
  selectedValuesUpdated: [(keyof UserProfileScrapedVec)[]]
}>()

const displayableValues = ref<(keyof UserProfileScrapedVec)[]>([
  'downloaded',
  'uploaded',
  'bonus_points',
  'leeching',
  'seeding',
  'ratio',
  'uploaded_torrents',
  'snatched',
  'invited',
  'torrent_comments',
])
const selectedValues = ref<(typeof displayableValues.value)[number][]>(['uploaded', 'downloaded'])
const loading = ref(false)
const selectableIndexers = ref<IndexerEnriched[]>([])
const selectedIndexer = ref<IndexerEnriched>()
const dateRange = ref<Date[]>([])
const form = ref<GetUserStatsQuery>({
  date_from: '',
  date_to: '',
  indexer_id: 0,
})

const setPresetRange = () => {
  const today = new Date()
  dateRange.value = [startOfMonth(today), endOfMonth(today)]
}
const fetchUserStats = async () => {
  if (selectedIndexer.value) {
    loading.value = true
    form.value.date_from = dateRange.value[0].toISOString().slice(0, -1)
    form.value.date_to = new Date(dateRange.value[1].setHours(23, 59, 59, 999)).toISOString().slice(0, -1)
    form.value.indexer_id = selectedIndexer.value.id
    getUserStats(form.value)
      .then((data) => emit('gotResults', data))
      .finally(() => (loading.value = false))
  }
}
const setDefaultForm = () => {
  if (!selectedIndexer.value) {
    showToast('', 'Select an indexer first', 'error', 2000)
  } else {
    localStorage.setItem('defaultSelectedValues', JSON.stringify(selectedValues.value))
    localStorage.setItem('defaultSelectedIndexerId', selectedIndexer.value.id.toString())
    showToast('', 'Indexer and displayed values set as default', 'success', 3000)
  }
}
onMounted(async () => {
  const indexers = await getIndexersEnriched(true)
  if (indexers.length === 0) {
    showToast('', 'No indexer with data available. Set them up in the settings', 'warn', 4000)
    return
  }
  selectableIndexers.value = indexers
  // load default form if it exists
  const defaultSelectedValues = localStorage.getItem('defaultSelectedValues')
  if (defaultSelectedValues) {
    selectedValues.value = JSON.parse(defaultSelectedValues)
    emit('selectedValuesUpdated', selectedValues.value)
  }
  const defaultSelectedIndexerId = localStorage.getItem('defaultSelectedIndexerId')
  if (defaultSelectedIndexerId) {
    selectedIndexer.value = selectableIndexers.value.find((indexer) => indexer.id === parseInt(defaultSelectedIndexerId))
  } else {
    selectedIndexer.value = selectableIndexers.value[0]
  }
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
