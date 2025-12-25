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
    <div class="tracker-select-container">
      <MultiSelect
        v-model="selectedIndexers"
        :options="selectableIndexers"
        filter
        optionLabel="name"
        placeholder="Select trackers"
        @update:modelValue="fetchUserStats"
        class="tracker-select"
      >
        <template #option="slotProps">
          <div class="tracker-option">
            <span class="tracker-color-indicator" :style="{ backgroundColor: getTrackerColor(slotProps.option.name) }"></span>
            <span>{{ slotProps.option.name }}</span>
          </div>
        </template>
        <template #value="slotProps">
          <div v-if="slotProps.value && slotProps.value.length" class="tracker-chips">
            <span
              v-for="indexer in slotProps.value"
              :key="indexer.id"
              class="tracker-chip"
            >
              <span class="tracker-color-indicator" :style="{ backgroundColor: getTrackerColor(indexer.name) }"></span>
              <span>{{ indexer.name }}</span>
            </span>
          </div>
          <span v-else>{{ slotProps.placeholder }}</span>
        </template>
      </MultiSelect>
    </div>
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
import { getUserStats, type GetUserStatsQuery, type UserProfileScrapedVec, type MultiIndexerUserStats } from '@/services/api/userStatsService'
import { DatePicker, MultiSelect, Button } from 'primevue'
import { onMounted, ref } from 'vue'
import { startOfMonth, endOfMonth } from 'date-fns'
import { getIndexersEnriched, type IndexerEnriched } from '@/services/api/indexerService'
import { showToast } from '@/main'
import { getTrackerColor } from '@/utils/trackerColors'

const emit = defineEmits<{
  gotResults: [MultiIndexerUserStats, Record<number, IndexerEnriched>]
  selectedValuesUpdated: [(keyof UserProfileScrapedVec)[]]
}>()

const displayableValues = ref<(keyof UserProfileScrapedVec)[]>([])
const selectedValues = ref<(typeof displayableValues.value)[number][]>(['uploaded', 'downloaded'])
const selectableIndexers = ref<IndexerEnriched[]>([])
const selectedIndexers = ref<IndexerEnriched[]>([])
const dateRange = ref<Date[]>([])
const indexerMap = ref<Record<number, IndexerEnriched>>({})

const setPresetRange = () => {
  const today = new Date()
  dateRange.value = [startOfMonth(today), endOfMonth(today)]
}

const fetchUserStats = async () => {
  if (selectedIndexers.value.length > 0 && dateRange.value.length === 2) {
    const endDate = new Date(dateRange.value[1])
    endDate.setHours(23, 59, 59, 999)
    const form: GetUserStatsQuery = {
      date_from: dateRange.value[0].toISOString().slice(0, -1),
      date_to: endDate.toISOString().slice(0, -1),
      indexer_ids: selectedIndexers.value.map((idx) => idx.id),
    }
    getUserStats(form)
      .then((data) => {
        if (!data || Object.keys(data).length === 0) {
          showToast('', 'No data available for selected trackers', 'warn', 3000)
          return
        }
        emit('gotResults', data, indexerMap.value)
        const firstIndexerData = Object.values(data)[0]
        if (firstIndexerData && firstIndexerData.profile) {
          displayableValues.value = (Object.keys(firstIndexerData.profile) as (keyof UserProfileScrapedVec)[]).filter(
            (key) => {
              const arr = firstIndexerData.profile[key]
              if (!arr || !Array.isArray(arr) || arr.length === 0) {
                return false
              }
              const lastIndex = arr.length - 1
              return arr[lastIndex] !== null && arr[lastIndex] !== undefined
            }
          )
          const avatarIndex = displayableValues.value.indexOf('avatar')
          if (avatarIndex !== -1) {
            displayableValues.value.splice(avatarIndex, 1)
          }
          const classIndex = displayableValues.value.indexOf('class')
          if (classIndex !== -1) {
            displayableValues.value.splice(classIndex, 1)
          }
          selectedValues.value = selectedValues.value.filter((val) => displayableValues.value.includes(val))
          emit('selectedValuesUpdated', selectedValues.value)
        }
      })
      .catch((error) => {
        console.error('Error fetching user stats:', error)
      })
  }
}

const setDefaultForm = () => {
  if (selectedIndexers.value.length === 0) {
    showToast('', 'Select at least one tracker first', 'error', 2000)
  } else {
    localStorage.setItem('defaultSelectedValues', JSON.stringify(selectedValues.value))
    localStorage.setItem('defaultSelectedIndexerIds', JSON.stringify(selectedIndexers.value.map((idx) => idx.id)))
    showToast('', 'Trackers and displayed values set as default', 'success', 3000)
  }
}

onMounted(async () => {
  const indexers = await getIndexersEnriched(true)
  if (indexers.length === 0) {
    showToast('', 'No indexer with data available. Set them up in the settings', 'warn', 4000)
    return
  }
  selectableIndexers.value = indexers
  indexerMap.value = indexers.reduce((acc, idx) => {
    acc[idx.id] = idx
    return acc
  }, {} as Record<number, IndexerEnriched>)
  
  // load default form if it exists
  const defaultSelectedValues = localStorage.getItem('defaultSelectedValues')
  if (defaultSelectedValues) {
    selectedValues.value = JSON.parse(defaultSelectedValues)
    emit('selectedValuesUpdated', selectedValues.value)
  }
  const defaultSelectedIndexerIds = localStorage.getItem('defaultSelectedIndexerIds')
  if (defaultSelectedIndexerIds) {
    const ids = JSON.parse(defaultSelectedIndexerIds) as number[]
    selectedIndexers.value = indexers.filter((idx) => ids.includes(idx.id))
  } else {
    selectedIndexers.value = [indexers[0]]
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

.tracker-select-container {
  display: flex;
  flex-direction: column;
}

.tracker-select {
  min-width: 17rem;
}

.tracker-option {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tracker-color-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  display: inline-block;
  flex-shrink: 0;
}

.tracker-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.tracker-chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 2px 8px;
  background-color: var(--p-chip-background, rgba(255, 255, 255, 0.1));
  border-radius: 4px;
  font-size: 0.875rem;
}
</style>
