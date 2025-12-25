<template>
  <div id="indexer-stats-detail">
    <div class="wrapper-center title">Evolution on the selected period</div>
    <div class="charts">
      <ContentContainer v-for="value in selectedValues" :key="value">
        <Chart type="line" :data="chartData(value)" :options="chartOptions(value)" />
      </ContentContainer>
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { MultiIndexerUserStats, UserProfileScrapedVec } from '@/services/api/userStatsService'
import type { IndexerEnriched } from '@/services/api/indexerService'
import Chart from 'primevue/chart'
import ContentContainer from '../ContentContainer.vue'
import 'chartjs-adapter-date-fns'
import { getTrackerColor } from '@/utils/trackerColors'

const props = defineProps<{
  userStats: MultiIndexerUserStats
  indexerMetadata: Record<number, IndexerEnriched>
  selectedValues: (keyof UserProfileScrapedVec)[]
}>()

const chartOptions = (value: keyof UserProfileScrapedVec) => {
  let unit = ''
  switch (value) {
    case 'uploaded':
    case 'downloaded':
    case 'seed_size':
    case 'uploaded_real':
    case 'downloaded_real':
      unit = 'GiB'
      break
  }
  return {
    scales: {
      x: {
        type: 'time',
        time: {
          unit: 'day',
        },
      },
      y: {
        title: {
          display: true,
          text: unit,
        },
      },
    },
  }
}

const chartData = (value: keyof UserProfileScrapedVec) => {
  const datasets = Object.entries(props.userStats).map(([indexerIdStr, userStatsVec]) => {
    const indexerId = parseInt(indexerIdStr)
    const indexer = props.indexerMetadata[indexerId]
    const trackerName = indexer?.name || `Tracker ${indexerId}`
    const color = getTrackerColor(trackerName)

    const rawData = userStatsVec.profile[value]
    let processedData: number[] = []
    switch (value) {
      case 'uploaded':
      case 'downloaded':
        processedData = (rawData as number[]).map((val) => val / 1024 / 1024 / 1024)
        break
      case 'seed_size':
      case 'uploaded_real':
      case 'downloaded_real':
        processedData = (rawData as (number | null)[]).map((val) => ((val ?? 0) as number) / 1024 / 1024 / 1024)
        break
      default:
        processedData = rawData as number[]
    }

    const dataPoints = processedData.map((val, idx) => ({
      x: new Date(userStatsVec.scraped_at[idx]).toISOString(),
      y: val,
    }))

    return {
      borderColor: color,
      label: `${value} - ${trackerName}`,
      data: dataPoints,
      tension: 0.2,
    }
  })

  return {
    datasets: datasets,
  }
}
</script>
<style scoped>
.charts {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  > * {
    margin: 0 5px;
    margin-bottom: 10px;
  }
  .p-chart {
    width: 36em;
    height: auto;
  }
}
</style>
