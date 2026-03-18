<template>
  <div class="line">
    <SelectButton
      v-model="selectedPreset"
      :options="[
        { label: 'Week', value: 7 },
        { label: 'Month', value: 30 },
        { label: 'Year', value: 365 },
        { label: 'All', value: 0 },
        { label: 'Custom', value: -1 },
      ]"
      optionLabel="label"
      optionValue="value"
      :allowEmpty="false"
      size="small"
      @update:modelValue="onPresetChange"
    />
    <DatePicker
      v-if="selectedPreset === -1"
      v-model="dateRange"
      dateFormat="d M yy"
      selectionMode="range"
      :manualInput="false"
      showIcon
      fluid
      :showOnFocus="false"
      @update:modelValue="fetchUserStats"
    />
    <MultiSelect
      v-model="selectedIndexers"
      :options="selectableIndexers"
      filter
      optionLabel="name"
      placeholder="Select indexers"
      display="chip"
      @update:modelValue="fetchUserStats"
    />
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
import {
  getUserStats,
  getIndexersEnriched,
  type GetUserStatsRequest,
  type UserProfileScrapedVec,
  type IndexerStats,
  type IndexerEnriched,
} from '@/services/api-schema'
import { DatePicker, MultiSelect, Button, SelectButton } from 'primevue'
import { onMounted, ref } from 'vue'
import { subDays } from 'date-fns'
import { showToast } from '@/main'

const emit = defineEmits<{
  gotResults: [IndexerStats[]]
  selectedValuesUpdated: [(keyof UserProfileScrapedVec)[]]
  selectedIndexersUpdated: [Map<number, string>]
}>()

const displayableValues = ref<(keyof UserProfileScrapedVec)[]>([])
const selectedValues = ref<(typeof displayableValues.value)[number][]>(['uploaded', 'downloaded'])
const loading = ref(false)
const selectableIndexers = ref<IndexerEnriched[]>([])
const selectedIndexers = ref<IndexerEnriched[]>([])
const dateRange = ref<Date[]>([])
const selectedPreset = ref(30)
const form = ref<GetUserStatsRequest>({
  date_from: '',
  date_to: '',
  indexer_ids: '',
})

const onPresetChange = (days: number) => {
  if (days === -1) return
  const today = new Date()
  if (days === 0) {
    dateRange.value = [new Date(0), today]
  } else {
    dateRange.value = [subDays(today, days), today]
  }
  fetchUserStats()
}
const fetchUserStats = async () => {
  if (selectedIndexers.value.length > 0) {
    loading.value = true
    form.value.date_from = dateRange.value[0].toISOString().slice(0, -1)
    form.value.date_to = new Date(dateRange.value[1].setHours(23, 59, 59, 999)).toISOString().slice(0, -1)
    form.value.indexer_ids = selectedIndexers.value.map((i) => i.id).join(',')
    const indexerNames = new Map(selectedIndexers.value.map((i) => [i.id, i.name]))
    emit('selectedIndexersUpdated', indexerNames)
    getUserStats(form.value)
      .then((data) => {
        emit('gotResults', data)
        if (data.length > 0) {
          const profile = data[0].profile
          displayableValues.value = (Object.keys(profile) as (keyof UserProfileScrapedVec)[]).filter(
            // @ts-expect-error TODO: fix error .at() doesn't exist
            (key) => profile[key] && profile[key].length > 0 && profile[key].at(-1) !== null,
          )
          displayableValues.value.splice(displayableValues.value.indexOf('avatar'), 1)
          displayableValues.value.splice(displayableValues.value.indexOf('class'), 1)
          selectedValues.value = selectedValues.value.filter((val) => displayableValues.value.includes(val))
        }
        emit('selectedValuesUpdated', selectedValues.value)
      })
      .finally(() => (loading.value = false))
  }
}
const setDefaultForm = () => {
  if (selectedIndexers.value.length === 0) {
    showToast('', 'Select an indexer first', 'error', 2000)
  } else {
    localStorage.setItem('defaultSelectedValues', JSON.stringify(selectedValues.value))
    localStorage.setItem('defaultSelectedIndexerIds', JSON.stringify(selectedIndexers.value.map((i) => i.id)))
    showToast('', 'Indexers and displayed values set as default', 'success', 3000)
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
  const defaultSelectedIndexerIds = localStorage.getItem('defaultSelectedIndexerIds')
  if (defaultSelectedIndexerIds) {
    const ids: number[] = JSON.parse(defaultSelectedIndexerIds)
    selectedIndexers.value = selectableIndexers.value.filter((indexer) => ids.includes(indexer.id))
  } else {
    selectedIndexers.value = [selectableIndexers.value[0]]
  }
  onPresetChange(selectedPreset.value)
})
</script>
<style scoped>
.line {
  display: flex;
  justify-content: center;
  align-items: center;
  > * {
    margin: 5px;
  }
}
</style>
